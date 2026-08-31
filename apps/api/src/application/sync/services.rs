use std::sync::Arc;

use crate::domain::sync::{
    CheckpointStore, ConflictSink, EntryStore, IdempotencyStore, WrappedKeyStore,
};

use super::apply_change_set::ApplyChangeSet;
use super::pull_changes::PullChanges;
use super::push_changes::PushChanges;

#[derive(Clone)]
pub struct SyncServices {
    pub entries: Arc<dyn EntryStore>,
    pub checkpoints: Arc<dyn CheckpointStore>,
    pub idempotency: Arc<dyn IdempotencyStore>,
    pub conflicts: Arc<dyn ConflictSink>,
    pub keys: Arc<dyn WrappedKeyStore>,
}

impl SyncServices {
    pub fn pull(&self) -> PullChanges {
        PullChanges::new(Arc::clone(&self.entries), Arc::clone(&self.checkpoints))
    }

    pub fn push(&self) -> PushChanges {
        PushChanges::new(
            ApplyChangeSet::new(Arc::clone(&self.entries), Arc::clone(&self.conflicts)),
            Arc::clone(&self.checkpoints),
            Arc::clone(&self.idempotency),
        )
    }
}
