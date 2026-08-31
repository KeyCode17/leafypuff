use std::sync::Arc;

use crate::domain::iam::{AccountRepository, IamError, OtpPurpose, PasswordHasher, email};

use super::issue_challenge::IssueChallenge;

pub struct StartSignInInput {
    pub email: String,
    pub password: String,
}

pub struct StartSignIn {
    accounts: Arc<dyn AccountRepository>,
    hasher: Arc<dyn PasswordHasher>,
    challenge: IssueChallenge,
}

impl StartSignIn {
    pub const fn new(
        accounts: Arc<dyn AccountRepository>,
        hasher: Arc<dyn PasswordHasher>,
        challenge: IssueChallenge,
    ) -> Self {
        Self {
            accounts,
            hasher,
            challenge,
        }
    }

    pub async fn execute(&self, input: StartSignInInput) -> Result<(), IamError> {
        let address = email::normalise(&input.email);
        let Some(account) = self.accounts.by_email(&address).await? else {
            self.hasher.decoy_verify(&input.password);
            return Err(IamError::InvalidCredentials);
        };

        if !self.hasher.verify(&input.password, &account.password_hash) {
            return Err(IamError::InvalidCredentials);
        }
        if !account.is_verified() {
            return Err(IamError::EmailNotVerified);
        }

        self.challenge
            .execute(account.id, &address, OtpPurpose::SignIn)
            .await
    }
}
