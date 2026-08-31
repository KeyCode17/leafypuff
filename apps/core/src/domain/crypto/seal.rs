use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{XChaCha20Poly1305, XNonce};
use zeroize::Zeroize;

use super::aad::FieldContext;
use super::error::CryptoError;
use super::keys::{ContentKey, random_bytes};
use super::padding;

pub const NONCE_LEN: usize = 24;

/// One sealed field: the nonce and the ciphertext, stored as the two plaintext columns the sync
/// contract names. The true plaintext length lives inside the ciphertext, never beside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SealedField {
    pub nonce: [u8; NONCE_LEN],
    pub ciphertext: Vec<u8>,
}

/// Pads the plaintext to a 256-byte bucket and seals it with XChaCha20-Poly1305 under a fresh nonce.
pub fn seal(
    key: &ContentKey,
    context: &FieldContext<'_>,
    plaintext: &[u8],
) -> Result<SealedField, CryptoError> {
    let associated = context.to_bytes()?;
    let mut padded = padding::pad(plaintext)?;
    let nonce: [u8; NONCE_LEN] = random_bytes()?;
    let cipher = XChaCha20Poly1305::new(key.as_bytes().into());
    let sealed = cipher.encrypt(
        &XNonce::from(nonce),
        Payload {
            msg: &padded,
            aad: &associated,
        },
    );
    padded.zeroize();
    Ok(SealedField {
        nonce,
        ciphertext: sealed.map_err(|_| CryptoError::Encryption)?,
    })
}

/// Opens a sealed field and strips the padding, returning exactly the bytes `seal` was given.
pub fn open(
    key: &ContentKey,
    context: &FieldContext<'_>,
    sealed: &SealedField,
) -> Result<Vec<u8>, CryptoError> {
    let associated = context.to_bytes()?;
    let cipher = XChaCha20Poly1305::new(key.as_bytes().into());
    let mut padded = cipher
        .decrypt(
            &XNonce::from(sealed.nonce),
            Payload {
                msg: sealed.ciphertext.as_slice(),
                aad: &associated,
            },
        )
        .map_err(|_| CryptoError::Decryption)?;
    let plaintext = padding::unpad(&padded);
    padded.zeroize();
    plaintext
}
