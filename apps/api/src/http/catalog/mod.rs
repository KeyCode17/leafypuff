pub mod dto;
pub mod handlers;
pub mod validation;

use axum::Router;
use axum::routing::{get, post};

use crate::http::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/catalog", get(handlers::read_published))
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/catalog", get(handlers::list_bundles))
        .route("/catalog", post(handlers::draft_bundle))
        .route(
            "/catalog/{bundle_id}/publish",
            post(handlers::publish_bundle),
        )
}
