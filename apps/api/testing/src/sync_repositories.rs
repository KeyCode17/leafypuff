use std::collections::HashMap;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use leafypuff_api::domain::sync::{
    ChangeSet, CheckpointStore, ConflictSink, EntryRecord, EntryStore, FieldConflict,
    IdempotencyStore, SyncCursor, SyncError, WrappedKeyRow, WrappedKeyStore,
};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryEntries {
    rows: Arc<Mutex<Vec<EntryRecord>>>,
    revision: Arc<AtomicI64>,
}

impl InMemoryEntries {
    pub fn snapshot(&self) -> Vec<EntryRecord> {
        self.rows.lock().expect("the entry lock holds").clone()
    }
}

#[async_trait]
impl EntryStore for InMemoryEntries {
    async fn load(
        &self,
        account_id: Uuid,
        entry_id: Uuid,
    ) -> Result<Option<EntryRecord>, SyncError> {
        let rows = self.rows.lock().expect("the entry lock holds");
        Ok(rows
            .iter()
            .find(|row| row.id == entry_id && row.account_id == account_id)
            .cloned())
    }

    async fn changed_since(
        &self,
        account_id: Uuid,
        cursor: SyncCursor,
        limit: u64,
    ) -> Result<ChangeSet, SyncError> {
        let rows = self.rows.lock().expect("the entry lock holds");
        let mut matching: Vec<EntryRecord> = rows
            .iter()
            .filter(|row| row.account_id == account_id && row.revision > cursor.0)
            .cloned()
            .collect();
        matching.sort_by_key(|row| row.revision);
        matching.truncate(usize::try_from(limit).unwrap_or(usize::MAX));

        let advanced = matching
            .iter()
            .fold(cursor, |seen, row| seen.advanced_to(row.revision));
        Ok(ChangeSet {
            records: matching,
            cursor: advanced,
        })
    }

    async fn next_revision(&self) -> Result<i64, SyncError> {
        Ok(self.revision.fetch_add(1, Ordering::SeqCst) + 1)
    }

    async fn upsert(&self, record: EntryRecord) -> Result<(), SyncError> {
        let mut rows = self.rows.lock().expect("the entry lock holds");
        rows.retain(|row| row.id != record.id);
        rows.push(record);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryCheckpoints {
    rows: Arc<Mutex<HashMap<(Uuid, Uuid), i64>>>,
}

#[async_trait]
impl CheckpointStore for InMemoryCheckpoints {
    async fn read(&self, account_id: Uuid, device_id: Uuid) -> Result<SyncCursor, SyncError> {
        let rows = self.rows.lock().expect("the checkpoint lock holds");
        Ok(rows
            .get(&(account_id, device_id))
            .map_or(SyncCursor::START, |cursor| SyncCursor(*cursor)))
    }

    async fn advance(
        &self,
        account_id: Uuid,
        device_id: Uuid,
        cursor: SyncCursor,
    ) -> Result<(), SyncError> {
        let mut rows = self.rows.lock().expect("the checkpoint lock holds");
        rows.insert((account_id, device_id), cursor.0);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryIdempotency {
    rows: Arc<Mutex<HashMap<String, String>>>,
}

#[async_trait]
impl IdempotencyStore for InMemoryIdempotency {
    async fn recall(&self, key: &str) -> Result<Option<String>, SyncError> {
        let rows = self.rows.lock().expect("the idempotency lock holds");
        Ok(rows.get(key).cloned())
    }

    async fn remember(
        &self,
        key: &str,
        _account_id: Uuid,
        _device_id: Uuid,
        response_hash: &str,
    ) -> Result<(), SyncError> {
        let mut rows = self.rows.lock().expect("the idempotency lock holds");
        rows.entry(key.to_owned())
            .or_insert_with(|| response_hash.to_owned());
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryConflicts {
    rows: Arc<Mutex<Vec<FieldConflict>>>,
}

impl InMemoryConflicts {
    pub fn snapshot(&self) -> Vec<FieldConflict> {
        self.rows.lock().expect("the conflict lock holds").clone()
    }
}

#[async_trait]
impl ConflictSink for InMemoryConflicts {
    async fn record(&self, _account_id: Uuid, conflict: FieldConflict) -> Result<(), SyncError> {
        let mut rows = self.rows.lock().expect("the conflict lock holds");
        rows.push(conflict);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryWrappedKeys {
    rows: Arc<Mutex<HashMap<(Uuid, &'static str), WrappedKeyRow>>>,
}

#[async_trait]
impl WrappedKeyStore for InMemoryWrappedKeys {
    async fn read_all(&self, account_id: Uuid) -> Result<Vec<WrappedKeyRow>, SyncError> {
        let rows = self.rows.lock().expect("the key lock holds");
        Ok(rows
            .iter()
            .filter(|((owner, _), _)| *owner == account_id)
            .map(|(_, row)| row.clone())
            .collect())
    }

    async fn put(&self, account_id: Uuid, row: WrappedKeyRow) -> Result<(), SyncError> {
        let mut rows = self.rows.lock().expect("the key lock holds");
        rows.insert((account_id, row.kind.as_str()), row);
        Ok(())
    }
}
