#![cfg(feature = "sync")]
#![allow(clippy::expect_used)]

use leafypuff_core::domain::PhotoKind;
use leafypuff_core::infrastructure::profile_sync::{ProfileSync, RemoteProfile};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const DEVICE: &str = "device-under-test";
const SEALED: &str = "c2VhbGVk";

async fn server_answering(status: &'static str, body: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("a loopback port is free");
    let address = listener.local_addr().expect("the port is known");

    tokio::spawn(async move {
        while let Ok((mut socket, _)) = listener.accept().await {
            let mut buffer = vec![0_u8; 8192];
            let _ = socket.read(&mut buffer).await;
            let response = format!(
                "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\n\r\n{body}",
                body.len()
            );
            let _ = socket.write_all(response.as_bytes()).await;
            let _ = socket.flush().await;
        }
    });

    format!("http://{address}")
}

fn wanted() -> RemoteProfile {
    RemoteProfile {
        sealed_profile: Some(b"sealed".to_vec()),
        avatar_photo_id: Some("photo".to_owned()),
        updated_at_ms: 42,
    }
}

#[tokio::test]
async fn a_server_without_the_route_reads_as_an_empty_profile() {
    let base_url = server_answering("404 Not Found", "{}").await;

    let held = ProfileSync::new(base_url, "token".to_owned(), DEVICE)
        .expect("the client builds")
        .pull()
        .await
        .expect("an older server is not a failure");

    assert_eq!(held.updated_at_ms, 0);
    assert!(held.sealed_profile.is_none());
    assert!(held.avatar_photo_id.is_none());
}

#[tokio::test]
async fn a_push_to_a_server_without_the_route_keeps_what_it_sent() {
    let base_url = server_answering("404 Not Found", "{}").await;

    let answered = ProfileSync::new(base_url, "token".to_owned(), DEVICE)
        .expect("the client builds")
        .push(&wanted())
        .await
        .expect("an older server is not a failure");

    assert_eq!(answered.updated_at_ms, 42);
    assert_eq!(answered.avatar_photo_id.as_deref(), Some("photo"));
}

#[tokio::test]
async fn an_avatar_upload_to_a_server_without_the_route_is_not_a_failure() {
    let base_url = server_answering("404 Not Found", "{}").await;

    ProfileSync::new(base_url, "token".to_owned(), DEVICE)
        .expect("the client builds")
        .upload_avatar(PhotoKind::Original, b"sealed".to_vec())
        .await
        .expect("an older server is not a failure");
}

#[tokio::test]
async fn a_refused_read_is_still_a_failure() {
    let base_url = server_answering("500 Internal Server Error", "{}").await;

    let refusal = ProfileSync::new(base_url, "token".to_owned(), DEVICE)
        .expect("the client builds")
        .pull()
        .await;

    assert!(refusal.is_err(), "a broken server must not read as empty");
}

#[tokio::test]
async fn a_profile_the_server_holds_is_read_back() {
    let body: &'static str = concat!(
        r#"{"success":true,"data":{"sealed_profile":"c2VhbGVk","#,
        r#""avatar_photo_id":"photo","updated_at_ms":7}}"#
    );
    let base_url = server_answering("200 OK", body).await;

    let held = ProfileSync::new(base_url, "token".to_owned(), DEVICE)
        .expect("the client builds")
        .pull()
        .await
        .expect("the profile reads");

    assert_eq!(held.updated_at_ms, 7);
    assert_eq!(held.avatar_photo_id.as_deref(), Some("photo"));
    assert_eq!(
        held.sealed_profile.as_deref(),
        Some(
            data_encoding::BASE64
                .decode(SEALED.as_bytes())
                .expect("the fixture decodes")
                .as_slice()
        )
    );
}
