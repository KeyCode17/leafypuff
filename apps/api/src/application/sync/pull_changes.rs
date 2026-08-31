use std::sync::Arc;

use uuid::Uuid;

use crate::domain::sync::{ChangeSet, CheckpointStore, EntryStore, SyncCursor, SyncError};

pub const PULL_PAGE_SIZE: u64 = 200;

pub struct PullChanges {
    entries: Arc<dyn EntryStore>,
    checkpoints: Arc<dyn CheckpointStore>,
}

impl PullChanges {
    pub const fn new(entries: Arc<dyn EntryStore>, checkpoints: Arc<dyn CheckpointStore>) -> Self {
        Self {
            entries,
            checkpoints,
        }
    }

    pub async fn execute(
        &self,
        account_id: Uuid,
        device_id: Uuid,
        cursor: Option<SyncCursor>,
    ) -> Result<ChangeSet, SyncError> {
        let from = match cursor {
            Some(given) => given,
            None => self.checkpoints.read(account_id, device_id).await?,
        };
        let changes = self
            .entries
            .changed_since(account_id, from, PULL_PAGE_SIZE)
            .await?;
        self.checkpoints
            .advance(account_id, device_id, changes.cursor)
            .await?;
        Ok(changes)
    }
}
