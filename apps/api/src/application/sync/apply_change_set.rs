use std::sync::Arc;

use uuid::Uuid;

use crate::domain::sync::{
    ConflictSink, EncryptedField, EntryRecord, EntryStore, SyncError, resolve_field,
};

pub struct ApplyChangeSet {
    entries: Arc<dyn EntryStore>,
    conflicts: Arc<dyn ConflictSink>,
}

impl ApplyChangeSet {
    pub const fn new(entries: Arc<dyn EntryStore>, conflicts: Arc<dyn ConflictSink>) -> Self {
        Self { entries, conflicts }
    }

    pub async fn execute(&self, account_id: Uuid, incoming: EntryRecord) -> Result<i64, SyncError> {
        if incoming.account_id != account_id {
            return Err(SyncError::Forbidden);
        }
        let held = self.entries.load(account_id, incoming.id).await?;

        let title = resolve_field(
            incoming.id,
            EncryptedField::Title,
            held.as_ref().map(|row| row.title.clone()),
            incoming.title.clone(),
        );
        let body = resolve_field(
            incoming.id,
            EncryptedField::Body,
            held.as_ref().map(|row| row.body.clone()),
            incoming.body.clone(),
        );

        let incoming_carries_a_winner = title.winner.device_id == incoming.title.device_id
            && title.winner.updated_at_ms == incoming.title.updated_at_ms
            || body.winner.device_id == incoming.body.device_id
                && body.winner.updated_at_ms == incoming.body.updated_at_ms;

        let revision = self.entries.next_revision().await?;
        let merged = EntryRecord {
            revision,
            deleted_at_ms: tombstone(held.as_ref(), &incoming),
            title: title.winner,
            body: body.winner,
            ..metadata(held, incoming, incoming_carries_a_winner)
        };
        self.entries.upsert(merged).await?;

        for conflict in [title.conflict, body.conflict].into_iter().flatten() {
            self.conflicts.record(account_id, conflict).await?;
        }
        Ok(revision)
    }
}

fn metadata(
    held: Option<EntryRecord>,
    incoming: EntryRecord,
    incoming_carries_a_winner: bool,
) -> EntryRecord {
    match held {
        Some(held) if !incoming_carries_a_winner => held,
        _ => incoming,
    }
}

fn tombstone(held: Option<&EntryRecord>, incoming: &EntryRecord) -> Option<i64> {
    match (
        held.and_then(|row| row.deleted_at_ms),
        incoming.deleted_at_ms,
    ) {
        (Some(held_at), Some(incoming_at)) => Some(held_at.min(incoming_at)),
        (Some(held_at), None) => Some(held_at),
        (None, incoming_at) => incoming_at,
    }
}
