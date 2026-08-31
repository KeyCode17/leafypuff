use super::error::CryptoError;

pub const BUCKET: usize = 256;
const LENGTH_PREFIX: usize = 4;

pub(crate) fn pad(plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let length = u32::try_from(plaintext.len()).map_err(|_| CryptoError::Payload)?;
    let occupied = LENGTH_PREFIX
        .checked_add(plaintext.len())
        .ok_or(CryptoError::Payload)?;
    let mut padded = vec![0u8; occupied.div_ceil(BUCKET) * BUCKET];
    padded[..LENGTH_PREFIX].copy_from_slice(&length.to_le_bytes());
    padded[LENGTH_PREFIX..occupied].copy_from_slice(plaintext);
    Ok(padded)
}

pub(crate) fn unpad(padded: &[u8]) -> Result<Vec<u8>, CryptoError> {
    if padded.len() < BUCKET || !padded.len().is_multiple_of(BUCKET) {
        return Err(CryptoError::Payload);
    }
    let mut header = [0u8; LENGTH_PREFIX];
    header.copy_from_slice(&padded[..LENGTH_PREFIX]);
    let length = usize::try_from(u32::from_le_bytes(header)).map_err(|_| CryptoError::Payload)?;
    let end = LENGTH_PREFIX
        .checked_add(length)
        .ok_or(CryptoError::Payload)?;
    if end > padded.len() {
        return Err(CryptoError::Payload);
    }
    Ok(padded[LENGTH_PREFIX..end].to_vec())
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{BUCKET, pad, unpad};

    #[test]
    fn short_and_medium_payloads_land_in_the_same_bucket() {
        let short = pad(&[1u8; 10]).expect("padding a short payload");
        let medium = pad(&[1u8; 200]).expect("padding a medium payload");
        let empty = pad(b"").expect("padding an empty payload");
        assert_eq!(short.len(), BUCKET);
        assert_eq!(medium.len(), BUCKET);
        assert_eq!(empty.len(), BUCKET);
    }

    #[test]
    fn a_longer_payload_moves_to_the_next_bucket() {
        assert_eq!(pad(&[1u8; 300]).expect("padding").len(), BUCKET * 2);
        assert_eq!(pad(&[1u8; 252]).expect("padding").len(), BUCKET);
        assert_eq!(pad(&[1u8; 253]).expect("padding").len(), BUCKET * 2);
    }

    #[test]
    fn padding_round_trips() {
        let original = b"a body that is not a multiple of anything";
        let restored = unpad(&pad(original).expect("padding")).expect("unpadding");
        assert_eq!(restored.as_slice(), original.as_slice());
    }

    #[test]
    fn a_truncated_buffer_is_rejected() {
        assert!(unpad(&[0u8; 100]).is_err());
        assert!(unpad(&[0u8; 257]).is_err());
    }
}
