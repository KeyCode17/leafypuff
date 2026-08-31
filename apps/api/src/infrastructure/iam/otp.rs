use rand::TryRng;
use rand::rngs::SysRng;

use crate::domain::iam::error::ERR_ENTROPY_UNAVAILABLE;
use crate::domain::iam::{IamError, OtpGenerator};

const CODE_SPACE: u64 = 1_000_000;
const DRAW_LIMIT: u64 = u64::MAX - (u64::MAX % CODE_SPACE);

pub struct Blake3Otp {
    pepper: [u8; 32],
}

impl Blake3Otp {
    pub const fn new(pepper: [u8; 32]) -> Self {
        Self { pepper }
    }
}

impl OtpGenerator for Blake3Otp {
    fn code(&self) -> Result<String, IamError> {
        loop {
            let draw = SysRng
                .try_next_u64()
                .map_err(|_| IamError::Storage(ERR_ENTROPY_UNAVAILABLE.to_owned()))?;
            if draw < DRAW_LIMIT {
                return Ok(format!("{:06}", draw % CODE_SPACE));
            }
        }
    }

    fn digest(&self, code: &str) -> String {
        blake3::keyed_hash(&self.pepper, code.as_bytes())
            .to_hex()
            .to_string()
    }
}
