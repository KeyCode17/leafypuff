pub mod dto;
pub mod handlers;

use axum::Router;
use axum::routing::{get, post};

use crate::http::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/overview", get(handlers::read_overview))
        .route("/accounts", get(handlers::list_accounts))
        .route("/accounts/{account_id}", get(handlers::read_account))
        .route(
            "/accounts/{account_id}/suspend",
            post(handlers::suspend_account),
        )
        .route(
            "/accounts/{account_id}/restore",
            post(handlers::restore_account),
        )
}
