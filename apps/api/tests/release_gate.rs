use api_testing::World;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use leafypuff_api::domain::rbac::{Permission, Role};
use leafypuff_api::domain::release::{Platform, ReleaseGate};
use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::DependencyProbe;
use serde_json::{Value, json};
use tower::ServiceExt;
use uuid::Uuid;

const BODY_LIMIT: usize = 64 * 1024;
const OLD_BUILD: &str = "7";
const CURRENT_BUILD: &str = "42";

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

fn gate(minimum_build: i32, force_update: bool) -> ReleaseGate {
    ReleaseGate {
        platform: Platform::Android,
        minimum_build,
        force_update,
        message: Some("Update to keep syncing".to_owned()),
        updated_at_ms: 1_756_000_000_000,
        updated_by: None,
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

fn pull(account_id: Uuid, build: &str) -> Request<Body> {
    Request::builder()
        .uri("/v1/sync/pull?cursor=0")
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .header("x-app-platform", "android")
        .header("x-app-build", build)
        .body(Body::empty())
        .expect("the request builds")
}

fn media_read(account_id: Uuid, build: &str) -> Request<Body> {
    Request::builder()
        .uri(format!("/v1/media/{}/original", Uuid::new_v4()))
        .header("authorization", format!("Bearer access:{account_id}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .header("x-app-platform", "android")
        .header("x-app-build", build)
        .body(Body::empty())
        .expect("the request builds")
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

#[tokio::test]
async fn a_forced_gate_blocks_sync_and_nothing_else() {
    let world = World::default();
    world.gates.set(gate(40, true));
    let device = Uuid::new_v4();

    let (sync_status, body) = send(router(&world), pull(device, OLD_BUILD)).await;
    assert_eq!(sync_status, StatusCode::UPGRADE_REQUIRED);
    assert_eq!(body["error"]["code"], json!("UPDATE_REQUIRED"));
    assert_eq!(body["error"]["detail"], json!("Update to keep syncing"));

    let (media_status, _) = send(router(&world), media_read(device, OLD_BUILD)).await;
    assert_eq!(
        media_status,
        StatusCode::NOT_FOUND,
        "a blocked build must still reach its own photos"
    );
}

#[tokio::test]
async fn a_minimum_without_force_update_is_a_nudge_rather_than_an_outage() {
    let world = World::default();
    world.gates.set(gate(40, false));

    let (status, _) = send(router(&world), pull(Uuid::new_v4(), OLD_BUILD)).await;

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn a_current_build_syncs_even_with_force_update_set() {
    let world = World::default();
    world.gates.set(gate(40, true));

    let (status, _) = send(router(&world), pull(Uuid::new_v4(), CURRENT_BUILD)).await;

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn a_device_that_sends_no_build_header_is_not_blocked() {
    let world = World::default();
    world.gates.set(gate(40, true));
    let request = Request::builder()
        .uri("/v1/sync/pull?cursor=0")
        .header("authorization", format!("Bearer access:{}", Uuid::new_v4()))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds");

    let (status, _) = send(router(&world), request).await;

    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn the_device_gate_endpoint_reports_behind_and_blocked_separately() {
    let world = World::default();
    world.gates.set(gate(40, false));
    let request = Request::builder()
        .uri("/v1/release?platform=android&build=7")
        .header("authorization", format!("Bearer access:{}", Uuid::new_v4()))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds");

    let (status, body) = send(router(&world), request).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["behind"], json!(true));
    assert_eq!(body["data"]["blocked"], json!(false));
}

#[tokio::test]
async fn setting_a_gate_needs_release_write_and_writes_an_audit_row() {
    let world = World::default();
    let reader = operator(&world, vec![Permission::ReleaseRead]);
    let writer = operator(&world, vec![Permission::ReleaseWrite]);
    let body = json!({
        "platform": "android",
        "minimum_build": 41,
        "force_update": true,
        "message": "Update to keep syncing",
    });
    let post = |caller: Uuid| {
        Request::builder()
            .method("POST")
            .uri("/v1/admin/release")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer access:{caller}"))
            .header("x-device-id", Uuid::new_v4().to_string())
            .body(Body::from(body.to_string()))
            .expect("the request builds")
    };

    let (refused, _) = send(router(&world), post(reader)).await;
    assert_eq!(refused, StatusCode::FORBIDDEN);
    assert!(world.audit.snapshot().is_empty());

    let (allowed, _) = send(router(&world), post(writer)).await;

    assert_eq!(allowed, StatusCode::OK);
    let recorded = world.audit.snapshot();
    assert_eq!(recorded.len(), 1);
    assert_eq!(recorded[0].action.as_str(), "release.gate_changed");
}

#[tokio::test]
async fn a_campaign_outside_its_window_is_not_served() {
    let world = World::default();
    let writer = operator(&world, vec![Permission::ReleaseWrite]);
    let past = json!({
        "title": "Last winter",
        "body": "This one is over",
        "platform": "android",
        "starts_at_ms": 1,
        "ends_at_ms": 2,
        "published": true,
    });
    let request = Request::builder()
        .method("POST")
        .uri("/v1/admin/campaigns")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer access:{writer}"))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::from(past.to_string()))
        .expect("the request builds");
    send(router(&world), request).await;

    let read = Request::builder()
        .uri("/v1/campaigns?platform=android&build=42")
        .header("authorization", format!("Bearer access:{}", Uuid::new_v4()))
        .header("x-device-id", Uuid::new_v4().to_string())
        .body(Body::empty())
        .expect("the request builds");
    let (status, body) = send(router(&world), read).await;

    assert_eq!(status, StatusCode::OK);
    assert!(
        body["data"].as_array().expect("an array").is_empty(),
        "a campaign whose window has passed must not be served"
    );
    assert_eq!(world.campaigns.snapshot().len(), 1);
}
