use std::sync::Arc;

use crate::domain::iam::{AccountRepository, IamError, OtpPurpose, email};

use super::issue_challenge::IssueChallenge;

pub struct StartPasswordResetInput {
    pub email: String,
}

pub struct StartPasswordReset {
    accounts: Arc<dyn AccountRepository>,
    challenge: IssueChallenge,
}

impl StartPasswordReset {
    pub const fn new(accounts: Arc<dyn AccountRepository>, challenge: IssueChallenge) -> Self {
        Self {
            accounts,
            challenge,
        }
    }

    pub async fn execute(&self, input: StartPasswordResetInput) -> Result<(), IamError> {
        let address = email::normalise(&input.email);
        let Some(account) = self.accounts.by_email(&address).await? else {
            return Ok(());
        };
        if !account.is_verified() {
            return Ok(());
        }
        self.challenge
            .execute(account.id, &address, OtpPurpose::ResetPassword)
            .await
    }
}
