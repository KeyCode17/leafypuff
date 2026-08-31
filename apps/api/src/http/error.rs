use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::domain::DomainError;
use crate::domain::iam::IamError;
use crate::domain::media::MediaError;
use crate::domain::sync::SyncError;

use super::envelope::Envelope;

pub const ERR_DEPENDENCY_UNAVAILABLE: &str = "DEPENDENCY_UNAVAILABLE";
pub const ERR_INVALID_CREDENTIALS: &str = "INVALID_CREDENTIALS";
pub const ERR_EMAIL_NOT_VERIFIED: &str = "EMAIL_NOT_VERIFIED";
pub const ERR_EMAIL_TAKEN: &str = "EMAIL_ALREADY_REGISTERED";
pub const ERR_TOO_MANY_ATTEMPTS: &str = "TOO_MANY_ATTEMPTS";
pub const ERR_MAIL_UNAVAILABLE: &str = "MAIL_UNAVAILABLE";
pub const ERR_INTERNAL: &str = "INTERNAL";
pub const ERR_FORBIDDEN: &str = "FORBIDDEN";
pub const ERR_MALFORMED_BATCH: &str = "MALFORMED_BATCH";
pub const ERR_OBJECT_NOT_FOUND: &str = "OBJECT_NOT_FOUND";
pub const ERR_OBJECT_TOO_LARGE: &str = "OBJECT_TOO_LARGE";

const MESSAGE_FAILED: &str = "Request failed";
const DETAIL_INVALID_CREDENTIALS: &str = "Invalid credentials";
const DETAIL_EMAIL_NOT_VERIFIED: &str = "Email is not verified";
const DETAIL_EMAIL_TAKEN: &str = "Email already registered";
const DETAIL_TOO_MANY_ATTEMPTS: &str = "Too many attempts";
const DETAIL_MAIL_UNAVAILABLE: &str = "The mail provider is unavailable";
const DETAIL_INTERNAL: &str = "Something went wrong";
const DETAIL_FORBIDDEN: &str = "That entry belongs to another account";
const DETAIL_OBJECT_NOT_FOUND: &str = "No such object";
const DETAIL_OBJECT_TOO_LARGE: &str = "Object is larger than the ceiling";

pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    detail: String,
}

impl ApiError {
    pub fn new(status: StatusCode, code: &'static str, detail: &str) -> Self {
        Self {
            status,
            code,
            detail: detail.to_owned(),
        }
    }
}

impl From<DomainError> for ApiError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::DependencyUnavailable(detail) => Self::new(
                StatusCode::SERVICE_UNAVAILABLE,
                ERR_DEPENDENCY_UNAVAILABLE,
                &detail,
            ),
        }
    }
}

impl From<IamError> for ApiError {
    fn from(error: IamError) -> Self {
        match error {
            IamError::InvalidCredentials | IamError::InvalidCode | IamError::ChallengeUnusable => {
                Self::new(
                    StatusCode::UNAUTHORIZED,
                    ERR_INVALID_CREDENTIALS,
                    DETAIL_INVALID_CREDENTIALS,
                )
            }
            IamError::EmailNotVerified => Self::new(
                StatusCode::FORBIDDEN,
                ERR_EMAIL_NOT_VERIFIED,
                DETAIL_EMAIL_NOT_VERIFIED,
            ),
            IamError::TooManyAttempts => Self::new(
                StatusCode::TOO_MANY_REQUESTS,
                ERR_TOO_MANY_ATTEMPTS,
                DETAIL_TOO_MANY_ATTEMPTS,
            ),
            IamError::EmailAlreadyRegistered => {
                Self::new(StatusCode::CONFLICT, ERR_EMAIL_TAKEN, DETAIL_EMAIL_TAKEN)
            }
            IamError::Mail(reason) => {
                tracing::error!(%reason, "the mail provider refused the request");
                Self::new(
                    StatusCode::BAD_GATEWAY,
                    ERR_MAIL_UNAVAILABLE,
                    DETAIL_MAIL_UNAVAILABLE,
                )
            }
            IamError::Storage(reason) => {
                tracing::error!(%reason, "an iam request failed");
                Self::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ERR_INTERNAL,
                    DETAIL_INTERNAL,
                )
            }
        }
    }
}

impl From<SyncError> for ApiError {
    fn from(error: SyncError) -> Self {
        match error {
            SyncError::Forbidden => {
                Self::new(StatusCode::FORBIDDEN, ERR_FORBIDDEN, DETAIL_FORBIDDEN)
            }
            SyncError::Malformed(detail) => Self::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                ERR_MALFORMED_BATCH,
                &detail,
            ),
            SyncError::Storage(reason) => {
                tracing::error!(%reason, "a sync request failed");
                Self::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ERR_INTERNAL,
                    DETAIL_INTERNAL,
                )
            }
        }
    }
}

impl From<MediaError> for ApiError {
    fn from(error: MediaError) -> Self {
        match error {
            MediaError::NotFound => Self::new(
                StatusCode::NOT_FOUND,
                ERR_OBJECT_NOT_FOUND,
                DETAIL_OBJECT_NOT_FOUND,
            ),
            MediaError::TooLarge => Self::new(
                StatusCode::PAYLOAD_TOO_LARGE,
                ERR_OBJECT_TOO_LARGE,
                DETAIL_OBJECT_TOO_LARGE,
            ),
            MediaError::Storage(reason) => {
                tracing::error!(%reason, "an object storage request failed");
                Self::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ERR_INTERNAL,
                    DETAIL_INTERNAL,
                )
            }
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Envelope::failed(MESSAGE_FAILED, self.code, &self.detail);
        (self.status, Json(body)).into_response()
    }
}
