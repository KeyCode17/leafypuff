pub const ERR_UNKNOWN_VARIANT: &str = "Stored variant is not a known value";

#[derive(Debug, thiserror::Error)]
pub enum MediaError {
    #[error("Object not found")]
    NotFound,
    #[error("Object is larger than the ceiling")]
    TooLarge,
    #[error("Object storage failure: {0}")]
    Storage(String),
}
