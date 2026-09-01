pub const ERR_UNKNOWN_PLATFORM: &str = "Stored platform is not a known value";

#[derive(Debug, thiserror::Error)]
pub enum ReleaseError {
    #[error("The caller lacks the required permission")]
    Forbidden,
    #[error("No gate exists for that platform")]
    GateNotFound,
    #[error("Storage failure: {0}")]
    Storage(String),
}
