use axum::http::StatusCode;

use crate::domain::DomainError;
use crate::domain::iam::IamError;
use crate::domain::media::MediaError;
use crate::domain::sync::SyncError;

use super::error::{
    ApiError, DETAIL_EMAIL_NOT_VERIFIED, DETAIL_EMAIL_TAKEN, DETAIL_FORBIDDEN, DETAIL_INTERNAL,
    DETAIL_INVALID_CREDENTIALS, DETAIL_MAIL_UNAVAILABLE, DETAIL_OBJECT_NOT_FOUND,
    DETAIL_OBJECT_TOO_LARGE, DETAIL_TOO_MANY_ATTEMPTS, ERR_DEPENDENCY_UNAVAILABLE,
    ERR_EMAIL_NOT_VERIFIED, ERR_EMAIL_TAKEN, ERR_FORBIDDEN, ERR_INTERNAL, ERR_INVALID_CREDENTIALS,
    ERR_MAIL_UNAVAILABLE, ERR_MALFORMED_BATCH, ERR_OBJECT_NOT_FOUND, ERR_OBJECT_TOO_LARGE,
    ERR_TOO_MANY_ATTEMPTS,
};

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
                // 503, not 502. A CDN in front of this replaces a 502 body with its own error
                // page, so the envelope never reaches the device and MAIL_UNAVAILABLE arrives as
                // unparseable text. 503 passes through, and "a dependency is unavailable" is what
                // this is anyway -- the same status DependencyUnavailable already uses.
                Self::new(
                    StatusCode::SERVICE_UNAVAILABLE,
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
