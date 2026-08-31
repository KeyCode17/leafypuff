use std::time::Instant;

use axum::http::HeaderMap;
use leafypuff_api::http::rate_limit::{
    MAX_REQUESTS_PER_WINDOW, RateLimiter, UNKNOWN_CLIENT, Verdict, WINDOW, client_key,
};

fn headers(pairs: &[(&'static str, &str)]) -> HeaderMap {
    let mut headers = HeaderMap::new();
    for (name, value) in pairs {
        headers.insert(*name, value.parse().expect("a valid header value"));
    }
    headers
}

#[test]
fn a_client_is_allowed_up_to_the_ceiling_and_then_refused() {
    let limiter = RateLimiter::new();
    let now = Instant::now();

    for _ in 0..MAX_REQUESTS_PER_WINDOW {
        assert_eq!(limiter.verdict("203.0.113.7", now), Verdict::Allowed);
    }

    assert_eq!(limiter.verdict("203.0.113.7", now), Verdict::Refused);
}

#[test]
fn the_window_reopens_once_it_has_elapsed() {
    let limiter = RateLimiter::new();
    let now = Instant::now();
    for _ in 0..MAX_REQUESTS_PER_WINDOW {
        limiter.verdict("203.0.113.7", now);
    }

    assert_eq!(
        limiter.verdict("203.0.113.7", now + WINDOW),
        Verdict::Allowed
    );
}

#[test]
fn two_clients_do_not_share_a_budget() {
    let limiter = RateLimiter::new();
    let now = Instant::now();
    for _ in 0..MAX_REQUESTS_PER_WINDOW {
        limiter.verdict("203.0.113.7", now);
    }

    assert_eq!(limiter.verdict("203.0.113.8", now), Verdict::Allowed);
}

#[test]
fn the_connecting_ip_wins_over_the_forwarded_chain() {
    let headers = headers(&[
        ("x-forwarded-for", "1.2.3.4, 172.71.0.9"),
        ("cf-connecting-ip", "203.0.113.7"),
    ]);

    assert_eq!(client_key(&headers), "203.0.113.7");
}

#[test]
fn a_direct_request_falls_back_to_the_last_forwarded_hop() {
    let headers = headers(&[("x-forwarded-for", "1.2.3.4, 203.0.113.7")]);

    assert_eq!(client_key(&headers), "203.0.113.7");
}

#[test]
fn a_request_with_no_client_header_falls_into_one_shared_bucket() {
    assert_eq!(client_key(&HeaderMap::new()), UNKNOWN_CLIENT);
}
