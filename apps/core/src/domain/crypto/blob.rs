use super::error::CryptoError;
use super::keys::ContentKey;
use super::seal::{NONCE_LEN, open_bytes, seal_bytes};

pub const TAG_LEN: usize = 16;
const LABEL_DOMAIN: &[u8] = b"leafypuff:label:v1";

fn label_bytes(label: &str) -> Result<Vec<u8>, CryptoError> {
    let name = label.as_bytes();
    if name.is_empty() {
        return Err(CryptoError::Payload);
    }
    let mut bytes = Vec::with_capacity(LABEL_DOMAIN.len() + name.len());
    bytes.extend_from_slice(LABEL_DOMAIN);
    bytes.extend_from_slice(name);
    Ok(bytes)
}

/// Seals bytes that no entry owns yet under an opaque label, returning the nonce and the ciphertext
/// as one blob. Photos are imported before an entry exists, so they cannot bind an entry id.
pub fn seal_blob(key: &ContentKey, label: &str, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let sealed = seal_bytes(key, &label_bytes(label)?, plaintext)?;
    let mut blob = Vec::with_capacity(NONCE_LEN + sealed.ciphertext.len());
    blob.extend_from_slice(&sealed.nonce);
    blob.extend_from_slice(&sealed.ciphertext);
    Ok(blob)
}

/// Opens a blob `seal_blob` produced. A blob shorter than a nonce and a tag is refused before any
/// slicing, so a truncated file on disk fails closed instead of panicking.
pub fn open_blob(key: &ContentKey, label: &str, blob: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let (head, ciphertext) = blob
        .split_at_checked(NONCE_LEN)
        .ok_or(CryptoError::Payload)?;
    if ciphertext.len() < TAG_LEN {
        return Err(CryptoError::Payload);
    }
    let nonce = <[u8; NONCE_LEN]>::try_from(head).map_err(|_| CryptoError::Payload)?;
    open_bytes(key, &label_bytes(label)?, &nonce, ciphertext)
}
