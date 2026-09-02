use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use leafypuff_api::domain::iam::{
    Account, AccountRepository, IamError, OtpCode, OtpPurpose, OtpRepository, RefreshToken,
    RefreshTokenRepository,
};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryAccounts {
    rows: Arc<Mutex<Vec<Account>>>,
}

impl InMemoryAccounts {
    pub fn snapshot(&self) -> Vec<Account> {
        self.rows.lock().expect("the account lock holds").clone()
    }
}

impl InMemoryCredentials {
    pub fn snapshot(&self) -> Vec<RefreshToken> {
        self.rows.lock().expect("the credential lock holds").clone()
    }
}

#[async_trait]
impl AccountRepository for InMemoryAccounts {
    async fn by_email(&self, email: &str) -> Result<Option<Account>, IamError> {
        let rows = self.rows.lock().expect("the account lock holds");
        Ok(rows
            .iter()
            .find(|row| row.email.eq_ignore_ascii_case(email))
            .cloned())
    }

    async fn by_id(&self, id: Uuid) -> Result<Option<Account>, IamError> {
        let rows = self.rows.lock().expect("the account lock holds");
        Ok(rows.iter().find(|row| row.id == id).cloned())
    }

    async fn insert(&self, account: Account) -> Result<Account, IamError> {
        let mut rows = self.rows.lock().expect("the account lock holds");
        if rows
            .iter()
            .any(|row| row.email.eq_ignore_ascii_case(&account.email))
        {
            return Err(IamError::EmailAlreadyRegistered);
        }
        rows.push(account.clone());
        Ok(account)
    }

    async fn mark_verified(&self, id: Uuid, at: DateTime<Utc>) -> Result<(), IamError> {
        let mut rows = self.rows.lock().expect("the account lock holds");
        for row in rows.iter_mut().filter(|row| row.id == id) {
            row.email_verified_at = Some(at);
        }
        Ok(())
    }

    async fn update_password(
        &self,
        id: Uuid,
        password_hash: String,
        _at: DateTime<Utc>,
    ) -> Result<(), IamError> {
        let mut rows = self.rows.lock().expect("the account lock holds");
        for row in rows.iter_mut().filter(|row| row.id == id) {
            row.password_hash.clone_from(&password_hash);
        }
        Ok(())
    }

    async fn hold_pending_email(
        &self,
        id: Uuid,
        email: Option<String>,
        _at: DateTime<Utc>,
    ) -> Result<(), IamError> {
        let mut rows = self.rows.lock().expect("the account lock holds");
        for row in rows.iter_mut().filter(|row| row.id == id) {
            row.pending_email.clone_from(&email);
        }
        Ok(())
    }

    async fn adopt_pending_email(
        &self,
        id: Uuid,
        email: String,
        _at: DateTime<Utc>,
    ) -> Result<(), IamError> {
        let mut rows = self.rows.lock().expect("the account lock holds");
        if rows
            .iter()
            .any(|row| row.id != id && row.email.eq_ignore_ascii_case(&email))
        {
            return Err(IamError::EmailAlreadyRegistered);
        }
        for row in rows.iter_mut().filter(|row| row.id == id) {
            row.email.clone_from(&email);
            row.pending_email = None;
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryOtps {
    rows: Arc<Mutex<Vec<OtpCode>>>,
}

#[async_trait]
impl OtpRepository for InMemoryOtps {
    async fn insert(&self, code: OtpCode) -> Result<(), IamError> {
        let mut rows = self.rows.lock().expect("the otp lock holds");
        rows.retain(|row| {
            row.consumed_at.is_some()
                || row.account_id != code.account_id
                || row.purpose != code.purpose
        });
        rows.push(code);
        Ok(())
    }

    async fn open_for(
        &self,
        account_id: Uuid,
        purpose: OtpPurpose,
    ) -> Result<Option<OtpCode>, IamError> {
        let rows = self.rows.lock().expect("the otp lock holds");
        Ok(rows
            .iter()
            .find(|row| {
                row.account_id == account_id && row.purpose == purpose && row.consumed_at.is_none()
            })
            .cloned())
    }

    async fn record_attempt(&self, id: Uuid) -> Result<(), IamError> {
        let mut rows = self.rows.lock().expect("the otp lock holds");
        for row in rows.iter_mut().filter(|row| row.id == id) {
            row.attempts += 1;
        }
        Ok(())
    }

    async fn consume(&self, id: Uuid, at: DateTime<Utc>) -> Result<(), IamError> {
        let mut rows = self.rows.lock().expect("the otp lock holds");
        for row in rows.iter_mut().filter(|row| row.id == id) {
            row.consumed_at = Some(at);
        }
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryCredentials {
    rows: Arc<Mutex<Vec<RefreshToken>>>,
}

#[async_trait]
impl RefreshTokenRepository for InMemoryCredentials {
    async fn insert(&self, token: RefreshToken) -> Result<(), IamError> {
        let mut rows = self.rows.lock().expect("the credential lock holds");
        for row in rows.iter_mut().filter(|row| {
            row.account_id == token.account_id
                && row.device_id == token.device_id
                && row.revoked_at.is_none()
        }) {
            row.revoked_at = Some(token.expires_at);
        }
        rows.push(token);
        Ok(())
    }

    async fn by_hash(&self, hash: &str) -> Result<Option<RefreshToken>, IamError> {
        let rows = self.rows.lock().expect("the credential lock holds");
        Ok(rows.iter().find(|row| row.token_hash == hash).cloned())
    }

    async fn revoke(&self, id: Uuid, at: DateTime<Utc>) -> Result<(), IamError> {
        let mut rows = self.rows.lock().expect("the credential lock holds");
        for row in rows.iter_mut().filter(|row| row.id == id) {
            row.revoked_at = Some(at);
        }
        Ok(())
    }

    async fn revoke_all(&self, account_id: Uuid, at: DateTime<Utc>) -> Result<(), IamError> {
        let mut rows = self.rows.lock().expect("the credential lock holds");
        for row in rows
            .iter_mut()
            .filter(|row| row.account_id == account_id && row.revoked_at.is_none())
        {
            row.revoked_at = Some(at);
        }
        Ok(())
    }
}
