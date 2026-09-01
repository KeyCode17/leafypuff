#[derive(Debug, thiserror::Error)]
pub enum AdminError {
    #[error("The caller lacks the required permission")]
    Forbidden,
    #[error("Account not found")]
    AccountNotFound,
    #[error("Storage failure: {0}")]
    Storage(String),
}
