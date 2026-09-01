pub mod dto;
pub mod gate_layer;
pub mod handlers;
pub mod validation;

use axum::Router;
use axum::routing::{get, post};

use crate::http::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/release", get(handlers::read_gate))
        .route("/campaigns", get(handlers::read_campaigns))
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/release", get(handlers::list_gates))
        .route("/release", post(handlers::set_gate))
        .route("/campaigns", get(handlers::list_campaigns))
        .route("/campaigns", post(handlers::save_campaign))
}
