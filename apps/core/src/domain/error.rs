#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Entry not found")]
    NotFound,
    #[error("Storage failure: {0}")]
    Storage(String),
    #[error("Invalid entry: {0}")]
    Invalid(String),
}
