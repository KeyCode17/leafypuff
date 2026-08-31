use crate::domain::{ContentSealer, CoreError};

/// The pass-through sealer the device runs on until a vault is unlocked.
///
/// Photo bytes and the cover thumbnail are encrypted at rest by
/// [`super::XChaChaSealer`], which this is swapped for in the constructor once a
/// content key exists; no call site above it knows whether the blob it handed
/// over was sealed.
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
