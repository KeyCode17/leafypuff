pub const ERR_UNKNOWN_OTP_PURPOSE: &str = "Stored otp purpose is not a known variant";
pub const ERR_ENTROPY_UNAVAILABLE: &str = "Operating system entropy is unavailable";
pub const ERR_ARGON_PARAMS_REJECTED: &str = "Argon2 rejected the configured cost parameters";
pub const ERR_PASSWORD_HASHING_FAILED: &str = "Password hashing failed";

#[derive(Debug, thiserror::Error)]
pub enum IamError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Email already registered")]
    EmailAlreadyRegistered,
    #[error("Challenge expired or already used")]
    ChallengeUnusable,
    #[error("Too many attempts")]
    TooManyAttempts,
    #[error("Mail delivery failed: {0}")]
    Mail(String),
    #[error("Storage failure: {0}")]
    Storage(String),
}
