use argon2::{Algorithm, Argon2, Params, PasswordHasher as _, PasswordVerifier as _, Version};

use crate::domain::iam::error::{ERR_ARGON_PARAMS_REJECTED, ERR_PASSWORD_HASHING_FAILED};
use crate::domain::iam::{IamError, PasswordHasher};

const MEMORY_KIB: u32 = 19 * 1024;
const ITERATIONS: u32 = 2;
const PARALLELISM: u32 = 1;
const DECOY_PASSWORD: &str = "leafypuff-timing-parity-decoy";

pub struct Argon2Hasher {
    decoy: String,
}

impl Argon2Hasher {
    pub fn new() -> Result<Self, IamError> {
        Ok(Self {
            decoy: derive(DECOY_PASSWORD)?,
        })
    }
}

impl PasswordHasher for Argon2Hasher {
    fn hash(&self, plain: &str) -> Result<String, IamError> {
        derive(plain)
    }

    fn verify(&self, plain: &str, hash: &str) -> bool {
        match engine() {
            Ok(argon) => argon.verify_password(plain.as_bytes(), hash).is_ok(),
            Err(_) => false,
        }
    }

    fn decoy_verify(&self, plain: &str) {
        let _ = self.verify(plain, &self.decoy);
    }
}

fn engine() -> Result<Argon2<'static>, IamError> {
    let params = Params::new(MEMORY_KIB, ITERATIONS, PARALLELISM, None)
        .map_err(|_| IamError::Storage(ERR_ARGON_PARAMS_REJECTED.to_owned()))?;
    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

fn derive(plain: &str) -> Result<String, IamError> {
    engine()?
        .hash_password(plain.as_bytes())
        .map(|hashed| hashed.to_string())
        .map_err(|_| IamError::Storage(ERR_PASSWORD_HASHING_FAILED.to_owned()))
}
