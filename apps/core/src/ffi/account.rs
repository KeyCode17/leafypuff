use crate::infrastructure::{AuthClient, SyncClient};

use super::LeafyPuffCore;
use super::auth_records::{FfiChallenge, FfiSession};
use super::error::LeafyPuffCoreError;
use super::records::FfiSyncOutcome;

#[uniffi::export(async_runtime = "tokio")]
impl LeafyPuffCore {
    /// Creates the account and mails a six-digit code. The API answers a challenge rather than a
    /// session, so nothing is signed in until `verify_email` and then `verify_sign_in` run.
    pub async fn register(
        &self,
        base_url: String,
        email: String,
        password: String,
        display_name: Option<String>,
    ) -> Result<FfiChallenge, LeafyPuffCoreError> {
        let client = AuthClient::new(base_url)?;
        Ok(FfiChallenge::from(
            client.register(email, password, display_name).await?,
        ))
    }

    pub async fn verify_email(
        &self,
        base_url: String,
        email: String,
        code: String,
    ) -> Result<(), LeafyPuffCoreError> {
        AuthClient::new(base_url)?.verify_email(email, code).await?;
        Ok(())
    }

    pub async fn sign_in(
        &self,
        base_url: String,
        email: String,
        password: String,
    ) -> Result<FfiChallenge, LeafyPuffCoreError> {
        let client = AuthClient::new(base_url)?;
        Ok(FfiChallenge::from(client.sign_in(email, password).await?))
    }

    /// The device id is the same one the sync exchange uses, so the refresh credential the server
    /// issues is scoped to this install rather than to a value the screen invented.
    pub async fn verify_sign_in(
        &self,
        base_url: String,
        email: String,
        code: String,
    ) -> Result<FfiSession, LeafyPuffCoreError> {
        let device_id = self.outbox.device_id().await?;
        let client = AuthClient::new(base_url)?;
        Ok(FfiSession::from(
            client.verify_sign_in(email, code, device_id).await?,
        ))
    }

    pub async fn sync_now(
        &self,
        base_url: String,
        access_token: String,
    ) -> Result<FfiSyncOutcome, LeafyPuffCoreError> {
        let client = SyncClient::new(base_url, access_token)?;
        let outcome = client.exchange(&self.outbox).await?;
        Ok(FfiSyncOutcome::from(outcome))
    }

    pub async fn device_id(&self) -> Result<String, LeafyPuffCoreError> {
        Ok(self.outbox.device_id().await?)
    }
}
