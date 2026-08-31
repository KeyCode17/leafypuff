use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::domain::DomainError;

pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    pub const fn new(status: StatusCode, message: String) -> Self {
        Self { status, message }
    }
}

impl From<DomainError> for ApiError {
    fn from(error: DomainError) -> Self {
        let status = match error {
            DomainError::DependencyUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        };
        Self::new(status, error.to_string())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({
            "message": self.message,
            "data": serde_json::Value::Null,
            "version": env!("CARGO_PKG_VERSION"),
        });
        (self.status, Json(body)).into_response()
    }
}
