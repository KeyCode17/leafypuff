use crate::domain::{Challenge, Session};

#[derive(uniffi::Record)]
pub struct FfiChallenge {
    pub expires_in_seconds: i64,
}

impl From<Challenge> for FfiChallenge {
    fn from(challenge: Challenge) -> Self {
        Self {
            expires_in_seconds: challenge.expires_in_seconds,
        }
    }
}

#[derive(uniffi::Record)]
pub struct FfiSession {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in_seconds: i64,
}

impl From<Session> for FfiSession {
    fn from(session: Session) -> Self {
        Self {
            access_token: session.access_token,
            refresh_token: session.refresh_token,
            expires_in_seconds: session.expires_in_seconds,
        }
    }
}
