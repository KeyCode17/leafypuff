use uuid::Uuid;

use super::field::EncryptedField;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldConflict {
    pub entry_id: Uuid,
    pub field: EncryptedField,
    pub winner_updated_at_ms: i64,
    pub loser_updated_at_ms: i64,
    pub loser_device_id: Uuid,
    pub loser_ciphertext_hash: String,
    pub loser_byte_len: i64,
}
