use crate::domain::{ContentSealer, CoreError};

/// The pass-through sealer the device runs on until the crypto slice lands.
///
/// Photo bytes and the cover thumbnail are meant to be encrypted at rest, and
/// [`ContentSealer`] is the single place they meet the filesystem. Swapping
/// this adapter for an XChaCha20-Poly1305 one is a constructor change; no call
/// site above it knows whether the blob it handed over was sealed.
#[derive(Debug, Default, Clone, Copy)]
pub struct PlaintextSealer;

impl ContentSealer for PlaintextSealer {
    fn seal(&self, _label: &str, plain: &[u8]) -> Result<Vec<u8>, CoreError> {
        Ok(plain.to_vec())
    }

    fn open(&self, _label: &str, sealed: &[u8]) -> Result<Vec<u8>, CoreError> {
        Ok(sealed.to_vec())
    }
}
