use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use super::error::ApiError;

pub const MAX_REQUESTS_PER_WINDOW: u32 = 10;
pub const WINDOW: Duration = Duration::from_secs(60);
pub const UNKNOWN_CLIENT: &str = "unknown";

const ERR_TOO_MANY_REQUESTS: &str = "Too many requests, try again shortly";
const ERR_LIMITER_UNAVAILABLE: &str = "The rate limiter is unavailable";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Allowed,
    Refused,
    Unavailable,
}

#[derive(Clone, Default)]
pub struct RateLimiter {
    windows: Arc<Mutex<HashMap<String, (Instant, u32)>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn verdict(&self, key: &str, now: Instant) -> Verdict {
        let Ok(mut windows) = self.windows.lock() else {
            return Verdict::Unavailable;
        };
        windows.retain(|_, (opened, _)| now.duration_since(*opened) < WINDOW);

        let entry = windows.entry(key.to_owned()).or_insert((now, 0));
        if now.duration_since(entry.0) >= WINDOW {
            *entry = (now, 0);
        }
        if entry.1 >= MAX_REQUESTS_PER_WINDOW {
            return Verdict::Refused;
        }
        entry.1 += 1;
        Verdict::Allowed
    }
}

pub fn client_key(headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.rsplit(',').next())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| UNKNOWN_CLIENT.to_owned())
}

pub async fn guard(limiter: RateLimiter, request: Request, next: Next) -> Response {
    let key = client_key(request.headers());
    match limiter.verdict(&key, Instant::now()) {
        Verdict::Allowed => next.run(request).await,
        Verdict::Refused => {
            ApiError::new(StatusCode::TOO_MANY_REQUESTS, ERR_TOO_MANY_REQUESTS).into_response()
        }
        Verdict::Unavailable => {
            tracing::error!("the rate limiter lock is poisoned");
            ApiError::new(StatusCode::SERVICE_UNAVAILABLE, ERR_LIMITER_UNAVAILABLE).into_response()
        }
    }
}
