use core::fmt;

use rand::TryRng;
use rand::rngs::SysRng;
use zeroize::{Zeroize, ZeroizeOnDrop};

use super::error::CryptoError;

pub const KEY_LEN: usize = 32;

pub(crate) fn random_bytes<const N: usize>() -> Result<[u8; N], CryptoError> {
    let mut bytes = [0u8; N];
    SysRng
        .try_fill_bytes(&mut bytes)
        .map_err(|_| CryptoError::Entropy)?;
    Ok(bytes)
}

/// The 256-bit key every entry field is sealed under. Zeroized on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct ContentKey {
    bytes: [u8; KEY_LEN],
}

#[allow(dead_code)]
impl ContentKey {
    /// Draws a fresh content key from the operating system entropy source.
    pub fn generate() -> Result<Self, CryptoError> {
        Ok(Self {
            bytes: random_bytes()?,
        })
    }

    pub(crate) const fn from_bytes(bytes: [u8; KEY_LEN]) -> Self {
        Self { bytes }
    }

    pub(crate) const fn as_bytes(&self) -> &[u8; KEY_LEN] {
        &self.bytes
    }
}

impl fmt::Debug for ContentKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ContentKey(redacted)")
    }
}

/// The key an argon2id pass over the passphrase produces. Wraps the content key. Zeroized on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct MasterKey {
    bytes: [u8; KEY_LEN],
}

#[allow(dead_code)]
impl MasterKey {
    pub(crate) const fn from_bytes(bytes: [u8; KEY_LEN]) -> Self {
        Self { bytes }
    }

    pub(crate) const fn as_bytes(&self) -> &[u8; KEY_LEN] {
        &self.bytes
    }
}

impl fmt::Debug for MasterKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("MasterKey(redacted)")
    }
}

/// The key an HKDF pass over the recovery code produces. Wraps the same content key. Zeroized on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct RecoveryKey {
    bytes: [u8; KEY_LEN],
}

#[allow(dead_code)]
impl RecoveryKey {
    pub(crate) const fn from_bytes(bytes: [u8; KEY_LEN]) -> Self {
        Self { bytes }
    }

    pub(crate) const fn as_bytes(&self) -> &[u8; KEY_LEN] {
        &self.bytes
    }
}

impl fmt::Debug for RecoveryKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RecoveryKey(redacted)")
    }
}

#[cfg(test)]
mod tests {
    use super::{ContentKey, KEY_LEN, MasterKey, RecoveryKey};
    use zeroize::{Zeroize, ZeroizeOnDrop};

    #[test]
    fn zeroizing_a_content_key_clears_every_byte() {
        let mut key = ContentKey::from_bytes([0x42; KEY_LEN]);
        key.zeroize();
        assert_eq!(key.as_bytes(), &[0u8; KEY_LEN]);
    }

    #[test]
    fn every_key_type_zeroizes_on_drop() {
        fn assert_zeroize_on_drop<T: ZeroizeOnDrop>() {}
        assert_zeroize_on_drop::<ContentKey>();
        assert_zeroize_on_drop::<MasterKey>();
        assert_zeroize_on_drop::<RecoveryKey>();
    }

    #[test]
    fn a_content_key_never_prints_its_bytes() {
        let key = ContentKey::from_bytes([0x42; KEY_LEN]);
        assert_eq!(format!("{key:?}"), "ContentKey(redacted)");
    }
}
