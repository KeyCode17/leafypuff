use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::domain::DomainError;
use crate::domain::iam::IamError;

pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: &str) -> Self {
        Self {
            status,
            message: message.to_owned(),
        }
    }
}

impl From<DomainError> for ApiError {
    fn from(error: DomainError) -> Self {
        let status = match error {
            DomainError::DependencyUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
        };
        Self::new(status, &error.to_string())
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

const ERR_INVALID_CREDENTIALS: &str = "Invalid credentials";
const ERR_EMAIL_NOT_VERIFIED: &str = "Email is not verified";
const ERR_TOO_MANY_ATTEMPTS: &str = "Too many attempts";
const ERR_MAIL_UNAVAILABLE: &str = "The mail provider is unavailable";
const ERR_EMAIL_TAKEN: &str = "Email already registered";
const ERR_INTERNAL: &str = "Something went wrong";

impl From<IamError> for ApiError {
    fn from(error: IamError) -> Self {
        match error {
            IamError::InvalidCredentials | IamError::InvalidCode | IamError::ChallengeUnusable => {
                Self::new(StatusCode::UNAUTHORIZED, ERR_INVALID_CREDENTIALS)
            }
            IamError::EmailNotVerified => Self::new(StatusCode::FORBIDDEN, ERR_EMAIL_NOT_VERIFIED),
            IamError::TooManyAttempts => {
                Self::new(StatusCode::TOO_MANY_REQUESTS, ERR_TOO_MANY_ATTEMPTS)
            }
            IamError::Mail(reason) => {
                tracing::error!(%reason, "the mail provider refused the request");
                Self::new(StatusCode::BAD_GATEWAY, ERR_MAIL_UNAVAILABLE)
            }
            IamError::EmailAlreadyRegistered => Self::new(StatusCode::CONFLICT, ERR_EMAIL_TAKEN),
            IamError::Storage(reason) => {
                tracing::error!(%reason, "an iam request failed");
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, ERR_INTERNAL)
            }
        }
    }
}
