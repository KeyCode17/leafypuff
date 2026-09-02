use std::sync::Arc;

use uuid::Uuid;

use crate::domain::iam::{AccountRepository, Clock, IamError, OtpPurpose, email};

use super::issue_challenge::IssueChallenge;

pub struct StartEmailChangeInput {
    pub account_id: Uuid,
    pub email: String,
}

pub struct StartEmailChange {
    accounts: Arc<dyn AccountRepository>,
    challenge: IssueChallenge,
    clock: Arc<dyn Clock>,
}

impl StartEmailChange {
    pub const fn new(
        accounts: Arc<dyn AccountRepository>,
        challenge: IssueChallenge,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            accounts,
            challenge,
            clock,
        }
    }

    pub async fn execute(&self, input: StartEmailChangeInput) -> Result<(), IamError> {
        let wanted = email::normalise(&input.email);
        let account = self
            .accounts
            .by_id(input.account_id)
            .await?
            .ok_or(IamError::InvalidCredentials)?;

        if account.email == wanted {
            return Err(IamError::EmailAlreadyRegistered);
        }
        if self.accounts.by_email(&wanted).await?.is_some() {
            return Err(IamError::EmailAlreadyRegistered);
        }

        self.accounts
            .hold_pending_email(account.id, Some(wanted.clone()), self.clock.now())
            .await?;
        self.challenge
            .execute(account.id, &wanted, OtpPurpose::ChangeEmail)
            .await
    }
}
