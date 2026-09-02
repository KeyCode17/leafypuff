use crate::domain::crop::Framing;
use crate::domain::{ContentSealer, PhotoKind, PhotoStore, Profile, ThumbnailMaker};
use crate::infrastructure::profile_sync::{ProfileSync, RemoteProfile};

use super::LeafyPuffCore;
use super::error::LeafyPuffCoreError;

const SEAL_LABEL: &str = "profile";
const ERR_SEALED_SHAPE: &str = "The stored profile could not be read";

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct FfiProfile {
    pub display_name: Option<String>,
    pub avatar_photo_id: Option<String>,
    pub updated_at_ms: i64,
}

impl From<Profile> for FfiProfile {
    fn from(held: Profile) -> Self {
        Self {
            display_name: held.display_name,
            avatar_photo_id: held.avatar_photo_id,
            updated_at_ms: held.updated_at_ms,
        }
    }
}

#[uniffi::export(async_runtime = "tokio")]
impl LeafyPuffCore {
    pub async fn profile(&self) -> Result<FfiProfile, LeafyPuffCoreError> {
        Ok(FfiProfile::from(self.profile.read().await?))
    }

    pub async fn save_profile(
        &self,
        display_name: Option<String>,
        avatar_photo_id: Option<String>,
    ) -> Result<FfiProfile, LeafyPuffCoreError> {
        let held = self.profile.read().await?;
        let wanted = Profile {
            display_name,
            avatar_photo_id: avatar_photo_id.clone(),
            avatar_framing: match avatar_photo_id == held.avatar_photo_id {
                true => held.avatar_framing,
                false => None,
            },
            updated_at_ms: self.stamp(held.updated_at_ms),
        };
        self.profile.save(&wanted).await?;
        Ok(FfiProfile::from(wanted))
    }

    pub async fn sync_profile(
        &self,
        base_url: String,
        access_token: String,
    ) -> Result<FfiProfile, LeafyPuffCoreError> {
        let device_id = self.outbox.device_id().await?;
        let client = ProfileSync::new(base_url, access_token, &device_id)?;
        let local = self.profile.read().await?;
        let remote = client.pull().await?;

        let settled = match local.updated_at_ms.cmp(&remote.updated_at_ms) {
            std::cmp::Ordering::Greater => {
                client.push(&self.sealed(&local)?).await?;
                local
            }
            std::cmp::Ordering::Less => {
                let adopted = self.opened(&remote)?;
                self.profile.save(&adopted).await?;
                adopted
            }
            std::cmp::Ordering::Equal => local,
        };

        self.carry_avatar(&client, &settled).await?;
        Ok(FfiProfile::from(settled))
    }
}

impl LeafyPuffCore {
    pub(super) fn stamp(&self, previous: i64) -> i64 {
        let now = chrono::Utc::now().timestamp_millis();
        match now > previous {
            true => now,
            false => previous + 1,
        }
    }

    pub(super) async fn remember_avatar_framing(
        &self,
        photo_id: &str,
        framing: Framing,
    ) -> Result<(), LeafyPuffCoreError> {
        let held = self.profile.read().await?;
        let wanted = Profile {
            display_name: held.display_name,
            avatar_photo_id: Some(photo_id.to_owned()),
            avatar_framing: Some(framing),
            updated_at_ms: self.stamp(held.updated_at_ms),
        };
        self.profile.save(&wanted).await?;
        Ok(())
    }

    fn sealed(&self, held: &Profile) -> Result<RemoteProfile, LeafyPuffCoreError> {
        let document = serde_json::json!({
            "display_name": held.display_name,
            "avatar_framing": held
                .avatar_framing
                .map(|framing| [framing.x, framing.y, framing.width]),
        });
        let sealed = self
            .sealer
            .seal(SEAL_LABEL, document.to_string().as_bytes())?;
        Ok(RemoteProfile {
            sealed_profile: Some(sealed),
            avatar_photo_id: held.avatar_photo_id.clone(),
            updated_at_ms: held.updated_at_ms,
        })
    }

    fn opened(&self, remote: &RemoteProfile) -> Result<Profile, LeafyPuffCoreError> {
        let Some(sealed) = remote.sealed_profile.as_deref() else {
            return Ok(Profile {
                display_name: None,
                avatar_photo_id: remote.avatar_photo_id.clone(),
                avatar_framing: None,
                updated_at_ms: remote.updated_at_ms,
            });
        };
        let plain = self.sealer.open(SEAL_LABEL, sealed)?;
        let document: serde_json::Value = serde_json::from_slice(&plain)
            .map_err(|_| crate::domain::CoreError::Storage(ERR_SEALED_SHAPE.to_owned()))?;
        Ok(Profile {
            display_name: document["display_name"].as_str().map(str::to_owned),
            avatar_photo_id: remote.avatar_photo_id.clone(),
            avatar_framing: framing_of(&document["avatar_framing"]),
            updated_at_ms: remote.updated_at_ms,
        })
    }

    async fn carry_avatar(
        &self,
        client: &ProfileSync,
        settled: &Profile,
    ) -> Result<(), LeafyPuffCoreError> {
        let Some(id) = settled.avatar_photo_id.as_deref() else {
            return Ok(());
        };
        if self.photos.holds(id, PhotoKind::Original) {
            for kind in [PhotoKind::Original, PhotoKind::Cover] {
                if self.photos.holds(id, kind) {
                    let sealed = self.photos.read_sealed(id, kind)?;
                    client.upload_avatar(kind, sealed).await?;
                }
            }
            return Ok(());
        }

        let Some(sealed) = client.download_avatar(PhotoKind::Original).await? else {
            return Ok(());
        };
        self.photos.write_sealed(id, PhotoKind::Original, &sealed)?;
        let original = self.photos.read(id, PhotoKind::Original)?;
        let square = self
            .thumbnails
            .framed_square(&original, settled.avatar_framing.unwrap_or_default())?;
        self.photos.write(id, PhotoKind::Cover, &square)?;
        Ok(())
    }
}

fn framing_of(value: &serde_json::Value) -> Option<Framing> {
    let held = value.as_array()?;
    Some(Framing {
        x: held.first()?.as_f64()?,
        y: held.get(1)?.as_f64()?,
        width: held.get(2)?.as_f64()?,
    })
}
