pub mod dto;
pub mod handlers;
pub mod validation;

use axum::Router;
use axum::routing::{get, post};

use crate::http::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/roles", get(handlers::list_roles))
        .route("/roles/assign", post(handlers::assign_role))
        .route("/roles/revoke", post(handlers::revoke_role))
        .route("/permissions", get(handlers::granted))
        .route("/audit", get(handlers::list_audit))
}
