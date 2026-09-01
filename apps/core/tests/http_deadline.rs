#![cfg(feature = "sync")]

use leafypuff_core::domain::CoreError;
use leafypuff_core::infrastructure::http_error::reached;

const UNREACHABLE: &str = "The service could not be reached";

/// A request that never gets an answer must become a timeout rather than a "no connection", which
/// is what a caller shows the owner and the two call for different actions.
#[tokio::test]
async fn a_request_that_outlives_its_deadline_reports_a_timeout() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a loopback port");
    let port = listener.local_addr().expect("an address").port();

    // Accepts the connection and then says nothing at all, which is what a phone leaving wifi
    // looks like from the other end.
    tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("a connection");
        std::mem::forget(stream);
    });

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(300))
        .build()
        .expect("a client");

    let error = client
        .get(format!("http://127.0.0.1:{port}/"))
        .send()
        .await
        .expect_err("a silent server must not answer");

    assert!(matches!(
        reached(&error, UNREACHABLE),
        CoreError::Timeout(_)
    ));
}

#[tokio::test]
async fn a_refused_connection_is_not_reported_as_a_timeout() {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("a client");

    // Port 1 on loopback refuses immediately.
    let error = client
        .get("http://127.0.0.1:1/")
        .send()
        .await
        .expect_err("a refused port must not answer");

    assert!(matches!(
        reached(&error, UNREACHABLE),
        CoreError::Storage(_)
    ));
}
