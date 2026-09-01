use axum::routing::get;
use axum::{Router, middleware};

use super::rate_limit::{RateLimiter, guard};
use super::state::AppState;
use super::{health, iam, media, rbac, sync};

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
        .nest("/v1/sync", sync::router())
        .nest("/v1/media", media::router())
        .nest("/v1/admin", rbac::router())
        .with_state(state)
}
