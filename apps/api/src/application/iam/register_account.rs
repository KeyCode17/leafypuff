use std::sync::Arc;

use uuid::Uuid;

use crate::domain::iam::{Account, AccountRepository, IamError, OtpPurpose, PasswordHasher, email};

use super::issue_challenge::IssueChallenge;

pub struct RegisterInput {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

pub struct RegisterAccount {
    accounts: Arc<dyn AccountRepository>,
    hasher: Arc<dyn PasswordHasher>,
    challenge: IssueChallenge,
}

impl RegisterAccount {
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

    pub async fn execute(&self, input: RegisterInput) -> Result<(), IamError> {
        let address = email::normalise(&input.email);
        let attempt = self
            .accounts
            .insert(Account {
                id: Uuid::new_v4(),
                email: address.clone(),
                password_hash: self.hasher.hash(&input.password)?,
                display_name: input.display_name,
                email_verified_at: None,
            })
            .await;

        let stored = match attempt {
            Ok(account) => account,
            Err(IamError::EmailAlreadyRegistered) => self.awaiting_verification(&address).await?,
            Err(error) => return Err(error),
        };

        self.challenge
            .execute(stored.id, &address, OtpPurpose::VerifyEmail)
            .await
    }

    async fn awaiting_verification(&self, address: &str) -> Result<Account, IamError> {
        let account = self
            .accounts
            .by_email(address)
            .await?
            .ok_or(IamError::EmailAlreadyRegistered)?;
        if account.is_verified() {
            return Err(IamError::EmailAlreadyRegistered);
        }
        Ok(account)
    }
}
