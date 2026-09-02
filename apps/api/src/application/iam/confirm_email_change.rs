use std::sync::Arc;

use uuid::Uuid;

use crate::domain::iam::{AccountRepository, Clock, IamError, OtpPurpose};

use super::consume_challenge::ConsumeChallenge;

pub struct ConfirmEmailChangeInput {
    pub account_id: Uuid,
    pub code: String,
}

pub struct ConfirmEmailChange {
    accounts: Arc<dyn AccountRepository>,
    challenge: ConsumeChallenge,
    clock: Arc<dyn Clock>,
}

impl ConfirmEmailChange {
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

    pub async fn execute(&self, input: ConfirmEmailChangeInput) -> Result<String, IamError> {
        let account = self
            .accounts
            .by_id(input.account_id)
            .await?
            .ok_or(IamError::InvalidCredentials)?;
        let wanted = account.pending_email.ok_or(IamError::ChallengeUnusable)?;

        self.challenge
            .execute(account.id, OtpPurpose::ChangeEmail, &input.code)
            .await?;

        let now = self.clock.now();
        self.accounts
            .adopt_pending_email(account.id, wanted.clone(), now)
            .await?;
        Ok(wanted)
    }
}
