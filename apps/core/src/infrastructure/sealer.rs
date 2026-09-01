use crate::domain::{ContentSealer, CoreError};

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
