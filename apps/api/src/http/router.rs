use axum::routing::get;
use axum::{Router, middleware};

use super::rate_limit::{RateLimiter, guard};
use super::state::AppState;
use super::{admin, catalog, health, iam, media, privacy, profile, rbac, release, sync};

pub fn build_router(state: AppState) -> Router {
    let limiter = RateLimiter::new();
    let auth = iam::router().layer(middleware::from_fn(move |request, next| {
        let limiter = limiter.clone();
        async move { guard(limiter, request, next).await }
    }));

    Router::new()
        .route("/healthz", get(health::liveness))
        .route("/ready", get(health::readiness))
        .nest("/v1/auth", auth)
        .nest(
            "/v1/sync",
            sync::router().layer(middleware::from_fn_with_state(
                state.clone(),
                release::gate_layer::guard,
            )),
        )
        .nest("/v1", profile::router())
        .nest("/v1/media", media::router())
        .nest("/v1/admin", rbac::router())
        .nest("/v1/admin", admin::router())
        .nest("/v1/admin", catalog::admin_router())
        .nest("/v1", catalog::router())
        .nest("/v1/admin", privacy::admin_router())
        .nest("/v1", privacy::router())
        .nest("/v1/admin", release::admin_router())
        .nest("/v1", release::router())
        .with_state(state)
}
