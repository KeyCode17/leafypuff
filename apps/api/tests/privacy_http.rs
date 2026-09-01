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

fn raise(caller: Uuid, kind: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/v1/data-requests")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer access:{caller}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::from(json!({ "kind": kind }).to_string()))
        .expect("the request builds")
}

fn fulfil(caller: Uuid, request_id: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(format!("/v1/admin/data-requests/{request_id}/fulfil"))
        .header("authorization", format!("Bearer access:{caller}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds")
}

#[tokio::test]
async fn anyone_signed_in_may_ask_and_only_an_operator_may_fulfil() {
    let world = World::default();
    let person = Uuid::new_v4();

    let (raised, body) = send(router(&world), raise(person, "erasure")).await;
    assert_eq!(raised, StatusCode::CREATED);
    let request_id = body["data"]["id"].as_str().expect("an id").to_owned();

    let (refused, _) = send(router(&world), fulfil(person, &request_id)).await;

    assert_eq!(refused, StatusCode::FORBIDDEN);
    assert!(world.eraser.erased().is_empty());
}

#[tokio::test]
async fn fulfilling_an_erasure_erases_and_records_one_audit_row() {
    let world = World::default();
    let person = Uuid::new_v4();
    let (_, raised) = send(router(&world), raise(person, "erasure")).await;
    let request_id = raised["data"]["id"].as_str().expect("an id").to_owned();
    let caller = operator(&world, vec![Permission::DataRequestFulfil]);

    let (status, _) = send(router(&world), fulfil(caller, &request_id)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(world.eraser.erased(), vec![person]);
    let recorded = world.audit.snapshot();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].action.as_str(), "data_request.fulfilled");
}

#[tokio::test]
async fn fulfilling_an_export_records_the_act_without_erasing() {
    let world = World::default();
    let person = Uuid::new_v4();
    let (_, raised) = send(router(&world), raise(person, "export")).await;
    let request_id = raised["data"]["id"].as_str().expect("an id").to_owned();
    let caller = operator(&world, vec![Permission::DataRequestFulfil]);

    let (status, _) = send(router(&world), fulfil(caller, &request_id)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(world.eraser.erased().is_empty());
    assert_eq!(world.audit.snapshot().len(), 1);
}

#[tokio::test]
async fn a_second_fulfilment_is_refused_rather_than_erasing_twice() {
    let world = World::default();
    let person = Uuid::new_v4();
    let (_, raised) = send(router(&world), raise(person, "erasure")).await;
    let request_id = raised["data"]["id"].as_str().expect("an id").to_owned();
    let caller = operator(&world, vec![Permission::DataRequestFulfil]);
    send(router(&world), fulfil(caller, &request_id)).await;

    let (status, body) = send(router(&world), fulfil(caller, &request_id)).await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"]["code"], json!("ALREADY_FULFILLED"));
    assert_eq!(world.eraser.erased().len(), 1);
}

#[tokio::test]
async fn an_unknown_kind_is_refused() {
    let world = World::default();

    let (status, body) = send(router(&world), raise(Uuid::new_v4(), "delete-everything")).await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(body["error"]["code"], json!("UNKNOWN_REQUEST_KIND"));
    assert!(world.requests.snapshot().is_empty());
}

#[tokio::test]
async fn reading_open_requests_needs_its_own_permission() {
    let world = World::default();
    let caller = operator(&world, vec![Permission::DataRequestFulfil]);
    let request = Request::builder()
        .uri("/v1/admin/data-requests")
        .header("authorization", format!("Bearer access:{caller}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds");

    let (status, _) = send(router(&world), request).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}
