pub const ERR_UNKNOWN_PERMISSION: &str = "Stored permission is not a known value";
pub const ERR_UNKNOWN_ACTION: &str = "Stored audit action is not a known value";

#[derive(Debug, thiserror::Error)]
pub enum RbacError {
    #[error("The caller lacks the required permission")]
    Forbidden,
    #[error("Role not found")]
    RoleNotFound,
    #[error("Storage failure: {0}")]
    Storage(String),
}
