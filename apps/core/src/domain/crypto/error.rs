#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Key derivation failed")]
    Derivation,
    #[error("Encryption failed")]
    Encryption,
    #[error("Decryption failed")]
    Decryption,
    #[error("Malformed recovery code")]
    RecoveryCode,
    #[error("Malformed payload")]
    Payload,
    #[error("Entropy source unavailable")]
    Entropy,
}
