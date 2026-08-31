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
    #[error("Mail delivery failed")]
    MailFailed,
    #[error("Storage failure: {0}")]
    Storage(String),
}
