pub mod enums;
pub mod error;
pub mod photos;
pub mod records;

use std::sync::Arc;

use chrono::NaiveDate;

use crate::application::SaveEntry;
use crate::domain::crypto::{KeyVault, RecoveryCode};
use crate::domain::error::ERR_DATE_UNREADABLE;
use crate::domain::{CoreError, Entry, EntryId, EntryRepository};
use crate::infrastructure::{
    FilePhotoStore, ImageThumbnailer, KamadakExifReader, SqliteEntryRepository, SqliteVaultStore,
    SystemClock, VaultSealer, db,
};

pub use error::LeafyPuffCoreError;
pub use records::{FfiEntry, FfiPhoto, FfiPlacedSticker};

use records::ISO_DATE;

fn read_date(raw: &str) -> Result<NaiveDate, CoreError> {
    NaiveDate::parse_from_str(raw, ISO_DATE)
        .map_err(|_| CoreError::Invalid(format!("{ERR_DATE_UNREADABLE}: {raw}")))
}

/// The only handle Kotlin holds. It owns the connection and every adapter behind it.
#[derive(uniffi::Object)]
pub struct LeafyPuffCore {
    repository: SqliteEntryRepository<VaultSealer>,
    vault: SqliteVaultStore,
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
            vault: SqliteVaultStore::new(connection),
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

    pub async fn save_entry(&self, entry: FfiEntry) -> Result<FfiEntry, LeafyPuffCoreError> {
        let draft = Entry::try_from(entry)?;
        let saved = SaveEntry::new(&self.repository, self.clock)
            .execute(draft)
            .await?;
        Ok(FfiEntry::from(saved))
    }

    pub async fn entry_by_id(&self, id: String) -> Result<Option<FfiEntry>, LeafyPuffCoreError> {
        let found = self.repository.by_id(EntryId::parse(&id)?).await?;
        Ok(found.map(FfiEntry::from))
    }

    pub async fn list_entries(&self, limit: u32) -> Result<Vec<FfiEntry>, LeafyPuffCoreError> {
        let found = self.repository.list_desc(limit).await?;
        Ok(found.into_iter().map(FfiEntry::from).collect())
    }

    pub async fn entries_in_range(
        &self,
        from: String,
        to: String,
    ) -> Result<Vec<FfiEntry>, LeafyPuffCoreError> {
        let found = self
            .repository
            .in_range(read_date(&from)?, read_date(&to)?)
            .await?;
        Ok(found.into_iter().map(FfiEntry::from).collect())
    }

    pub async fn entry_on_date(&self, date: String) -> Result<Vec<FfiEntry>, LeafyPuffCoreError> {
        let found = self.repository.on_date(read_date(&date)?).await?;
        Ok(found.into_iter().map(FfiEntry::from).collect())
    }

    pub async fn delete_all_entries(&self) -> Result<(), LeafyPuffCoreError> {
        self.repository.delete_all().await?;
        Ok(())
    }
}
