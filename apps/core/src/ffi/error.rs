use crate::domain::CoreError;

/// The error every exported method returns. Kotlin receives it as a sealed exception
/// hierarchy, one subclass per variant, never as a string.
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
        }
    }
}
