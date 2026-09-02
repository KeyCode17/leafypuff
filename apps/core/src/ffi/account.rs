use crate::domain::PhotoKind;
use crate::domain::crop::Framing;
use crate::domain::{PhotoStore, ThumbnailMaker};
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
        let base_url_for_profile = base_url.clone();
        let access_token_for_profile = access_token.clone();
        let media = MediaSync::new(base_url.clone(), access_token.clone(), &device_id)?;
        for (id, entry_id) in self.outbox.pending_photos().await? {
            self.upload_photo(&media, &id, &entry_id).await?;
        }

        let client = SyncClient::new(base_url, access_token, &device_id)?;
        let outcome = client.exchange(&self.outbox).await?;

        for id in self.outbox.unfetched_photos().await? {
            self.fetch_photo(&media, &id).await?;
        }
        self.sync_profile(base_url_for_profile, access_token_for_profile)
            .await?;
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

    pub async fn frame_photo(
        &self,
        photo_id: String,
        x: f64,
        y: f64,
        width: f64,
    ) -> Result<(), LeafyPuffCoreError> {
        let framing = Framing { x, y, width }.clamped();
        let original = self.photos.read(&photo_id, PhotoKind::Original)?;
        let cover = self.thumbnails.framed_cover(&original, framing)?;
        self.photos.write(&photo_id, PhotoKind::Cover, &cover)?;
        self.outbox.frame_photo(&photo_id, framing).await?;
        Ok(())
    }

    pub async fn frame_avatar(
        &self,
        photo_id: String,
        x: f64,
        y: f64,
        width: f64,
    ) -> Result<(), LeafyPuffCoreError> {
        let framing = Framing { x, y, width }.clamped();
        let original = self.photos.read(&photo_id, PhotoKind::Original)?;
        let square = self.thumbnails.framed_square(&original, framing)?;
        self.photos.write(&photo_id, PhotoKind::Cover, &square)?;
        self.remember_avatar_framing(&photo_id, framing).await?;
        Ok(())
    }

    pub async fn place_photo(
        &self,
        photo_id: String,
        x: f64,
        y: f64,
        size: f64,
        rotation: f64,
    ) -> Result<(), LeafyPuffCoreError> {
        self.outbox
            .place_photo(&photo_id, x, y, size, rotation)
            .await?;
        Ok(())
    }

    pub async fn photo_placement(&self, photo_id: String) -> Result<Vec<f64>, LeafyPuffCoreError> {
        Ok(self
            .outbox
            .placement_of(&photo_id)
            .await?
            .map_or_else(Vec::new, |held| vec![held[0], held[1], held[2], held[3]]))
    }

    pub async fn photo_framing(&self, photo_id: String) -> Result<Vec<f64>, LeafyPuffCoreError> {
        Ok(self
            .outbox
            .framing_of(&photo_id)
            .await?
            .map_or_else(Vec::new, |held| vec![held.x, held.y, held.width]))
    }

    pub async fn original_photo(&self, photo_id: String) -> Result<Vec<u8>, LeafyPuffCoreError> {
        Ok(self.photos.read(&photo_id, PhotoKind::Original)?)
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
        let Some(sealed) = media.download(id, PhotoKind::Original).await? else {
            return Ok(());
        };
        self.photos.write_sealed(id, PhotoKind::Original, &sealed)?;

        let original = self.photos.read(id, PhotoKind::Original)?;
        let cover = match self.outbox.framing_of(id).await? {
            Some(held) => self.thumbnails.framed_cover(&original, held)?,
            None => self.thumbnails.cover(&original)?,
        };
        self.photos.write(id, PhotoKind::Cover, &cover)?;
        let path = self.photos.root().join(id).to_string_lossy().into_owned();
        self.outbox.record_photo_path(id, &path).await?;
        Ok(())
    }
}
