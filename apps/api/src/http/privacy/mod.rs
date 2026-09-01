pub mod dto;
pub mod handlers;
pub mod validation;

use axum::Router;
use axum::routing::post;

use crate::http::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/data-requests", post(handlers::raise_request))
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/data-requests", axum::routing::get(handlers::list_open))
        .route(
            "/data-requests/{request_id}/fulfil",
            post(handlers::fulfil_request),
        )
}
