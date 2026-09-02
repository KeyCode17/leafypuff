pub mod dto;
pub mod handlers;

use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::get;

use crate::domain::media::MAX_OBJECT_BYTES;
use crate::http::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/profile",
            get(handlers::read_profile).put(handlers::save_profile),
        )
        .route(
            "/profile/avatar/{variant}",
            get(handlers::get_avatar).put(handlers::put_avatar),
        )
        .layer(DefaultBodyLimit::max(MAX_OBJECT_BYTES))
}
