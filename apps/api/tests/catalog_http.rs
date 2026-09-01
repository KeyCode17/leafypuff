use api_testing::World;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use leafypuff_api::domain::rbac::{Permission, Role};
use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::DependencyProbe;
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

const BODY_LIMIT: usize = 128 * 1024;

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

fn operator(world: &World, permissions: Vec<Permission>) -> Uuid {
    let role = Role {
        id: Uuid::new_v4(),
        name: "operator".to_owned(),
        description: None,
        permissions,
    };
    let caller = Uuid::new_v4();
    world.roles.define(role.clone());
    world.roles.hold(caller, role.id);
    caller
}

async fn send(app: Router, request: Request<Body>) -> (StatusCode, Value) {
    let response = app.oneshot(request).await.expect("the router answers");
    let status = response.status();
    let bytes = to_bytes(response.into_body(), BODY_LIMIT)
        .await
        .expect("the body reads");
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

fn get(path: &str, caller: Uuid) -> Request<Body> {
    Request::builder()
        .uri(path)
        .header("authorization", format!("Bearer access:{caller}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds")
}

fn post(path: &str, caller: Uuid, body: Option<Value>) -> Request<Body> {
    let builder = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer access:{caller}"))
        .header("x-device-id", Uuid::new_v4().to_string());
    builder
        .body(body.map_or_else(Body::empty, |value| Body::from(value.to_string())))
        .expect("the request builds")
}

fn payload() -> Value {
    json!({ "moods": [{ "id": "hopeful", "label": "Hopeful" }], "stickers": [] })
}

#[tokio::test]
async fn drafting_needs_write_and_publishing_needs_publish() {
    let world = World::default();
    let writer = operator(&world, vec![Permission::CatalogWrite]);

    let (drafted, body) = send(
        router(&world),
        post(
            "/v1/admin/catalog",
            writer,
            Some(json!({ "payload": payload() })),
        ),
    )
    .await;
    assert_eq!(drafted, StatusCode::CREATED);
    let bundle_id = body["data"]["id"].as_str().expect("an id").to_owned();

    let (refused, _) = send(
        router(&world),
        post(
            &format!("/v1/admin/catalog/{bundle_id}/publish"),
            writer,
            None,
        ),
    )
    .await;

    assert_eq!(refused, StatusCode::FORBIDDEN);
    assert!(world.audit.snapshot().is_empty());
}

#[tokio::test]
async fn publishing_replaces_the_live_bundle_and_writes_an_audit_row() {
    let world = World::default();
    let author = operator(
        &world,
        vec![Permission::CatalogWrite, Permission::CatalogPublish],
    );

    let mut ids = Vec::new();
    for _ in 0..2 {
        let (_, body) = send(
            router(&world),
            post(
                "/v1/admin/catalog",
                author,
                Some(json!({ "payload": payload() })),
            ),
        )
        .await;
        ids.push(body["data"]["id"].as_str().expect("an id").to_owned());
    }

    for id in &ids {
        let (status, _) = send(
            router(&world),
            post(&format!("/v1/admin/catalog/{id}/publish"), author, None),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
    }

    let live: Vec<_> = world
        .bundles
        .snapshot()
        .into_iter()
        .filter(leafypuff_api::domain::catalog::CatalogBundle::is_published)
        .collect();
    assert_eq!(live.len(), 1);
    assert_eq!(live[0].id.to_string(), ids[1]);
    assert_eq!(world.audit.snapshot().len(), 2);
}

#[tokio::test]
async fn a_device_reads_the_published_catalog_without_an_operator_permission() {
    let world = World::default();
    let author = operator(
        &world,
        vec![Permission::CatalogWrite, Permission::CatalogPublish],
    );
    let (_, drafted) = send(
        router(&world),
        post(
            "/v1/admin/catalog",
            author,
            Some(json!({ "payload": payload() })),
        ),
    )
    .await;
    let bundle_id = drafted["data"]["id"].as_str().expect("an id").to_owned();
    send(
        router(&world),
        post(
            &format!("/v1/admin/catalog/{bundle_id}/publish"),
            author,
            None,
        ),
    )
    .await;

    let device = Uuid::new_v4();
    let (status, body) = send(router(&world), get("/v1/catalog", device)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["payload"]["moods"][0]["id"], json!("hopeful"));
    assert_eq!(body["data"]["published"], json!(true));
}

#[tokio::test]
async fn a_device_asking_before_anything_is_published_gets_a_definite_answer() {
    let world = World::default();

    let (status, body) = send(router(&world), get("/v1/catalog", Uuid::new_v4())).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], json!("NO_CATALOG_PUBLISHED"));
}

#[tokio::test]
async fn a_payload_that_is_not_an_object_is_refused() {
    let world = World::default();
    let writer = operator(&world, vec![Permission::CatalogWrite]);

    let (status, _) = send(
        router(&world),
        post(
            "/v1/admin/catalog",
            writer,
            Some(json!({ "payload": ["not", "an", "object"] })),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(world.bundles.snapshot().is_empty());
}
