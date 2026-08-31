use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::application::CheckReadiness;

use super::dto::ReadinessResponse;
use super::envelope::Envelope;
use super::error::ApiError;
use super::state::AppState;

pub async fn liveness() -> Response {
    (StatusCode::OK, Json(Envelope::new("Service is live", ()))).into_response()
}

pub async fn readiness(State(state): State<AppState>) -> Response {
    match CheckReadiness::new(state.readiness).execute().await {
        Ok(report) => (
            StatusCode::OK,
            Json(Envelope::new(
                "Service is ready",
                ReadinessResponse::from(report),
            )),
        )
            .into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}
