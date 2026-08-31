use std::sync::Arc;

use chrono::Duration;
use uuid::Uuid;

use crate::domain::iam::policy::REFRESH_TOKEN_TTL_SECONDS;
use crate::domain::iam::{Clock, IamError, RefreshToken, RefreshTokenRepository, TokenIssuer};

pub struct Session {
    pub access_token: String,
    pub refresh_secret: String,
}

pub struct MintSession {
    tokens: Arc<dyn TokenIssuer>,
    credentials: Arc<dyn RefreshTokenRepository>,
    clock: Arc<dyn Clock>,
}

impl MintSession {
    pub const fn new(
        tokens: Arc<dyn TokenIssuer>,
        credentials: Arc<dyn RefreshTokenRepository>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            tokens,
            credentials,
            clock,
        }
    }

    pub async fn execute(&self, account_id: Uuid, device_id: String) -> Result<Session, IamError> {
        let secret = self.tokens.refresh_secret()?;
        self.credentials
            .insert(RefreshToken {
                id: Uuid::new_v4(),
                account_id,
                device_id,
                token_hash: self.tokens.digest(&secret),
                expires_at: self.clock.now() + Duration::seconds(REFRESH_TOKEN_TTL_SECONDS),
                revoked_at: None,
            })
            .await?;
        Ok(Session {
            access_token: self.tokens.access_token(account_id)?,
            refresh_secret: secret,
        })
    }
}
