pub mod enums;
pub mod error;
pub mod records;

use std::sync::Arc;

use chrono::NaiveDate;

use crate::application::SaveEntry;
use crate::domain::error::ERR_DATE_UNREADABLE;
use crate::domain::{CoreError, Entry, EntryId, EntryRepository};
use crate::infrastructure::{SqliteEntryRepository, SystemClock, db};

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
    repository: SqliteEntryRepository,
    clock: SystemClock,
}

#[uniffi::export(async_runtime = "tokio")]
impl LeafyPuffCore {
    #[uniffi::constructor(name = "open")]
    pub async fn new(db_path: String) -> Result<Arc<Self>, LeafyPuffCoreError> {
        let connection = db::open(&db_path).await?;
        db::run_migrations(&connection).await?;
        Ok(Arc::new(Self {
            repository: SqliteEntryRepository::new(connection),
            clock: SystemClock,
        }))
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
