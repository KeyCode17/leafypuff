use api_testing::World;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use data_encoding::BASE64;
use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::DependencyProbe;
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

const BODY_LIMIT: usize = 64 * 1024;

fn router(world: &World) -> Router {
    let probe = DependencyProbe::new("postgres://unused".to_owned(), "127.0.0.1:3900".to_owned());
    build_router(AppState {
        readiness: probe,
        iam: world.services.clone(),
        sync: world.sync.clone(),
        media: world.media.clone(),
        rbac: world.rbac.clone(),
        admin: world.admin.clone(),
        catalog: world.catalog.clone(),
        privacy: world.privacy.clone(),
        release: world.release.clone(),
    })
}

fn envelope(plaintext: &[u8], updated_at_ms: i64, device_id: Uuid) -> Value {
    json!({
        "ciphertext": BASE64.encode(plaintext),
        "nonce": BASE64.encode(&[7_u8; 24]),
        "updated_at_ms": updated_at_ms,
        "device_id": device_id,
    })
}

fn record(id: Uuid, device_id: Uuid, at: i64) -> Value {
    record_carrying(id, device_id, at, "[]")
}

fn record_carrying(id: Uuid, device_id: Uuid, at: i64, photo_refs: &str) -> Value {
    json!({
        "id": id,
        "date": "2026-09-01",
        "mood": "calm",
        "tags": ["rain"],
        "sticker_placements": "[]",
        "photo_refs": photo_refs,
        "weather": "sunny",
        "location": "home",
        "device_updated_at_ms": at,
        "deleted_at_ms": Value::Null,
        "title": envelope(b"sealed-title", at, device_id),
        "body": envelope(b"sealed-body", at, device_id),
    })
}

async fn send(app: Router, request: Request<Body>) -> (StatusCode, Value) {
    let response = app.oneshot(request).await.expect("the router answers");
    let status = response.status();
    let bytes = to_bytes(response.into_body(), BODY_LIMIT)
        .await
        .expect("the body reads");
    let parsed = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, parsed)
}

fn push_request(account_id: Uuid, device_id: Uuid, key: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/v1/sync/push")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", device_id.to_string())
        .header("idempotency-key", key)
        .body(Body::from(body.to_string()))
        .expect("the request builds")
}

fn pull_request(account_id: Uuid, device_id: Uuid, cursor: i64) -> Request<Body> {
    Request::builder()
        .uri(format!("/v1/sync/pull?cursor={cursor}"))
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", device_id.to_string())
        .body(Body::empty())
        .expect("the request builds")
}

#[tokio::test]
async fn a_pull_without_a_bearer_token_is_refused() {
    let world = World::default();
    let request = Request::builder()
        .uri("/v1/sync/pull")
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds");

    let (status, body) = send(router(&world), request).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"]["code"], "UNAUTHENTICATED");
}

#[tokio::test]
async fn a_pull_without_a_device_header_is_refused() {
    let world = World::default();
    let request = Request::builder()
        .uri("/v1/sync/pull")
        .header("authorization", format!("Bearer access:{}", Uuid::new_v4()))
        .body(Body::empty())
        .expect("the request builds");

    let (status, body) = send(router(&world), request).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "DEVICE_UNIDENTIFIED");
}

#[tokio::test]
async fn a_push_without_an_idempotency_key_is_refused() {
    let world = World::default();
    let account_id = Uuid::new_v4();
    let device_id = Uuid::new_v4();
    let request = Request::builder()
        .method("POST")
        .uri("/v1/sync/push")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", device_id.to_string())
        .body(Body::from(
            json!({ "records": [record(Uuid::new_v4(), device_id, 1_000)] }).to_string(),
        ))
        .expect("the request builds");

    let (status, body) = send(router(&world), request).await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"]["code"], "IDEMPOTENCY_KEY_MISSING");
}

#[tokio::test]
async fn a_pushed_entry_comes_back_from_a_pull_still_sealed() {
    let world = World::default();
    let account_id = Uuid::new_v4();
    let device_id = Uuid::new_v4();
    let entry_id = Uuid::new_v4();

    let (push_status, push_body) = send(
        router(&world),
        push_request(
            account_id,
            device_id,
            &Uuid::new_v4().to_string(),
            json!({ "records": [record(entry_id, device_id, 1_000)] }),
        ),
    )
    .await;
    assert_eq!(push_status, StatusCode::OK);
    assert_eq!(push_body["success"], json!(true));
    assert_eq!(push_body["data"]["replayed"], json!(false));

    let (pull_status, pull_body) =
        send(router(&world), pull_request(account_id, Uuid::new_v4(), 0)).await;

    assert_eq!(pull_status, StatusCode::OK);
    let records = pull_body["data"]["records"]
        .as_array()
        .expect("records is an array");
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["id"], json!(entry_id));
    assert_eq!(
        records[0]["title"]["ciphertext"],
        json!(BASE64.encode(b"sealed-title"))
    );
}

#[tokio::test]
async fn the_pull_body_never_carries_a_plaintext_field() {
    let world = World::default();
    let account_id = Uuid::new_v4();
    let device_id = Uuid::new_v4();

    send(
        router(&world),
        push_request(
            account_id,
            device_id,
            &Uuid::new_v4().to_string(),
            json!({ "records": [record(Uuid::new_v4(), device_id, 1_000)] }),
        ),
    )
    .await;
    let (_, body) = send(router(&world), pull_request(account_id, Uuid::new_v4(), 0)).await;

    let record = &body["data"]["records"][0];
    let keys: Vec<&String> = record
        .as_object()
        .expect("a record is an object")
        .keys()
        .collect();
    assert!(!keys.contains(&&"title_plaintext".to_owned()));
    let title = record["title"].as_object().expect("title is an object");
    let mut envelope_keys: Vec<&String> = title.keys().collect();
    envelope_keys.sort();
    assert_eq!(
        envelope_keys,
        vec!["ciphertext", "device_id", "nonce", "updated_at_ms"]
    );
}

