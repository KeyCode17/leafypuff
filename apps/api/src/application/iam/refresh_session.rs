use std::sync::Arc;

use crate::domain::iam::{Clock, IamError, RefreshTokenRepository, TokenIssuer};

use super::mint_session::{MintSession, Session};

pub struct RefreshInput {
    pub refresh_secret: String,
    pub device_id: String,
}

pub struct RefreshSession {
    credentials: Arc<dyn RefreshTokenRepository>,
    tokens: Arc<dyn TokenIssuer>,
    clock: Arc<dyn Clock>,
    session: MintSession,
}

impl RefreshSession {
    pub const fn new(
        credentials: Arc<dyn RefreshTokenRepository>,
        tokens: Arc<dyn TokenIssuer>,
        clock: Arc<dyn Clock>,
        session: MintSession,
    ) -> Self {
        Self {
            credentials,
            tokens,
            clock,
            session,
        }
    }

    pub async fn execute(&self, input: RefreshInput) -> Result<Session, IamError> {
        let presented = self
            .credentials
            .by_hash(&self.tokens.digest(&input.refresh_secret))
            .await?
            .ok_or(IamError::InvalidCredentials)?;

        if presented.device_id != input.device_id {
            return Err(IamError::InvalidCredentials);
        }
        if !presented.is_usable(self.clock.now()) {
            self.credentials
                .revoke(presented.id, self.clock.now())
                .await?;
            return Err(IamError::InvalidCredentials);
        }

        self.session
            .execute(presented.account_id, presented.device_id)
            .await
    }
}
