pub mod account;
pub mod auth_records;
pub mod entries;
pub mod enums;
pub mod error;
pub mod photos;
pub mod records;
pub mod stats_records;
pub mod vault;

use std::sync::Arc;

use chrono::NaiveDate;

use crate::domain::CoreError;
use crate::domain::crypto::{
    KeyVault, RecoveryCode, hash_pin as crypto_hash_pin, verify_pin as crypto_verify_pin,
};
use crate::domain::error::ERR_DATE_UNREADABLE;
use crate::infrastructure::{
    FilePhotoStore, ImageThumbnailer, KamadakExifReader, SqliteDeviceSlotStore,
    SqliteEntryRepository, SqliteVaultStore, SyncOutbox, SystemClock, VaultSealer, db,
};

pub use auth_records::{FfiChallenge, FfiSession};
pub use error::LeafyPuffCoreError;
pub use records::{FfiEntry, FfiPhoto, FfiPlacedSticker, FfiSyncOutcome};
pub use stats_records::{
    FfiGroupCount, FfiMoodCount, FfiMoodGroup, FfiStats, FfiStatsRange, FfiTagCount,
    FfiWeekdayCount,
};

use records::ISO_DATE;

pub(super) fn read_date(raw: &str) -> Result<NaiveDate, CoreError> {
    NaiveDate::parse_from_str(raw, ISO_DATE)
        .map_err(|_| CoreError::Invalid(format!("{ERR_DATE_UNREADABLE}: {raw}")))
}

/// Hashes a screen-lock PIN. The caller stores the returned string and never the digits. Free
/// rather than a method: the screen lock has nothing to do with the database handle.
#[uniffi::export]
pub fn hash_pin(pin: String) -> Result<String, LeafyPuffCoreError> {
    Ok(crypto_hash_pin(&pin).map_err(CoreError::from)?)
}

#[uniffi::export]
pub fn verify_pin(pin: String, stored: String) -> bool {
    crypto_verify_pin(&pin, &stored)
}

/// The only handle Kotlin holds. It owns the connection and every adapter behind it.
#[derive(uniffi::Object)]
pub struct LeafyPuffCore {
    repository: SqliteEntryRepository<VaultSealer>,
    vault: SqliteVaultStore,
    device_slot: SqliteDeviceSlotStore,
    outbox: SyncOutbox,
    sealer: VaultSealer,
    clock: SystemClock,
    photos: FilePhotoStore<VaultSealer>,
    exif: KamadakExifReader,
    thumbnails: ImageThumbnailer,
}

#[uniffi::export(async_runtime = "tokio")]
impl LeafyPuffCore {
    #[uniffi::constructor(name = "open")]
    pub async fn new(db_path: String) -> Result<Arc<Self>, LeafyPuffCoreError> {
        let connection = db::open(&db_path).await?;
        db::run_migrations(&connection).await?;
        let sealer = VaultSealer::new();
        Ok(Arc::new(Self {
            repository: SqliteEntryRepository::new(connection.clone(), sealer.clone()),
            vault: SqliteVaultStore::new(connection.clone()),
            device_slot: SqliteDeviceSlotStore::new(connection.clone()),
            outbox: SyncOutbox::new(connection),
            sealer: sealer.clone(),
            clock: SystemClock,
            photos: FilePhotoStore::beside(&db_path, sealer),
            exif: KamadakExifReader,
            thumbnails: ImageThumbnailer,
        }))
    }

    /// Creates the one vault this device will ever hold and returns the recovery code once.
    /// The caller must show it and never store it; nothing here writes it down.
    pub async fn create_vault(&self, passphrase: String) -> Result<String, LeafyPuffCoreError> {
        let code = RecoveryCode::generate().map_err(CoreError::from)?;
        let (vault, key) = KeyVault::create(&passphrase, &code).map_err(CoreError::from)?;
        self.vault.create(&vault).await?;
        self.sealer.unlock(key)?;
        Ok(code.to_code_string().to_string())
    }

    pub async fn has_vault(&self) -> Result<bool, LeafyPuffCoreError> {
        Ok(self.vault.exists().await?)
    }

    pub async fn unlock(&self, passphrase: String) -> Result<(), LeafyPuffCoreError> {
        let vault = self.vault.read().await?;
        let key = vault
            .unlock_with_passphrase(&passphrase)
            .map_err(CoreError::from)?;
        self.sealer.unlock(key)?;
        Ok(())
    }

    pub async fn unlock_with_recovery_code(&self, code: String) -> Result<(), LeafyPuffCoreError> {
        let vault = self.vault.read().await?;
        let parsed = RecoveryCode::parse(&code).map_err(CoreError::from)?;
        let key = vault
            .unlock_with_recovery_code(&parsed)
            .map_err(CoreError::from)?;
        self.sealer.unlock(key)?;
        Ok(())
    }

    /// Rewraps the content key under a new passphrase. No entry is re-encrypted.
    pub async fn change_passphrase(
        &self,
        current: String,
        replacement: String,
    ) -> Result<(), LeafyPuffCoreError> {
        let vault = self.vault.read().await?;
        let rewrapped = vault
            .rewrap_passphrase(&current, &replacement)
            .map_err(CoreError::from)?;
        self.vault.replace(&rewrapped).await?;
        Ok(())
    }

    pub fn lock(&self) -> Result<(), LeafyPuffCoreError> {
        self.sealer.lock()?;
        Ok(())
    }

    pub fn is_unlocked(&self) -> bool {
        self.sealer.is_unlocked()
    }
}
