use std::sync::Mutex;

use chrono::{DateTime, NaiveDate, Utc};

use crate::domain::error::ERR_STORE_LOCK_POISONED;
use crate::domain::{Clock, CoreError, Entry, EntryId, EntryRepository};

#[derive(Debug, Clone, Copy)]
pub struct FixedClock {
    at: DateTime<Utc>,
}

impl FixedClock {
    pub const fn new(at: DateTime<Utc>) -> Self {
        Self { at }
    }
}

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.at
    }
}

#[derive(Default)]
pub struct InMemoryEntryRepository {
    entries: Mutex<Vec<Entry>>,
}

impl InMemoryEntryRepository {
    pub fn new() -> Self {
        Self::default()
    }

    fn read(&self) -> Result<Vec<Entry>, CoreError> {
        let held = self
            .entries
            .lock()
            .map_err(|_| CoreError::Storage(ERR_STORE_LOCK_POISONED.to_owned()))?;
        let mut all = held.clone();
        all.sort_by(|left, right| {
            right
                .date
                .cmp(&left.date)
                .then(right.created_at.cmp(&left.created_at))
        });
        Ok(all)
    }
}

impl EntryRepository for InMemoryEntryRepository {
    async fn save(&self, entry: Entry) -> Result<Entry, CoreError> {
        let mut held = self
            .entries
            .lock()
            .map_err(|_| CoreError::Storage(ERR_STORE_LOCK_POISONED.to_owned()))?;
        held.retain(|other| other.id != entry.id);
        held.push(entry.clone());
        Ok(entry)
    }

    async fn by_id(&self, id: EntryId) -> Result<Option<Entry>, CoreError> {
        Ok(self.read()?.into_iter().find(|entry| entry.id == id))
    }

    async fn list_desc(&self, limit: u32) -> Result<Vec<Entry>, CoreError> {
        Ok(self.read()?.into_iter().take(limit as usize).collect())
    }

    async fn in_range(&self, from: NaiveDate, to: NaiveDate) -> Result<Vec<Entry>, CoreError> {
        Ok(self
            .read()?
            .into_iter()
            .filter(|entry| entry.date >= from && entry.date <= to)
            .collect())
    }

    async fn on_date(&self, date: NaiveDate) -> Result<Vec<Entry>, CoreError> {
        Ok(self
            .read()?
            .into_iter()
            .filter(|entry| entry.date == date)
            .collect())
    }

    async fn delete_all(&self) -> Result<(), CoreError> {
        let mut held = self
            .entries
            .lock()
            .map_err(|_| CoreError::Storage(ERR_STORE_LOCK_POISONED.to_owned()))?;
        held.clear();
        Ok(())
    }
}
