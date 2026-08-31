use std::sync::Arc;

use crate::domain::iam::{AccountRepository, Clock, IamError, OtpPurpose, email};

use super::consume_challenge::ConsumeChallenge;

pub struct VerifyEmailInput {
    pub email: String,
    pub code: String,
}

pub struct VerifyEmail {
    accounts: Arc<dyn AccountRepository>,
    challenge: ConsumeChallenge,
    clock: Arc<dyn Clock>,
}

impl VerifyEmail {
    pub const fn new(
        accounts: Arc<dyn AccountRepository>,
        challenge: ConsumeChallenge,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            accounts,
            challenge,
            clock,
        }
    }

    pub async fn execute(&self, input: VerifyEmailInput) -> Result<(), IamError> {
        let account = self
            .accounts
            .by_email(&email::normalise(&input.email))
            .await?
            .ok_or(IamError::ChallengeUnusable)?;

        self.challenge
            .execute(account.id, OtpPurpose::VerifyEmail, &input.code)
            .await?;
        self.accounts
            .mark_verified(account.id, self.clock.now())
            .await
    }
}
