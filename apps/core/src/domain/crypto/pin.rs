use argon2::{Algorithm, Argon2, Params, PasswordHasher as _, PasswordVerifier as _, Version};

use super::error::CryptoError;

const MEMORY_KIB: u32 = 19 * 1024;
const ITERATIONS: u32 = 2;
const PARALLELISM: u32 = 1;

/// Hashes a screen-lock PIN into a PHC string. The digits are never stored, and the cost
/// parameters travel with the hash so raising them later does not invalidate what is on disk.
pub fn hash_pin(pin: &str) -> Result<String, CryptoError> {
    engine()?
        .hash_password(pin.as_bytes())
        .map(|hashed| hashed.to_string())
        .map_err(|_| CryptoError::Encryption)
}

/// Constant-time on the digest, and false on any malformed stored value rather than an error the
/// caller could branch on to learn whether a PIN was ever set.
pub fn verify_pin(pin: &str, stored: &str) -> bool {
    engine().is_ok_and(|argon| argon.verify_password(pin.as_bytes(), stored).is_ok())
}

fn engine() -> Result<Argon2<'static>, CryptoError> {
    let params = Params::new(MEMORY_KIB, ITERATIONS, PARALLELISM, None)
        .map_err(|_| CryptoError::Encryption)?;
    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}
