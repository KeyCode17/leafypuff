use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use zeroize::Zeroize;

use super::error::CryptoError;
use super::keys::{ContentKey, KEY_LEN, random_bytes};
use super::passphrase::{SALT_LEN, derive_master_key, generate_salt};
use super::recovery::RecoveryCode;
use super::seal::NONCE_LEN;

const PASSPHRASE_SLOT: &[u8] = b"leafypuff:wrap:passphrase:v1";
const RECOVERY_SLOT: &[u8] = b"leafypuff:wrap:recovery:v1";
const DEVICE_SLOT: &[u8] = b"leafypuff:wrap:device:v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WrappedKey {
    pub nonce: [u8; NONCE_LEN],
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyVault {
    pub passphrase_salt: [u8; SALT_LEN],
    pub passphrase_slot: WrappedKey,
    pub recovery_slot: WrappedKey,
}

fn wrap_content_key(
    wrapping: &[u8; KEY_LEN],
    slot: &[u8],
    content: &ContentKey,
) -> Result<WrappedKey, CryptoError> {
    let nonce: [u8; NONCE_LEN] = random_bytes()?;
    let cipher = XChaCha20Poly1305::new(wrapping.into());
    let ciphertext = cipher
        .encrypt(
            &XNonce::from(nonce),
            Payload {
                msg: content.as_bytes(),
                aad: slot,
            },
        )
        .map_err(|_| CryptoError::Encryption)?;
    Ok(WrappedKey { nonce, ciphertext })
}

fn open_slot(
    wrapping: &[u8; KEY_LEN],
    slot: &[u8],
    wrapped: &WrappedKey,
) -> Result<ContentKey, CryptoError> {
    let cipher = XChaCha20Poly1305::new(wrapping.into());
    let mut plain = cipher
        .decrypt(
            &XNonce::from(wrapped.nonce),
            Payload {
                msg: wrapped.ciphertext.as_slice(),
                aad: slot,
            },
        )
        .map_err(|_| CryptoError::Decryption)?;
    let mut bytes =
        <[u8; KEY_LEN]>::try_from(plain.as_slice()).map_err(|_| CryptoError::Payload)?;
    plain.zeroize();
    let key = ContentKey::from_bytes(bytes);
    bytes.zeroize();
    Ok(key)
}

impl KeyVault {
    pub fn create(
        passphrase: &str,
        code: &RecoveryCode,
    ) -> Result<(Self, ContentKey), CryptoError> {
        let content = ContentKey::generate()?;
        let passphrase_salt = generate_salt()?;
        let master = derive_master_key(passphrase, &passphrase_salt)?;
        let recovery = code.recovery_key()?;
        Ok((
            Self {
                passphrase_salt,
                passphrase_slot: wrap_content_key(master.as_bytes(), PASSPHRASE_SLOT, &content)?,
                recovery_slot: wrap_content_key(recovery.as_bytes(), RECOVERY_SLOT, &content)?,
            },
            content,
        ))
    }

    pub fn unlock_with_passphrase(&self, passphrase: &str) -> Result<ContentKey, CryptoError> {
        let master = derive_master_key(passphrase, &self.passphrase_salt)?;
        open_slot(master.as_bytes(), PASSPHRASE_SLOT, &self.passphrase_slot)
    }

    pub fn unlock_with_recovery_code(
        &self,
        code: &RecoveryCode,
    ) -> Result<ContentKey, CryptoError> {
        let recovery = code.recovery_key()?;
        open_slot(recovery.as_bytes(), RECOVERY_SLOT, &self.recovery_slot)
    }

    pub fn rewrap_passphrase(&self, current: &str, replacement: &str) -> Result<Self, CryptoError> {
        let content = self.unlock_with_passphrase(current)?;
        let passphrase_salt = generate_salt()?;
        let master = derive_master_key(replacement, &passphrase_salt)?;
        Ok(Self {
            passphrase_salt,
            passphrase_slot: wrap_content_key(master.as_bytes(), PASSPHRASE_SLOT, &content)?,
            recovery_slot: self.recovery_slot.clone(),
        })
    }
}

pub fn seal_for_device(
    device_key: &[u8; KEY_LEN],
    content: &ContentKey,
) -> Result<WrappedKey, CryptoError> {
    wrap_content_key(device_key, DEVICE_SLOT, content)
}

pub fn open_for_device(
    device_key: &[u8; KEY_LEN],
    wrapped: &WrappedKey,
) -> Result<ContentKey, CryptoError> {
    open_slot(device_key, DEVICE_SLOT, wrapped)
}
