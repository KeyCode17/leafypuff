use crate::application::{ExportDiary, SaveEntry};
use crate::domain::{CoreError, Entry, EntryId, EntryRepository, StatsRange, summarise};

use super::error::LeafyPuffCoreError;
use super::records::FfiEntry;
use super::stats_records::{FfiStats, FfiStatsRange};
use super::{LeafyPuffCore, read_date};

/// Statistics and the export both read the whole diary; neither has another bound.
const WHOLE_DIARY: u32 = u32::MAX;

const ERR_EXPORT_WRITE: &str = "The archive could not be saved";

#[uniffi::export(async_runtime = "tokio")]
impl LeafyPuffCore {
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

    /// Writes the whole diary to `destination` as a zip and answers the path. The bytes are
    /// plaintext by design: an export the owner cannot open is not an export.
    pub async fn export_diary(&self, destination: String) -> Result<String, LeafyPuffCoreError> {
        let archive = ExportDiary::new(&self.repository, &self.photos)
            .execute(WHOLE_DIARY)
            .await?;
        std::fs::write(&destination, archive)
            .map_err(|error| CoreError::Storage(format!("{ERR_EXPORT_WRITE}: {error}")))?;
        Ok(destination)
    }

    /// Every figure the Statistics screen draws. The definitions live here rather than in Kotlin
    /// because the streak walks calendar days, which is a rule about the data and not the view.
    pub async fn statistics(
        &self,
        range: FfiStatsRange,
        today: String,
    ) -> Result<FfiStats, LeafyPuffCoreError> {
        let entries = self.repository.list_desc(WHOLE_DIARY).await?;
        Ok(FfiStats::from(summarise(
            &entries,
            StatsRange::from(range),
            read_date(&today)?,
        )))
    }
}
