use core::fmt;

use data_encoding::BASE32_NOPAD;
use hkdf::Hkdf;
use sha2::Sha256;
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

use super::error::CryptoError;
use super::keys::{KEY_LEN, RecoveryKey, random_bytes};

pub const ENTROPY_LEN: usize = 16;
pub const CODE_CHARS: usize = 26;
const RECOVERY_INFO: &[u8] = b"leafypuff:recovery-key:v1";

/// 128 bits of entropy shown to the user once, as 26 base32 characters. Zeroized on drop.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct RecoveryCode {
    entropy: [u8; ENTROPY_LEN],
}

impl RecoveryCode {
    /// Draws a fresh recovery code from the operating system entropy source.
    pub fn generate() -> Result<Self, CryptoError> {
        Ok(Self {
            entropy: random_bytes()?,
        })
    }

    /// Reads a code the user typed, tolerating lowercase, spaces and dashes and nothing else.
    pub fn parse(text: &str) -> Result<Self, CryptoError> {
        let mut normalized: String = text
            .chars()
            .filter(|character| !character.is_ascii_whitespace() && *character != '-')
            .map(|character| character.to_ascii_uppercase())
            .collect();
        if normalized.len() != CODE_CHARS {
            normalized.zeroize();
            return Err(CryptoError::RecoveryCode);
        }
        let decoded = BASE32_NOPAD.decode(normalized.as_bytes());
        normalized.zeroize();
        let mut decoded = decoded.map_err(|_| CryptoError::RecoveryCode)?;
        let entropy = <[u8; ENTROPY_LEN]>::try_from(decoded.as_slice())
            .map_err(|_| CryptoError::RecoveryCode)?;
        decoded.zeroize();
        Ok(Self { entropy })
    }

    /// Renders the code for the one screen that shows it. The buffer wipes itself when dropped.
    pub fn to_code_string(&self) -> Zeroizing<String> {
        Zeroizing::new(BASE32_NOPAD.encode(&self.entropy))
    }

    /// Derives the wrapping key this code unwraps the recovery slot with.
    pub fn recovery_key(&self) -> Result<RecoveryKey, CryptoError> {
        let hkdf = Hkdf::<Sha256>::new(None, &self.entropy);
        let mut bytes = [0u8; KEY_LEN];
        hkdf.expand(RECOVERY_INFO, &mut bytes)
            .map_err(|_| CryptoError::Derivation)?;
        let key = RecoveryKey::from_bytes(bytes);
        bytes.zeroize();
        Ok(key)
    }
}

impl fmt::Debug for RecoveryCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RecoveryCode(redacted)")
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{CODE_CHARS, RecoveryCode};
    use zeroize::ZeroizeOnDrop;

    const KAT_CODE: &str = "AAISEM2EKVTHPCEZVK54ZXPO74";
    const KAT_RECOVERY_KEY: [u8; 32] = [
        0x9a, 0x99, 0x89, 0xe4, 0x00, 0x5e, 0xf4, 0xe7, 0xca, 0xb6, 0xbb, 0xdd, 0xfb, 0x90, 0x1f,
        0x43, 0x87, 0xcb, 0x71, 0xf9, 0x06, 0x0b, 0x05, 0x83, 0xd3, 0xd2, 0x96, 0xee, 0xb1, 0xd6,
        0xd6, 0x63,
    ];

    #[test]
    fn a_fixed_recovery_code_derives_a_fixed_recovery_key() {
        let code = RecoveryCode::parse(KAT_CODE).expect("the vector must parse");
        let key = code.recovery_key().expect("hkdf must succeed");
        assert_eq!(key.as_bytes(), &KAT_RECOVERY_KEY);
    }

    #[test]
    fn a_recovery_code_zeroizes_on_drop() {
        fn assert_zeroize_on_drop<T: ZeroizeOnDrop>() {}
        assert_zeroize_on_drop::<RecoveryCode>();
    }

    #[test]
    fn a_generated_code_is_twenty_six_base32_characters() {
        let code = RecoveryCode::generate().expect("entropy must be available");
        let text = code.to_code_string();
        assert_eq!(text.len(), CODE_CHARS);
        assert!(
            text.chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        );
    }

    #[test]
    fn parsing_tolerates_dashes_spaces_and_lowercase() {
        let key = RecoveryCode::parse("aaise-m2ekv thpce zvk54-zxpo74")
            .expect("a formatted code must parse")
            .recovery_key()
            .expect("hkdf");
        assert_eq!(key.as_bytes(), &KAT_RECOVERY_KEY);
    }

    #[test]
    fn a_malformed_code_is_rejected_rather_than_truncated() {
        assert!(RecoveryCode::parse("").is_err());
        assert!(RecoveryCode::parse("AAISEM2EKVTHPCEZVK54ZXPO").is_err());
        assert!(RecoveryCode::parse("AAISEM2EKVTHPCEZVK54ZXPO71").is_err());
        assert!(RecoveryCode::parse("AAISEM2EKVTHPCEZVK54ZXPO7!").is_err());
    }

    #[test]
    fn a_recovery_code_never_prints_its_bytes() {
        let code = RecoveryCode::parse(KAT_CODE).expect("the vector must parse");
        assert_eq!(format!("{code:?}"), "RecoveryCode(redacted)");
    }
}
