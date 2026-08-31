use uuid::Uuid;

use super::variant::Variant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaObject {
    pub photo_id: Uuid,
    pub account_id: Uuid,
    pub entry_id: Uuid,
    pub variant: Variant,
    pub byte_len: i64,
    pub ciphertext_hash: String,
    pub created_at_ms: i64,
}
