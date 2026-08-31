use std::sync::Mutex;

use crate::domain::{CoreError, Entry, EntryId, EntryRepository};

#[derive(Default)]
pub struct InMemoryEntryRepository {
    entries: Mutex<Vec<Entry>>,
}

impl InMemoryEntryRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl EntryRepository for InMemoryEntryRepository {
    async fn save(&self, entry: Entry) -> Result<Entry, CoreError> {
        let mut entries = self
            .entries
            .lock()
            .map_err(|_| CoreError::Storage("Entry store lock poisoned".to_owned()))?;
        entries.retain(|held| held.id != entry.id);
        entries.push(entry.clone());
        Ok(entry)
    }

    async fn find(&self, id: EntryId) -> Result<Option<Entry>, CoreError> {
        let entries = self
            .entries
            .lock()
            .map_err(|_| CoreError::Storage("Entry store lock poisoned".to_owned()))?;
        Ok(entries.iter().find(|held| held.id == id).cloned())
    }
}
