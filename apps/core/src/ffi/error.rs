use crate::domain::{CoreError, Rejection};

#[derive(Debug, thiserror::Error, uniffi::Error)]
#[uniffi(flat_error)]
pub enum LeafyPuffCoreError {
    #[error("{message}")]
    NotFound { message: String },
    #[error("{message}")]
    Storage { message: String },
    #[error("{message}")]
    Photo { message: String },
    #[error("{message}")]
    Exif { message: String },
    #[error("{message}")]
    Crypto { message: String },
    #[error("{message}")]
    Invalid { message: String },
    #[error("{message}")]
    Locked { message: String },
    #[error("{message}")]
    InvalidCredentials { message: String },
    #[error("{message}")]
    EmailNotVerified { message: String },
    #[error("{message}")]
    EmailTaken { message: String },
    #[error("{message}")]
    TooManyAttempts { message: String },
    #[error("{message}")]
    MailUnavailable { message: String },
    #[error("{message}")]
    ServiceUnavailable { message: String },
    #[error("{message}")]
    Timeout { message: String },
    #[error("{message}")]
    Unreadable { message: String },
}

impl From<CoreError> for LeafyPuffCoreError {
    fn from(error: CoreError) -> Self {
        let message = error.to_string();
        match error {
            CoreError::NotFound { .. } => Self::NotFound { message },
            CoreError::Storage(_) => Self::Storage { message },
            CoreError::Photo(_) => Self::Photo { message },
            CoreError::Exif(_) => Self::Exif { message },
            CoreError::Crypto(_) => Self::Crypto { message },
            CoreError::Invalid(_) => Self::Invalid { message },
            CoreError::Locked => Self::Locked { message },
            CoreError::Timeout(_) => Self::Timeout { message },
            CoreError::Unreadable(_) => Self::Unreadable { message },
            CoreError::Rejected { rejection, .. } => match rejection {
                Rejection::InvalidCredentials => Self::InvalidCredentials { message },
                Rejection::EmailNotVerified => Self::EmailNotVerified { message },
                Rejection::EmailTaken => Self::EmailTaken { message },
                Rejection::TooManyAttempts => Self::TooManyAttempts { message },
                Rejection::MailUnavailable => Self::MailUnavailable { message },
                Rejection::ServiceUnavailable => Self::ServiceUnavailable { message },
                Rejection::Unknown => Self::Invalid { message },
            },
        }
    }
}
