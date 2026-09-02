pub mod dto;
pub mod handlers;
pub mod validation;

use axum::Router;
use axum::routing::post;

use crate::http::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/verify-email", post(handlers::verify_email))
        .route("/sign-in", post(handlers::sign_in))
        .route("/sign-in/verify", post(handlers::complete_sign_in))
        .route("/password/forgot", post(handlers::forgot_password))
        .route("/password/reset", post(handlers::reset_password))
        .route("/email/change", post(handlers::change_email))
        .route("/email/confirm", post(handlers::confirm_email))
        .route("/refresh", post(handlers::refresh))
}
