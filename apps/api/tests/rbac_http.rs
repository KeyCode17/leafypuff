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
    build_router(AppState::new(
        probe,
        world.services.clone(),
        world.sync.clone(),
        world.media.clone(),
        world.rbac.clone(),
        world.admin.clone(),
        world.catalog.clone(),
    ))
}

fn role(name: &str, permissions: Vec<Permission>) -> Role {
    Role {
        id: Uuid::new_v4(),
        name: name.to_owned(),
        description: None,
        permissions,
    }
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

fn get(path: &str, account_id: Uuid) -> Request<Body> {
    Request::builder()
        .uri(path)
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds")
}

fn post(path: &str, account_id: Uuid, body: Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::from(body.to_string()))
        .expect("the request builds")
}

#[tokio::test]
async fn reading_roles_needs_the_role_read_permission() {
    let world = World::default();
    let stranger = Uuid::new_v4();

    let (status, body) = send(router(&world), get("/v1/admin/roles", stranger)).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body["error"]["code"], "FORBIDDEN");
}

#[tokio::test]
async fn a_reader_sees_the_catalog_and_still_cannot_write() {
    let world = World::default();
    let reader_role = role("auditor", vec![Permission::RoleRead, Permission::AuditRead]);
    let reader = Uuid::new_v4();
    world.roles.define(reader_role.clone());
    world.roles.hold(reader, reader_role.id);

    let (read_status, read_body) = send(router(&world), get("/v1/admin/roles", reader)).await;
    assert_eq!(read_status, StatusCode::OK);
    assert_eq!(read_body["data"][0]["name"], json!("auditor"));

    let (write_status, _) = send(
        router(&world),
        post(
            "/v1/admin/roles/assign",
            reader,
            json!({ "account_id": Uuid::new_v4(), "role_id": reader_role.id }),
        ),
    )
    .await;

    assert_eq!(write_status, StatusCode::FORBIDDEN);
    assert!(world.audit.snapshot().is_empty());
}

#[tokio::test]
async fn assigning_a_role_writes_one_audit_row() {
    let world = World::default();
    let owner_role = role("owner", vec![Permission::RoleWrite, Permission::AuditRead]);
    let owner = Uuid::new_v4();
    world.roles.define(owner_role.clone());
    world.roles.hold(owner, owner_role.id);
    let subject = Uuid::new_v4();

    let (status, _) = send(
        router(&world),
        post(
            "/v1/admin/roles/assign",
            owner,
            json!({ "account_id": subject, "role_id": owner_role.id }),
        ),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    let recorded = world.audit.snapshot();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].actor_id, owner);
    assert_eq!(recorded[0].action.as_str(), "role.assigned");
}

#[tokio::test]
async fn the_audit_log_needs_its_own_permission() {
    let world = World::default();
    let writer_role = role("writer", vec![Permission::RoleWrite]);
    let writer = Uuid::new_v4();
    world.roles.define(writer_role.clone());
    world.roles.hold(writer, writer_role.id);

    let (status, _) = send(router(&world), get("/v1/admin/audit", writer)).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn the_caller_can_always_read_the_permissions_it_holds() {
    let world = World::default();
    let held = role("support", vec![Permission::AccountList]);
    let caller = Uuid::new_v4();
    world.roles.define(held.clone());
    world.roles.hold(caller, held.id);

    let (status, body) = send(router(&world), get("/v1/admin/permissions", caller)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["permissions"], json!(["account:list"]));
}

#[tokio::test]
async fn an_unauthenticated_caller_reaches_no_admin_route() {
    let world = World::default();
    let request = Request::builder()
        .uri("/v1/admin/roles")
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds");

    let (status, _) = send(router(&world), request).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
