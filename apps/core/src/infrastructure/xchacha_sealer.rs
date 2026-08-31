use crate::domain::crypto::{ContentKey, open_blob, seal_blob};
use crate::domain::{ContentSealer, CoreError};

/// The [`ContentSealer`] every blob crosses once a vault is unlocked. Holds the content key, seals
/// with XChaCha20-Poly1305 and binds the caller's label into the associated data, so a file moved
/// to another photo's name fails to open.
pub struct XChaChaSealer {
    key: ContentKey,
}

impl XChaChaSealer {
    pub const fn new(key: ContentKey) -> Self {
        Self { key }
    }
}

impl ContentSealer for XChaChaSealer {
    fn seal(&self, label: &str, plain: &[u8]) -> Result<Vec<u8>, CoreError> {
        Ok(seal_blob(&self.key, label, plain)?)
    }

    fn open(&self, label: &str, sealed: &[u8]) -> Result<Vec<u8>, CoreError> {
        Ok(open_blob(&self.key, label, sealed)?)
    }
}
