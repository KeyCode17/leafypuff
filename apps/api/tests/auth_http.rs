use api_testing::World;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use leafypuff_api::http::rate_limit::MAX_REQUESTS_PER_WINDOW;
use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::DependencyProbe;
use serde_json::{Value, json};
use tower::ServiceExt;

const BODY_LIMIT: usize = 64 * 1024;
const EMAIL: &str = "person@example.test";
const PASSWORD: &str = "correct horse battery";

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

async fn post(app: Router, path: &str, body: Value) -> (StatusCode, String) {
    let request = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("the request builds");
    let response = app.oneshot(request).await.expect("the router answers");
    let status = response.status();
    let bytes = to_bytes(response.into_body(), BODY_LIMIT)
        .await
        .expect("the body reads");
    (
        status,
        String::from_utf8(bytes.to_vec()).expect("the body is utf-8"),
    )
}

fn registration() -> Value {
    json!({ "email": EMAIL, "password": PASSWORD })
}

#[tokio::test]
async fn a_duplicate_registration_is_indistinguishable_from_a_new_one() {
    let world = World::default();
    world.generator.queue("123456");
    world.generator.queue("654321");

    let (first_status, first_body) =
        post(router(&world), "/v1/auth/register", registration()).await;
    let (second_status, second_body) =
        post(router(&world), "/v1/auth/register", registration()).await;

    assert_eq!(first_status, StatusCode::ACCEPTED);
    assert_eq!(second_status, StatusCode::ACCEPTED);
    assert_eq!(first_body, second_body);
    assert_eq!(world.accounts.snapshot().len(), 1);
    assert_eq!(world.mailer.sent().len(), 2);
}

#[tokio::test]
async fn registering_a_verified_address_answers_the_same_bytes_and_mails_nothing() {
    let world = World::default();
    world.generator.queue("123456");
    let (_, fresh_body) = post(router(&world), "/v1/auth/register", registration()).await;
    post(
        router(&world),
        "/v1/auth/verify-email",
        json!({ "email": EMAIL, "code": "123456" }),
    )
    .await;
    let mailed = world.mailer.sent().len();

    let (status, body) = post(router(&world), "/v1/auth/register", registration()).await;

    assert_eq!(status, StatusCode::ACCEPTED);
    assert_eq!(body, fresh_body);
    assert_eq!(world.mailer.sent().len(), mailed);
}

#[tokio::test]
async fn an_unknown_address_and_a_wrong_password_answer_the_same_bytes() {
    let world = World::default();
    world.generator.queue("123456");
    post(router(&world), "/v1/auth/register", registration()).await;
    post(
        router(&world),
        "/v1/auth/verify-email",
        json!({ "email": EMAIL, "code": "123456" }),
    )
    .await;

    let (unknown_status, unknown_body) = post(
        router(&world),
        "/v1/auth/sign-in",
        json!({ "email": "nobody@example.test", "password": PASSWORD }),
    )
    .await;
    let (wrong_status, wrong_body) = post(
        router(&world),
        "/v1/auth/sign-in",
        json!({ "email": EMAIL, "password": "not the password" }),
    )
    .await;

    assert_eq!(unknown_status, StatusCode::UNAUTHORIZED);
    assert_eq!(wrong_status, StatusCode::UNAUTHORIZED);
    assert_eq!(unknown_body, wrong_body);
}

#[tokio::test]
async fn a_short_password_is_refused_before_the_use_case_runs() {
    let world = World::default();

    let (status, body) = post(
        router(&world),
        "/v1/auth/register",
        json!({ "email": EMAIL, "password": "hunter2" }),
    )
    .await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    assert!(!body.contains("hunter2"));
    assert!(world.accounts.snapshot().is_empty());
}

#[tokio::test]
async fn a_rejection_carries_the_envelope_and_nothing_else() {
    let world = World::default();

    let (_, body) = post(
        router(&world),
        "/v1/auth/register",
        json!({ "email": "not-an-address", "password": PASSWORD }),
    )
    .await;

    let parsed: Value = serde_json::from_str(&body).expect("the rejection is json");
    let mut keys: Vec<&String> = parsed
        .as_object()
        .expect("the rejection is an object")
        .keys()
        .collect();
    keys.sort();
    assert_eq!(keys, vec!["data", "error", "message", "success"]);
    assert_eq!(parsed["success"], serde_json::json!(false));
    assert_eq!(parsed["data"], Value::Null);
    assert_eq!(parsed["error"]["code"], "VALIDATION_FAILED");
}

#[tokio::test]
async fn an_unknown_field_is_refused() {
    let world = World::default();

    let (status, _) = post(
        router(&world),
        "/v1/auth/register",
        json!({ "email": EMAIL, "password": PASSWORD, "role": "admin" }),
    )
    .await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn a_body_that_is_not_json_is_refused_with_400() {
    let world = World::default();
    let request = Request::builder()
        .method("POST")
        .uri("/v1/auth/register")
        .header("content-type", "application/json")
        .body(Body::from("{"))
        .expect("the request builds");

    let response = router(&world)
        .oneshot(request)
        .await
        .expect("the router answers");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn the_window_closes_after_the_ceiling() {
    let world = World::default();
    let app = router(&world);

    for _ in 0..MAX_REQUESTS_PER_WINDOW {
        let (status, _) = post(
            app.clone(),
            "/v1/auth/sign-in",
            json!({ "email": EMAIL, "password": PASSWORD }),
        )
        .await;
        assert_ne!(status, StatusCode::TOO_MANY_REQUESTS);
    }

    let (status, _) = post(
        app,
        "/v1/auth/sign-in",
        json!({ "email": EMAIL, "password": PASSWORD }),
    )
    .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn the_health_probes_are_outside_the_rate_limited_window() {
    let world = World::default();
    let app = router(&world);

    for _ in 0..MAX_REQUESTS_PER_WINDOW + 1 {
        let request = Request::builder()
            .uri("/healthz")
            .body(Body::empty())
            .expect("the request builds");
        let response = app
            .clone()
            .oneshot(request)
            .await
            .expect("the router answers");
        assert_eq!(response.status(), StatusCode::OK);
    }
}
