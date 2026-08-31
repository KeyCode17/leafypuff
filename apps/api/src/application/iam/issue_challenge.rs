use std::sync::Arc;

use chrono::Duration;
use uuid::Uuid;

use crate::domain::iam::policy::OTP_TTL_SECONDS;
use crate::domain::iam::{
    Clock, EmailSender, IamError, OtpCode, OtpGenerator, OtpPurpose, OtpRepository,
};

pub struct IssueChallenge {
    otps: Arc<dyn OtpRepository>,
    generator: Arc<dyn OtpGenerator>,
    mail: Arc<dyn EmailSender>,
    clock: Arc<dyn Clock>,
}

impl IssueChallenge {
    pub const fn new(
        otps: Arc<dyn OtpRepository>,
        generator: Arc<dyn OtpGenerator>,
        mail: Arc<dyn EmailSender>,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            otps,
            generator,
            mail,
            clock,
        }
    }

    pub async fn execute(
        &self,
        account_id: Uuid,
        email: &str,
        purpose: OtpPurpose,
    ) -> Result<(), IamError> {
        let code = self.generator.code()?;
        self.otps
            .insert(OtpCode {
                id: Uuid::new_v4(),
                account_id,
                code_hash: self.generator.digest(&code),
                purpose,
                attempts: 0,
                expires_at: self.clock.now() + Duration::seconds(OTP_TTL_SECONDS),
                consumed_at: None,
            })
            .await?;
        self.mail.send_code(email, &code, purpose).await
    }
}