#[tokio::test]
async fn a_replayed_push_writes_nothing_more() {
    let world = World::default();
    let account_id = Uuid::new_v4();
    let device_id = Uuid::new_v4();
    let entry_id = Uuid::new_v4();
    let key = Uuid::new_v4().to_string();
    let body = json!({ "records": [record(entry_id, device_id, 1_000)] });

    send(
        router(&world),
        push_request(account_id, device_id, &key, body.clone()),
    )
    .await;
    let revision_after_first = world.entries.snapshot()[0].revision;

    let (status, replay) = send(
        router(&world),
        push_request(account_id, device_id, &key, body),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(replay["data"]["replayed"], json!(true));
    assert_eq!(world.entries.snapshot()[0].revision, revision_after_first);
    assert!(world.conflicts.snapshot().is_empty());
}

#[tokio::test]
async fn an_entry_pushed_for_another_account_is_refused() {
    let world = World::default();
    let device_id = Uuid::new_v4();
    let owner = Uuid::new_v4();
    let entry_id = Uuid::new_v4();

    send(
        router(&world),
        push_request(
            owner,
            device_id,
            &Uuid::new_v4().to_string(),
            json!({ "records": [record(entry_id, device_id, 1_000)] }),
        ),
    )
    .await;

    let (status, body) = send(
        router(&world),
        pull_request(Uuid::new_v4(), Uuid::new_v4(), 0),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(
        body["data"]["records"]
            .as_array()
            .expect("records is an array")
            .is_empty()
    );
}

#[tokio::test]
async fn a_wrapped_key_round_trips_as_an_opaque_blob() {
    let world = World::default();
    let account_id = Uuid::new_v4();
    let device_id = Uuid::new_v4();
    let blob = BASE64.encode(b"wrapped-content-key");

    let put = Request::builder()
        .method("PUT")
        .uri("/v1/sync/keys")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", device_id.to_string())
        .body(Body::from(
            json!({
                "kind": "passphrase",
                "blob": blob,
                "salt": BASE64.encode(&[1_u8; 16]),
                "updated_at_ms": 1_000,
            })
            .to_string(),
        ))
        .expect("the request builds");
    let (put_status, _) = send(router(&world), put).await;
    assert_eq!(put_status, StatusCode::OK);

    let read = Request::builder()
        .uri("/v1/sync/keys")
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", device_id.to_string())
        .body(Body::empty())
        .expect("the request builds");
    let (read_status, body) = send(router(&world), read).await;

    assert_eq!(read_status, StatusCode::OK);
    assert_eq!(body["data"][0]["kind"], json!("passphrase"));
    assert_eq!(body["data"][0]["blob"], json!(blob));
}

#[tokio::test]
async fn a_pushed_record_carries_its_photo_references_back_out() {
    let world = World::default();
    let account_id = Uuid::new_v4();
    let device_id = Uuid::new_v4();
    let refs = r#"[{"id":"3f2a91c0-0000-4000-8000-0000000000aa","ordinal":0}]"#;

    let (status, _) = send(
        router(&world),
        push_request(
            account_id,
            device_id,
            "photo-refs",
            json!({ "records": [record_carrying(Uuid::new_v4(), device_id, 10, refs)] }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, body) = send(router(&world), pull_request(account_id, Uuid::new_v4(), 0)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["records"][0]["photo_refs"], refs);
    assert_eq!(body["data"]["records"][0]["weather"], "sunny");
    assert_eq!(body["data"]["records"][0]["location"], "home");
}

#[tokio::test]
async fn a_pull_never_returns_another_accounts_entries() {
    let world = World::default();
    let mine = Uuid::new_v4();
    let theirs = Uuid::new_v4();
    let device = Uuid::new_v4();

    let (status, _) = send(
        router(&world),
        push_request(
            theirs,
            device,
            "theirs",
            json!({ "records": [record(Uuid::new_v4(), device, 10)] }),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, body) = send(router(&world), pull_request(mine, Uuid::new_v4(), 0)).await;

    assert_eq!(status, StatusCode::OK);
    // Not "no titles leaked" -- no rows at all. A neighbour's entry has no business being
    // counted, dated or moods-listed here either.
    assert_eq!(body["data"]["records"].as_array().map(Vec::len), Some(0));
}

#[tokio::test]
async fn a_wrapped_vault_key_is_never_handed_to_another_account() {
    let world = World::default();
    let mine = Uuid::new_v4();
    let theirs = Uuid::new_v4();

    let (status, _) = send(
        router(&world),
        Request::builder()
            .method("PUT")
            .uri("/v1/sync/keys")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer access:{theirs}"))
            .header("x-device-id", Uuid::new_v4().to_string())
            .body(Body::from(
                json!({
                    "kind": "passphrase",
                    "blob": BASE64.encode(b"their-wrapped-content-key"),
                    "salt": BASE64.encode(b"their-salt"),
                    "updated_at_ms": 1,
                })
                .to_string(),
            ))
            .expect("the request builds"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);

    let (status, body) = send(
        router(&world),
        Request::builder()
            .uri("/v1/sync/keys")
            .header("authorization", format!("Bearer access:{mine}"))
            .header("x-device-id", Uuid::new_v4().to_string())
            .body(Body::empty())
            .expect("the request builds"),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"].as_array().map(Vec::len), Some(0));
    // The wrapped key is what a diary is sealed under. Handing one to the wrong account is the
    // one leak that would matter even though the server cannot open it itself.
    assert!(!body.to_string().contains("their-wrapped-content-key"));
}
