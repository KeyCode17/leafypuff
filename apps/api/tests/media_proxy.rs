use api_testing::World;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use leafypuff_api::domain::media::{ObjectKey, Variant};
use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::DependencyProbe;
use serde_json::Value;
use tower::ServiceExt;
use uuid::Uuid;

const BODY_LIMIT: usize = 1024 * 1024;
const CIPHERTEXT: &[u8] = b"\x00\x01sealed-photo-bytes\xff";

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

fn put(account_id: Uuid, entry_id: Uuid, photo_id: Uuid, variant: &str) -> Request<Body> {
    Request::builder()
        .method("PUT")
        .uri(format!(
            "/v1/media/{photo_id}/{variant}?entry_id={entry_id}"
        ))
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .header("content-type", "application/octet-stream")
        .body(Body::from(CIPHERTEXT))
        .expect("the request builds")
}

fn get(account_id: Uuid, photo_id: Uuid, variant: &str) -> Request<Body> {
    Request::builder()
        .uri(format!("/v1/media/{photo_id}/{variant}"))
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds")
}

async fn send(app: Router, request: Request<Body>) -> (StatusCode, Vec<u8>) {
    let response = app.oneshot(request).await.expect("the router answers");
    let status = response.status();
    let bytes = to_bytes(response.into_body(), BODY_LIMIT)
        .await
        .expect("the body reads");
    (status, bytes.to_vec())
}

#[tokio::test]
async fn an_object_round_trips_through_the_proxy_unchanged() {
    let world = World::default();
    let account_id = Uuid::new_v4();
    let photo_id = Uuid::new_v4();

    let (put_status, _) = send(
        router(&world),
        put(account_id, Uuid::new_v4(), photo_id, "original"),
    )
    .await;
    assert_eq!(put_status, StatusCode::CREATED);

    let (get_status, body) = send(router(&world), get(account_id, photo_id, "original")).await;

    assert_eq!(get_status, StatusCode::OK);
    assert_eq!(body, CIPHERTEXT);
}

#[tokio::test]
async fn the_storage_key_is_namespaced_by_account_and_never_by_a_client_string() {
    let world = World::default();
    let account_id = Uuid::new_v4();
    let photo_id = Uuid::new_v4();

    send(
        router(&world),
        put(account_id, Uuid::new_v4(), photo_id, "derivative"),
    )
    .await;

    assert_eq!(
        world.objects.keys(),
        vec![ObjectKey::new(account_id, photo_id, Variant::Derivative).to_string()]
    );
}

#[tokio::test]
async fn another_account_cannot_read_an_object() {
    let world = World::default();
    let owner = Uuid::new_v4();
    let photo_id = Uuid::new_v4();

    send(
        router(&world),
        put(owner, Uuid::new_v4(), photo_id, "original"),
    )
    .await;

    let (status, _) = send(router(&world), get(Uuid::new_v4(), photo_id, "original")).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn an_unknown_variant_is_refused_before_storage_is_touched() {
    let world = World::default();

    let (status, body) = send(
        router(&world),
        put(Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), "thumbnail"),
    )
    .await;
    let parsed: Value = serde_json::from_slice(&body).expect("the rejection is json");

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(parsed["error"]["code"], "UNKNOWN_VARIANT");
    assert!(world.objects.keys().is_empty());
}

#[tokio::test]
async fn an_unauthenticated_read_is_refused() {
    let world = World::default();
    let request = Request::builder()
        .uri(format!("/v1/media/{}/original", Uuid::new_v4()))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds");

    let (status, _) = send(router(&world), request).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn deleting_a_photo_removes_both_variants() {
    let world = World::default();
    let account_id = Uuid::new_v4();
    let photo_id = Uuid::new_v4();
    let entry_id = Uuid::new_v4();

    send(
        router(&world),
        put(account_id, entry_id, photo_id, "original"),
    )
    .await;
    send(
        router(&world),
        put(account_id, entry_id, photo_id, "derivative"),
    )
    .await;
    assert_eq!(world.objects.keys().len(), 2);

    let request = Request::builder()
        .method("DELETE")
        .uri(format!("/v1/media/{photo_id}"))
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds");
    let (status, _) = send(router(&world), request).await;

    assert_eq!(status, StatusCode::OK);
    assert!(world.objects.keys().is_empty());
}
