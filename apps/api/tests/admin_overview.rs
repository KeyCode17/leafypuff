use api_testing::World;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use leafypuff_api::domain::admin::ServiceOverview;
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
    ))
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

fn overview() -> ServiceOverview {
    ServiceOverview {
        account_count: 12,
        verified_account_count: 9,
        suspended_account_count: 1,
        entry_count: 340,
        tombstoned_entry_count: 7,
        device_count: 15,
        devices_synced_last_day: 11,
        field_conflict_count: 2,
        media_object_count: 88,
        media_bytes: 12_582_912,
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

fn get(caller: Uuid) -> Request<Body> {
    Request::builder()
        .uri("/v1/admin/overview")
        .header("authorization", format!("Bearer access:{caller}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds")
}

#[tokio::test]
async fn the_overview_reports_counts_and_storage() {
    let world = World::default();
    world.metrics.publish(overview());
    let caller = operator(&world, vec![Permission::EntryCountRead]);

    let (status, body) = send(router(&world), get(caller)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["account_count"], json!(12));
    assert_eq!(body["data"]["devices_synced_last_day"], json!(11));
    assert_eq!(body["data"]["media_bytes"], json!(12_582_912));
}

#[tokio::test]
async fn the_overview_carries_no_field_that_could_hold_content() {
    let world = World::default();
    world.metrics.publish(overview());
    let caller = operator(&world, vec![Permission::EntryCountRead]);

    let (_, body) = send(router(&world), get(caller)).await;

    let row = body["data"].as_object().expect("the overview is an object");
    for value in row.values() {
        assert!(value.is_number(), "every figure must be a number");
    }
}

#[tokio::test]
async fn the_overview_needs_the_count_permission() {
    let world = World::default();
    world.metrics.publish(overview());
    let caller = operator(&world, vec![Permission::AccountList]);

    let (status, _) = send(router(&world), get(caller)).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}
