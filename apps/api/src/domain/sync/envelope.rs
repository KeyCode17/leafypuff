use std::fmt;

use uuid::Uuid;

#[derive(Clone, PartialEq, Eq)]
pub struct FieldEnvelope {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub updated_at_ms: i64,
    pub device_id: Uuid,
}

impl fmt::Debug for FieldEnvelope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FieldEnvelope")
            .field("ciphertext_len", &self.ciphertext.len())
            .field("nonce_len", &self.nonce.len())
            .field("updated_at_ms", &self.updated_at_ms)
            .field("device_id", &self.device_id)
            .finish()
    }
}
