use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub email_verified_at: Option<DateTime<Utc>>,
}

impl Account {
    pub const fn is_verified(&self) -> bool {
        self.email_verified_at.is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshToken {
    pub id: Uuid,
    pub account_id: Uuid,
    pub device_id: String,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

impl RefreshToken {
    pub fn is_usable(&self, now: DateTime<Utc>) -> bool {
        self.revoked_at.is_none() && self.expires_at > now
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtpPurpose {
    VerifyEmail,
    SignIn,
}

impl OtpPurpose {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifyEmail => "verify_email",
            Self::SignIn => "sign_in",
        }
    }

    pub fn from_stored(value: &str) -> Option<Self> {
        match value {
            "verify_email" => Some(Self::VerifyEmail),
            "sign_in" => Some(Self::SignIn),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpCode {
    pub id: Uuid,
    pub account_id: Uuid,
    pub code_hash: String,
    pub purpose: OtpPurpose,
    pub attempts: i32,
    pub expires_at: DateTime<Utc>,
    pub consumed_at: Option<DateTime<Utc>>,
}

impl OtpCode {
    pub const MAX_ATTEMPTS: i32 = 5;

    pub fn is_open(&self, now: DateTime<Utc>) -> bool {
        self.consumed_at.is_none() && self.expires_at > now && self.attempts < Self::MAX_ATTEMPTS
    }
}
