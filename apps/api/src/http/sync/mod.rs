pub mod dto;
pub mod handlers;
pub mod mapping;
pub mod validation;

use axum::Router;
use axum::routing::{get, post, put};

use crate::http::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/pull", get(handlers::pull))
        .route("/push", post(handlers::push))
        .route("/keys", get(handlers::read_keys))
        .route("/keys", put(handlers::put_key))
}
