use argon2::{Algorithm, Argon2, Params, Version};
use zeroize::Zeroize;

use super::error::CryptoError;
use super::keys::{KEY_LEN, MasterKey, random_bytes};

pub const SALT_LEN: usize = 16;
pub const MEMORY_KIB: u32 = 65_536;
pub const ITERATIONS: u32 = 3;
pub const PARALLELISM: u32 = 1;

/// Draws a fresh argon2id salt. Stored in plaintext beside the wrapped content key.
pub fn generate_salt() -> Result<[u8; SALT_LEN], CryptoError> {
    random_bytes()
}

/// Runs argon2id at m=64MiB, t=3, p=1 over the passphrase. Changing any parameter changes every
/// derived key, which is what the known-answer vector in this module exists to catch.
pub fn derive_master_key(
    passphrase: &str,
    salt: &[u8; SALT_LEN],
) -> Result<MasterKey, CryptoError> {
    let params = Params::new(MEMORY_KIB, ITERATIONS, PARALLELISM, Some(KEY_LEN))
        .map_err(|_| CryptoError::Derivation)?;
    let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut bytes = [0u8; KEY_LEN];
    argon
        .hash_password_into(passphrase.as_bytes(), salt, &mut bytes)
        .map_err(|_| CryptoError::Derivation)?;
    let key = MasterKey::from_bytes(bytes);
    bytes.zeroize();
    Ok(key)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{ITERATIONS, MEMORY_KIB, PARALLELISM, SALT_LEN, derive_master_key};

    const KAT_PASSPHRASE: &str = "correct horse battery staple";
    const KAT_SALT: [u8; SALT_LEN] = *b"leafypuffsalt000";
    const KAT_MASTER_KEY: [u8; 32] = [
        0x0b, 0xdf, 0xb2, 0x91, 0x69, 0xdf, 0x8d, 0xa2, 0xf7, 0x76, 0x5f, 0xdb, 0x78, 0x02, 0x6e,
        0x1c, 0xd1, 0x0f, 0x3a, 0x6d, 0xb3, 0xa7, 0x68, 0x6f, 0x28, 0xdd, 0x95, 0xd4, 0xb6, 0x45,
        0x89, 0xa0,
    ];

    #[test]
    fn the_parameters_are_the_ones_the_spec_names() {
        assert_eq!(MEMORY_KIB, 65_536);
        assert_eq!(ITERATIONS, 3);
        assert_eq!(PARALLELISM, 1);
    }

    #[test]
    fn a_fixed_passphrase_and_salt_derive_a_fixed_master_key() {
        let key = derive_master_key(KAT_PASSPHRASE, &KAT_SALT).expect("derivation must succeed");
        assert_eq!(key.as_bytes(), &KAT_MASTER_KEY);
    }

    #[test]
    fn a_different_passphrase_derives_a_different_master_key() {
        let right = derive_master_key(KAT_PASSPHRASE, &KAT_SALT).expect("derivation");
        let wrong =
            derive_master_key("correct horse battery stapl", &KAT_SALT).expect("derivation");
        assert_ne!(right.as_bytes(), wrong.as_bytes());
    }

    #[test]
    fn a_different_salt_derives_a_different_master_key() {
        let left = derive_master_key(KAT_PASSPHRASE, &KAT_SALT).expect("derivation");
        let right = derive_master_key(KAT_PASSPHRASE, b"leafypuffsalt001").expect("derivation");
        assert_ne!(left.as_bytes(), right.as_bytes());
    }
}
