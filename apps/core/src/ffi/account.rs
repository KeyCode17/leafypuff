use crate::domain::PhotoKind;
use crate::infrastructure::{AuthClient, MediaSync, SyncClient};

use super::LeafyPuffCore;
use super::auth_records::{FfiChallenge, FfiSession};
use super::error::LeafyPuffCoreError;
use super::records::FfiSyncOutcome;

#[uniffi::export(async_runtime = "tokio")]
impl LeafyPuffCore {
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

    pub async fn forgot_password(
        &self,
        base_url: String,
        email: String,
    ) -> Result<FfiChallenge, LeafyPuffCoreError> {
        let client = AuthClient::new(base_url)?;
        Ok(FfiChallenge::from(client.forgot_password(email).await?))
    }

    pub async fn reset_password(
        &self,
        base_url: String,
        email: String,
        code: String,
        password: String,
    ) -> Result<(), LeafyPuffCoreError> {
        AuthClient::new(base_url)?
            .reset_password(email, code, password)
            .await?;
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

    pub async fn refresh_session(
        &self,
        base_url: String,
        refresh_token: String,
    ) -> Result<FfiSession, LeafyPuffCoreError> {
        let device_id = self.outbox.device_id().await?;
        let client = AuthClient::new(base_url)?;
        Ok(FfiSession::from(
            client.refresh(refresh_token, device_id).await?,
        ))
    }

    pub async fn change_email(
        &self,
        base_url: String,
        access_token: String,
        email: String,
    ) -> Result<FfiChallenge, LeafyPuffCoreError> {
        let device_id = self.outbox.device_id().await?;
        let client = AuthClient::for_device(base_url, &device_id)?;
        Ok(FfiChallenge::from(
            client.change_email(&access_token, email).await?,
        ))
    }

    pub async fn confirm_email(
        &self,
        base_url: String,
        access_token: String,
        code: String,
    ) -> Result<String, LeafyPuffCoreError> {
        let device_id = self.outbox.device_id().await?;
        Ok(AuthClient::for_device(base_url, &device_id)?
            .confirm_email(&access_token, code)
            .await?)
    }

    pub async fn sync_now(
        &self,
        base_url: String,
        access_token: String,
    ) -> Result<FfiSyncOutcome, LeafyPuffCoreError> {
        let device_id = self.outbox.device_id().await?;
        let media = MediaSync::new(base_url.clone(), access_token.clone(), &device_id)?;
        for (id, entry_id) in self.outbox.pending_photos().await? {
            self.upload_photo(&media, &id, &entry_id).await?;
        }

        let client = SyncClient::new(base_url, access_token, &device_id)?;
        let outcome = client.exchange(&self.outbox).await?;

        for id in self.outbox.unfetched_photos().await? {
            self.fetch_photo(&media, &id).await?;
        }
        Ok(FfiSyncOutcome::from(outcome))
    }

    pub async fn forget_photo(
        &self,
        base_url: String,
        access_token: String,
        photo_id: String,
    ) -> Result<(), LeafyPuffCoreError> {
        let device_id = self.outbox.device_id().await?;
        MediaSync::new(base_url, access_token, &device_id)?
            .forget(&photo_id)
            .await?;
        self.photos.forget(&photo_id)?;
        self.outbox.forget_photo(&photo_id).await?;
        Ok(())
    }

    pub async fn device_id(&self) -> Result<String, LeafyPuffCoreError> {
        Ok(self.outbox.device_id().await?)
    }
}

impl LeafyPuffCore {
    async fn upload_photo(
        &self,
        media: &MediaSync,
        id: &str,
        entry_id: &str,
    ) -> Result<(), LeafyPuffCoreError> {
        for kind in [PhotoKind::Original, PhotoKind::Cover] {
            if !self.photos.holds(id, kind) {
                continue;
            }
            let sealed = self.photos.read_sealed(id, kind)?;
            media.upload(id, entry_id, kind, sealed).await?;
        }
        Ok(())
    }

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
