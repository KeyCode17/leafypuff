use crate::domain::crypto::{ContentKey, open_blob, seal_blob};
use crate::domain::{ContentSealer, CoreError};

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
