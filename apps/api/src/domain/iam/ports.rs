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
    fn refresh_secret(&self) -> String;
    fn digest(&self, secret: &str) -> String;
}

pub trait OtpGenerator: Send + Sync {
    fn code(&self) -> String;
    fn digest(&self, code: &str) -> String;
}

pub trait EmailSender: Send + Sync {
    fn send_code(
        &self,
        to: &str,
        code: &str,
        purpose: OtpPurpose,
    ) -> impl Future<Output = Result<(), IamError>> + Send;
}

pub trait AccountRepository: Send + Sync {
    fn by_email(
        &self,
        email: &str,
    ) -> impl Future<Output = Result<Option<Account>, IamError>> + Send;
    fn by_id(&self, id: Uuid) -> impl Future<Output = Result<Option<Account>, IamError>> + Send;
    fn insert(&self, account: Account) -> impl Future<Output = Result<Account, IamError>> + Send;
    fn mark_verified(
        &self,
        id: Uuid,
        at: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), IamError>> + Send;
}

pub trait RefreshTokenRepository: Send + Sync {
    fn insert(&self, token: RefreshToken) -> impl Future<Output = Result<(), IamError>> + Send;
    fn by_hash(
        &self,
        hash: &str,
    ) -> impl Future<Output = Result<Option<RefreshToken>, IamError>> + Send;
    fn revoke(
        &self,
        id: Uuid,
        at: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), IamError>> + Send;
}

pub trait OtpRepository: Send + Sync {
    fn insert(&self, code: OtpCode) -> impl Future<Output = Result<(), IamError>> + Send;
    fn open_for(
        &self,
        account_id: Uuid,
        purpose: OtpPurpose,
    ) -> impl Future<Output = Result<Option<OtpCode>, IamError>> + Send;
    fn record_attempt(&self, id: Uuid) -> impl Future<Output = Result<(), IamError>> + Send;
    fn consume(
        &self,
        id: Uuid,
        at: DateTime<Utc>,
    ) -> impl Future<Output = Result<(), IamError>> + Send;
}
