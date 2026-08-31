pub const ERR_UNKNOWN_MOOD: &str = "Stored mood is not a known variant";
pub const ERR_UNKNOWN_KEY_KIND: &str = "Stored wrapped key kind is not a known variant";
pub const ERR_TAGS_UNREADABLE: &str = "Stored tags are not a json array of strings";

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("Entry does not belong to this account")]
    Forbidden,
    #[error("Batch is malformed: {0}")]
    Malformed(String),
    #[error("Storage failure: {0}")]
    Storage(String),
}
