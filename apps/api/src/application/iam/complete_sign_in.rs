use std::sync::Arc;

use crate::domain::iam::{AccountRepository, IamError, OtpPurpose, email};

use super::consume_challenge::ConsumeChallenge;
use super::mint_session::{MintSession, Session};

pub struct CompleteSignInInput {
    pub email: String,
    pub code: String,
    pub device_id: String,
}

pub struct CompleteSignIn {
    accounts: Arc<dyn AccountRepository>,
    challenge: ConsumeChallenge,
    session: MintSession,
}

impl CompleteSignIn {
    pub const fn new(
        accounts: Arc<dyn AccountRepository>,
        challenge: ConsumeChallenge,
        session: MintSession,
    ) -> Self {
        Self {
            accounts,
            challenge,
            session,
        }
    }

    pub async fn execute(&self, input: CompleteSignInInput) -> Result<Session, IamError> {
        let account = self
            .accounts
            .by_email(&email::normalise(&input.email))
            .await?
            .ok_or(IamError::ChallengeUnusable)?;

        self.challenge
            .execute(account.id, OtpPurpose::SignIn, &input.code)
            .await?;
        self.session.execute(account.id, input.device_id).await
    }
}
