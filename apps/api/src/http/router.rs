use axum::routing::get;
use axum::{Extension, Router};

use super::health;
use super::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health::liveness))
        .route("/ready", get(health::readiness))
        .layer(Extension(state))
}
