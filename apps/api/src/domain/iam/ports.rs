use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::account::{Account, OtpCode, OtpPurpose, RefreshToken};
use super::error::IamError;

pub trait Clock: Send + Sync {
    fn now(&self) -> DateTime<Utc>;
}

pub trait PasswordHasher: Send + Sync {
    fn hash(&self, plain: &str) -> Result<String, IamError>;
    fn verify(&self, plain: &str, hash: &str) -> bool;
    fn decoy_verify(&self, plain: &str);
}

pub trait TokenIssuer: Send + Sync {
    fn access_token(&self, account_id: Uuid) -> Result<String, IamError>;
    fn refresh_secret(&self) -> Result<String, IamError>;
    fn digest(&self, secret: &str) -> String;
}

pub trait TokenVerifier: Send + Sync {
    fn account_id(&self, access_token: &str) -> Result<Uuid, IamError>;
}

pub trait OtpGenerator: Send + Sync {
    fn code(&self) -> Result<String, IamError>;
    fn digest(&self, code: &str) -> String;
}

#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send_code(&self, to: &str, code: &str, purpose: OtpPurpose) -> Result<(), IamError>;
}

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn by_email(&self, email: &str) -> Result<Option<Account>, IamError>;
    async fn by_id(&self, id: Uuid) -> Result<Option<Account>, IamError>;
    async fn insert(&self, account: Account) -> Result<Account, IamError>;
    async fn mark_verified(&self, id: Uuid, at: DateTime<Utc>) -> Result<(), IamError>;
}

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn insert(&self, token: RefreshToken) -> Result<(), IamError>;
    async fn by_hash(&self, hash: &str) -> Result<Option<RefreshToken>, IamError>;
    async fn revoke(&self, id: Uuid, at: DateTime<Utc>) -> Result<(), IamError>;
}

#[async_trait]
pub trait OtpRepository: Send + Sync {
    async fn insert(&self, code: OtpCode) -> Result<(), IamError>;
    async fn open_for(
        &self,
        account_id: Uuid,
        purpose: OtpPurpose,
    ) -> Result<Option<OtpCode>, IamError>;
    async fn record_attempt(&self, id: Uuid) -> Result<(), IamError>;
    async fn consume(&self, id: Uuid, at: DateTime<Utc>) -> Result<(), IamError>;
}
