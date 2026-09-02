#![cfg(feature = "sync")]
#![allow(clippy::expect_used)]

use leafypuff_core::domain::{CoreError, Rejection};
use leafypuff_core::infrastructure::{MediaSync, VaultSync};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

const DEVICE: &str = "device-under-test";
const HEADER: &str = "x-device-id: device-under-test";

async fn recording_server(
    status: &'static str,
    body: &'static str,
) -> (String, oneshot::Receiver<String>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a loopback port is free");
    let address = listener.local_addr().expect("the port is known");
    let (sender, receiver) = oneshot::channel();

    tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("the client connects");
        let mut buffer = vec![0_u8; 8192];
        let read = socket.read(&mut buffer).await.expect("the request arrives");
        let head = String::from_utf8_lossy(&buffer[..read]).into_owned();
        let response = format!(
            "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{body}",
            body.len()
        );
        socket
            .write_all(response.as_bytes())
            .await
            .expect("the answer is written");
        socket.flush().await.expect("the answer is flushed");
        let _ = sender.send(head);
    });

    (format!("http://{address}"), receiver)
}

#[tokio::test]
async fn the_vault_client_names_its_device_on_every_request() {
    let (base_url, captured) = recording_server("200 OK", r#"{"success":true,"data":[]}"#).await;

    let held = VaultSync::new(base_url, "token".to_owned(), DEVICE)
        .expect("the client builds")
        .pull()
        .await
        .expect("an empty list is not a failure");

    assert!(held.is_none());
    let head = captured.await.expect("the server captured the request");
    assert!(
        head.to_lowercase().contains(HEADER),
        "the vault client must name its device, or the server answers 400: {head}"
    );
}

#[tokio::test]
async fn the_media_client_names_its_device_on_every_request() {
    let (base_url, captured) = recording_server("404 Not Found", "{}").await;

    let found = MediaSync::new(base_url, "token".to_owned(), DEVICE)
        .expect("the client builds")
        .download("photo", leafypuff_core::domain::PhotoKind::Original)
        .await
        .expect("a missing photo is not a failure");

    assert!(found.is_none());
    let head = captured.await.expect("the server captured the request");
    assert!(
        head.to_lowercase().contains(HEADER),
        "the media client must name its device, or the server answers 400: {head}"
    );
}

#[tokio::test]
async fn a_refused_vault_read_is_not_reported_as_an_unreadable_answer() {
    let (base_url, _captured) = recording_server(
        "400 Bad Request",
        r#"{"success":false,"data":null,"error":{"code":"DEVICE_UNIDENTIFIED"}}"#,
    )
    .await;

    let failure = VaultSync::new(base_url, "token".to_owned(), DEVICE)
        .expect("the client builds")
        .pull()
        .await
        .expect_err("a refused read must fail");

    assert!(
        matches!(failure, CoreError::Storage(_)),
        "a refusal names the status, it does not blame the shape: {failure:?}"
    );
}

#[tokio::test]
async fn an_expired_session_is_named_as_one_rather_than_a_storage_failure() {
    let (base_url, _captured) = recording_server(
        "401 Unauthorized",
        r#"{"success":false,"data":null,"error":{"code":"UNAUTHENTICATED"}}"#,
    )
    .await;

    let failure = VaultSync::new(base_url, "stale".to_owned(), DEVICE)
        .expect("the client builds")
        .pull()
        .await
        .expect_err("a stale token must fail");

    assert!(
        matches!(
            failure,
            CoreError::Rejected {
                rejection: Rejection::InvalidCredentials,
                ..
            }
        ),
        "the app has to know to renew, not to report a broken server: {failure:?}"
    );
}
