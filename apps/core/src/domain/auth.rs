use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in_seconds: i64,
}

impl fmt::Debug for Session {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Session")
            .field("access_token_len", &self.access_token.len())
            .field("refresh_token_len", &self.refresh_token.len())
            .field("expires_in_seconds", &self.expires_in_seconds)
            .finish()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Challenge {
    pub expires_in_seconds: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rejection {
    InvalidCredentials,
    EmailNotVerified,
    EmailTaken,
    TooManyAttempts,
    MailUnavailable,
    ServiceUnavailable,
    Unknown,
}

const CODE_INVALID_CREDENTIALS: &str = "INVALID_CREDENTIALS";
const CODE_EMAIL_NOT_VERIFIED: &str = "EMAIL_NOT_VERIFIED";
const CODE_EMAIL_TAKEN: &str = "EMAIL_ALREADY_REGISTERED";
const CODE_TOO_MANY_ATTEMPTS: &str = "TOO_MANY_ATTEMPTS";
const CODE_TOO_MANY_REQUESTS: &str = "TOO_MANY_REQUESTS";
const CODE_MAIL_UNAVAILABLE: &str = "MAIL_UNAVAILABLE";
const CODE_DEPENDENCY_UNAVAILABLE: &str = "DEPENDENCY_UNAVAILABLE";
const CODE_LIMITER_UNAVAILABLE: &str = "LIMITER_UNAVAILABLE";

impl Rejection {
    pub fn from_code(code: &str) -> Self {
        match code {
            CODE_INVALID_CREDENTIALS => Self::InvalidCredentials,
            CODE_EMAIL_NOT_VERIFIED => Self::EmailNotVerified,
            CODE_EMAIL_TAKEN => Self::EmailTaken,
            CODE_TOO_MANY_ATTEMPTS | CODE_TOO_MANY_REQUESTS => Self::TooManyAttempts,
            CODE_MAIL_UNAVAILABLE => Self::MailUnavailable,
            CODE_DEPENDENCY_UNAVAILABLE | CODE_LIMITER_UNAVAILABLE => Self::ServiceUnavailable,
            _ => Self::Unknown,
        }
    }
}

impl fmt::Display for Rejection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::InvalidCredentials => CODE_INVALID_CREDENTIALS,
            Self::EmailNotVerified => CODE_EMAIL_NOT_VERIFIED,
            Self::EmailTaken => CODE_EMAIL_TAKEN,
            Self::TooManyAttempts => CODE_TOO_MANY_ATTEMPTS,
            Self::MailUnavailable => CODE_MAIL_UNAVAILABLE,
            Self::ServiceUnavailable => CODE_DEPENDENCY_UNAVAILABLE,
            Self::Unknown => "UNKNOWN",
        };
        formatter.write_str(text)
    }
}
