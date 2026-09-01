use crate::domain::PhotoKind;
use crate::infrastructure::{AuthClient, MediaSync, SyncClient};

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

    /// One exchange: photos this device owes go up first, then the entries that name them, then
    /// whatever another device wrote comes down, then the blobs those entries are still missing.
    /// Photos lead on the way up so a record never lands pointing at a blob the server lacks.
    pub async fn sync_now(
        &self,
        base_url: String,
        access_token: String,
    ) -> Result<FfiSyncOutcome, LeafyPuffCoreError> {
        let media = MediaSync::new(base_url.clone(), access_token.clone())?;
        for id in self.outbox.pending_photo_ids().await? {
            self.upload_photo(&media, &id).await?;
        }

        let client = SyncClient::new(base_url, access_token)?;
        let outcome = client.exchange(&self.outbox).await?;

        for id in self.outbox.unfetched_photos().await? {
            self.fetch_photo(&media, &id).await?;
        }
        Ok(FfiSyncOutcome::from(outcome))
    }

    pub async fn device_id(&self) -> Result<String, LeafyPuffCoreError> {
        Ok(self.outbox.device_id().await?)
    }
}

impl LeafyPuffCore {
    async fn upload_photo(&self, media: &MediaSync, id: &str) -> Result<(), LeafyPuffCoreError> {
        for kind in [PhotoKind::Original, PhotoKind::Cover] {
            if !self.photos.holds(id, kind) {
                continue;
            }
            let sealed = self.photos.read_sealed(id, kind)?;
            media.upload(id, kind, sealed).await?;
        }
        Ok(())
    }

    /// The blob comes down sealed and is written as it arrived. Nothing here opens it: a device
    /// that cannot open it has the wrong content key, and that is a question for the unlock
    /// screen rather than for the sync.
    async fn fetch_photo(&self, media: &MediaSync, id: &str) -> Result<(), LeafyPuffCoreError> {
        let Some(original) = media.download(id, PhotoKind::Original).await? else {
            return Ok(());
        };
        self.photos
            .write_sealed(id, PhotoKind::Original, &original)?;
        if let Some(cover) = media.download(id, PhotoKind::Cover).await? {
            self.photos.write_sealed(id, PhotoKind::Cover, &cover)?;
        }
        let path = self.photos.root().join(id).to_string_lossy().into_owned();
        self.outbox.record_photo_path(id, &path).await?;
        Ok(())
    }
}
