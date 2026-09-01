use crate::domain::CoreError;
use crate::domain::crypto::{ContentKey, KEY_LEN, KeyVault, RecoveryCode, open_for_device};
use crate::infrastructure::VaultSync;

use super::LeafyPuffCore;
use super::error::LeafyPuffCoreError;

const ERR_DEVICE_KEY_LEN: &str = "A device key must be 32 bytes";
const ERR_NO_DEVICE_SLOT: &str = "This device has never been unlocked";

#[uniffi::export(async_runtime = "tokio")]
impl LeafyPuffCore {
    pub async fn has_device_slot(&self) -> Result<bool, LeafyPuffCoreError> {
        Ok(self.device_slot.read().await?.is_some())
    }

    pub async fn remember_on_device(&self, device_key: Vec<u8>) -> Result<(), LeafyPuffCoreError> {
        let key = device_key_of(&device_key)?;
        let wrapped = self.sealer.seal_for_device(&key)?;
        self.device_slot.replace(&wrapped).await?;
        Ok(())
    }

    pub async fn unlock_with_device_key(
        &self,
        device_key: Vec<u8>,
    ) -> Result<(), LeafyPuffCoreError> {
        let key = device_key_of(&device_key)?;
        let wrapped = self
            .device_slot
            .read()
            .await?
            .ok_or_else(|| CoreError::Invalid(ERR_NO_DEVICE_SLOT.to_owned()))?;
        let content = open_for_device(&key, &wrapped).map_err(CoreError::from)?;
        self.sealer.unlock(content)?;
        Ok(())
    }

    pub async fn forget_device_key(&self) -> Result<(), LeafyPuffCoreError> {
        self.device_slot.forget().await?;
        Ok(())
    }

    pub async fn upload_vault(
        &self,
        base_url: String,
        access_token: String,
        updated_at_ms: i64,
    ) -> Result<(), LeafyPuffCoreError> {
        let held = self.vault.read().await?;
        VaultSync::new(base_url, access_token)?
            .push(&held, updated_at_ms)
            .await?;
        Ok(())
    }

    pub async fn change_passphrase(
        &self,
        base_url: String,
        access_token: String,
        current: String,
        replacement: String,
        updated_at_ms: i64,
    ) -> Result<(), LeafyPuffCoreError> {
        let vault = self.vault.read().await?;
        let content = vault
            .unlock_with_passphrase(&current)
            .map_err(CoreError::from)?;
        self.reseal(
            base_url,
            access_token,
            &vault,
            content,
            &replacement,
            updated_at_ms,
        )
        .await
    }

    pub async fn reseal_with_recovery_code(
        &self,
        base_url: String,
        access_token: String,
        code: String,
        replacement: String,
        updated_at_ms: i64,
    ) -> Result<(), LeafyPuffCoreError> {
        let vault = self.vault.read().await?;
        let parsed = RecoveryCode::parse(&code).map_err(CoreError::from)?;
        let content = vault
            .unlock_with_recovery_code(&parsed)
            .map_err(CoreError::from)?;
        self.reseal(
            base_url,
            access_token,
            &vault,
            content,
            &replacement,
            updated_at_ms,
        )
        .await
    }

    pub async fn restore_vault(
        &self,
        base_url: String,
        access_token: String,
    ) -> Result<bool, LeafyPuffCoreError> {
        let Some(held) = VaultSync::new(base_url, access_token)?.pull().await? else {
            return Ok(false);
        };
        self.vault.replace(&held).await?;
        Ok(true)
    }
}

impl LeafyPuffCore {
    async fn reseal(
        &self,
        base_url: String,
        access_token: String,
        vault: &KeyVault,
        content: ContentKey,
        replacement: &str,
        updated_at_ms: i64,
    ) -> Result<(), LeafyPuffCoreError> {
        let rewrapped = vault
            .rewrap_with(&content, replacement)
            .map_err(CoreError::from)?;
        self.vault.replace(&rewrapped).await?;
        self.sealer.unlock(content)?;
        VaultSync::new(base_url, access_token)?
            .push(&rewrapped, updated_at_ms)
            .await?;
        Ok(())
    }
}

fn device_key_of(raw: &[u8]) -> Result<[u8; KEY_LEN], CoreError> {
    <[u8; KEY_LEN]>::try_from(raw).map_err(|_| CoreError::Invalid(ERR_DEVICE_KEY_LEN.to_owned()))
}
