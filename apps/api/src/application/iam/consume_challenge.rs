use std::sync::Arc;

use uuid::Uuid;

use crate::domain::iam::{Clock, IamError, OtpGenerator, OtpPurpose, OtpRepository};

pub struct ConsumeChallenge {
    otps: Arc<dyn OtpRepository>,
    generator: Arc<dyn OtpGenerator>,
    clock: Arc<dyn Clock>,
}

impl ConsumeChallenge {
    pub const fn new(
        otps: Arc<dyn OtpRepository>,
        generator: Arc<dyn OtpGenerator>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            otps,
            generator,
            clock,
        }
    }

    pub async fn execute(
        &self,
        account_id: Uuid,
        purpose: OtpPurpose,
        code: &str,
    ) -> Result<(), IamError> {
        let now = self.clock.now();
        let challenge = self
            .otps
            .open_for(account_id, purpose)
            .await?
            .ok_or(IamError::ChallengeUnusable)?;

        if !challenge.is_open(now) {
            return Err(IamError::ChallengeUnusable);
        }
        if challenge.code_hash != self.generator.digest(code) {
            self.otps.record_attempt(challenge.id).await?;
            return Err(IamError::InvalidCode);
        }
        self.otps.consume(challenge.id, now).await
    }
}
