use api_testing::World;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use leafypuff_api::domain::admin::AccountSummary;
use leafypuff_api::domain::rbac::{Permission, Role};
use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::DependencyProbe;
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

const BODY_LIMIT: usize = 64 * 1024;

/// The whole point of the admin surface: counts, dates and flags. If a key ever appears outside
/// this list, someone put a person's diary on an operator's screen.
const ALLOWED_KEYS: [&str; 9] = [
    "account_id",
    "email",
    "entry_count",
    "first_entry_date",
    "last_entry_date",
    "media_bytes",
    "media_object_count",
    "suspended",
    "verified",
];

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

fn summary(account_id: Uuid) -> AccountSummary {
    AccountSummary {
        account_id,
        email: "person@example.test".to_owned(),
        verified: true,
        suspended: false,
        entry_count: 12,
        first_entry_date: Some("2026-01-04".to_owned()),
        last_entry_date: Some("2026-09-01".to_owned()),
        media_object_count: 4,
        media_bytes: 2_048,
    }
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

fn post(path: &str, caller: Uuid) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(path)
        .header("authorization", format!("Bearer access:{caller}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds")
}

#[tokio::test]
async fn the_account_row_carries_counts_and_metadata_and_nothing_else() {
    let world = World::default();
    world.directory.add(summary(Uuid::new_v4()));
    let caller = operator(&world, vec![Permission::AccountList]);

    let (status, body) = send(router(&world), get("/v1/admin/accounts", caller)).await;

    assert_eq!(status, StatusCode::OK);
    let row = body["data"][0].as_object().expect("a row is an object");
    let mut keys: Vec<&str> = row.keys().map(String::as_str).collect();
    keys.sort_unstable();
    assert_eq!(keys, ALLOWED_KEYS);

    let raw = body.to_string();
    for forbidden in ["title", "body", "tags", "mood", "ciphertext", "storage_key"] {
        assert!(!raw.contains(forbidden), "{forbidden} reached the operator");
    }
}

#[tokio::test]
async fn listing_accounts_needs_account_list() {
    let world = World::default();
    let caller = operator(&world, vec![Permission::AccountRead]);

    let (status, _) = send(router(&world), get("/v1/admin/accounts", caller)).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn suspending_needs_its_own_permission_and_writes_an_audit_row() {
    let world = World::default();
    let subject = Uuid::new_v4();
    world.directory.add(summary(subject));
    let reader = operator(&world, vec![Permission::AccountRead]);
    let suspender = operator(&world, vec![Permission::AccountSuspend]);

    let (refused, _) = send(
        router(&world),
        post(&format!("/v1/admin/accounts/{subject}/suspend"), reader),
    )
    .await;
    assert_eq!(refused, StatusCode::FORBIDDEN);
    assert!(world.audit.snapshot().is_empty());

    let (allowed, _) = send(
        router(&world),
        post(&format!("/v1/admin/accounts/{subject}/suspend"), suspender),
    )
    .await;

    assert_eq!(allowed, StatusCode::OK);
    assert!(world.directory.snapshot()[0].suspended);
    let recorded = world.audit.snapshot();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].action.as_str(), "account.suspended");
}

#[tokio::test]
async fn restoring_clears_the_suspension() {
    let world = World::default();
    let subject = Uuid::new_v4();
    world.directory.add(summary(subject));
    let caller = operator(
        &world,
        vec![Permission::AccountSuspend, Permission::AccountRestore],
    );

    send(
        router(&world),
        post(&format!("/v1/admin/accounts/{subject}/suspend"), caller),
    )
    .await;
    let (status, _) = send(
        router(&world),
        post(&format!("/v1/admin/accounts/{subject}/restore"), caller),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(!world.directory.snapshot()[0].suspended);
}

#[tokio::test]
async fn an_unknown_account_is_a_404_rather_than_an_empty_row() {
    let world = World::default();
    let caller = operator(&world, vec![Permission::AccountRead]);

    let (status, body) = send(
        router(&world),
        get(&format!("/v1/admin/accounts/{}", Uuid::new_v4()), caller),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], json!("ACCOUNT_NOT_FOUND"));
}
