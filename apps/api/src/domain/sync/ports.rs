use async_trait::async_trait;
use uuid::Uuid;

use super::change_set::{ChangeSet, WrappedKeyRow};
use super::conflict::FieldConflict;
use super::cursor::SyncCursor;
use super::entry_record::EntryRecord;
use super::error::SyncError;

#[async_trait]
pub trait EntryStore: Send + Sync {
    async fn load(
        &self,
        account_id: Uuid,
        entry_id: Uuid,
    ) -> Result<Option<EntryRecord>, SyncError>;
    async fn changed_since(
        &self,
        account_id: Uuid,
        cursor: SyncCursor,
        limit: u64,
    ) -> Result<ChangeSet, SyncError>;
    async fn next_revision(&self) -> Result<i64, SyncError>;
    async fn upsert(&self, record: EntryRecord) -> Result<(), SyncError>;
}

#[async_trait]
pub trait CheckpointStore: Send + Sync {
    async fn read(&self, account_id: Uuid, device_id: Uuid) -> Result<SyncCursor, SyncError>;
    async fn advance(
        &self,
        account_id: Uuid,
        device_id: Uuid,
        cursor: SyncCursor,
    ) -> Result<(), SyncError>;
}

#[async_trait]
pub trait IdempotencyStore: Send + Sync {
    async fn recall(&self, key: &str) -> Result<Option<String>, SyncError>;
    async fn remember(
        &self,
        key: &str,
        account_id: Uuid,
        device_id: Uuid,
        response_hash: &str,
    ) -> Result<(), SyncError>;
}

#[async_trait]
pub trait ConflictSink: Send + Sync {
    async fn record(&self, account_id: Uuid, conflict: FieldConflict) -> Result<(), SyncError>;
}

#[async_trait]
pub trait WrappedKeyStore: Send + Sync {
    async fn read_all(&self, account_id: Uuid) -> Result<Vec<WrappedKeyRow>, SyncError>;
    async fn put(&self, account_id: Uuid, row: WrappedKeyRow) -> Result<(), SyncError>;
}
