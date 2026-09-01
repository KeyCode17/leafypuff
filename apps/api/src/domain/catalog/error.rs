#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    #[error("The caller lacks the required permission")]
    Forbidden,
    #[error("Bundle not found")]
    NotFound,
    #[error("No catalog has been published")]
    NonePublished,
    #[error("Bundle payload is malformed: {0}")]
    Malformed(String),
    #[error("Storage failure: {0}")]
    Storage(String),
}
