use std::sync::Arc;

use crate::domain::iam::{
    AccountRepository, Clock, IamError, OtpPurpose, PasswordHasher, RefreshTokenRepository, email,
};

use super::consume_challenge::ConsumeChallenge;

pub struct ResetPasswordInput {
    pub email: String,
    pub code: String,
    pub password: String,
}

pub struct ResetPassword {
    accounts: Arc<dyn AccountRepository>,
    credentials: Arc<dyn RefreshTokenRepository>,
    hasher: Arc<dyn PasswordHasher>,
    challenge: ConsumeChallenge,
    clock: Arc<dyn Clock>,
}

impl ResetPassword {
    pub const fn new(
        accounts: Arc<dyn AccountRepository>,
        credentials: Arc<dyn RefreshTokenRepository>,
        hasher: Arc<dyn PasswordHasher>,
        challenge: ConsumeChallenge,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            accounts,
            credentials,
            hasher,
            challenge,
            clock,
        }
    }

    pub async fn execute(&self, input: ResetPasswordInput) -> Result<(), IamError> {
        let account = self
            .accounts
            .by_email(&email::normalise(&input.email))
            .await?
            .ok_or(IamError::ChallengeUnusable)?;

        self.challenge
            .execute(account.id, OtpPurpose::ResetPassword, &input.code)
            .await?;

        let hash = self.hasher.hash(&input.password)?;
        let now = self.clock.now();
        self.accounts.update_password(account.id, hash, now).await?;
        self.credentials.revoke_all(account.id, now).await
    }
}
