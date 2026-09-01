pub const ERR_UNKNOWN_KIND: &str = "Stored request kind is not a known value";
pub const ERR_UNKNOWN_STATUS: &str = "Stored request status is not a known value";

#[derive(Debug, thiserror::Error)]
pub enum PrivacyError {
    #[error("The caller lacks the required permission")]
    Forbidden,
    #[error("Request not found")]
    NotFound,
    #[error("Request was already fulfilled")]
    AlreadyFulfilled,
    #[error("Storage failure: {0}")]
    Storage(String),
}
