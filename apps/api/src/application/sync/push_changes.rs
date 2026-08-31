use std::sync::Arc;

use serde::Serialize;
use uuid::Uuid;

use crate::domain::sync::{
    CheckpointStore, EntryRecord, IdempotencyStore, SyncCursor, SyncError, fingerprint,
};

use super::apply_change_set::ApplyChangeSet;

#[derive(Debug, Clone, Serialize)]
pub struct PushReceipt {
    pub cursor: i64,
    pub applied: Vec<Uuid>,
    pub replayed: bool,
}

pub struct PushChanges {
    apply: ApplyChangeSet,
    checkpoints: Arc<dyn CheckpointStore>,
    idempotency: Arc<dyn IdempotencyStore>,
}

impl PushChanges {
    pub const fn new(
        apply: ApplyChangeSet,
        checkpoints: Arc<dyn CheckpointStore>,
        idempotency: Arc<dyn IdempotencyStore>,
    ) -> Self {
        Self {
            apply,
            checkpoints,
            idempotency,
        }
    }

    pub async fn execute(
        &self,
        account_id: Uuid,
        device_id: Uuid,
        idempotency_key: &str,
        records: Vec<EntryRecord>,
    ) -> Result<PushReceipt, SyncError> {
        if self.idempotency.recall(idempotency_key).await?.is_some() {
            let cursor = self.checkpoints.read(account_id, device_id).await?;
            return Ok(PushReceipt {
                cursor: cursor.0,
                applied: Vec::new(),
                replayed: true,
            });
        }

        let mut cursor = self.checkpoints.read(account_id, device_id).await?;
        let mut applied = Vec::with_capacity(records.len());
        for record in records {
            let id = record.id;
            let revision = self.apply.execute(account_id, record).await?;
            cursor = cursor.advanced_to(revision);
            applied.push(id);
        }

        self.checkpoints
            .advance(account_id, device_id, SyncCursor(cursor.0))
            .await?;
        let receipt = PushReceipt {
            cursor: cursor.0,
            applied,
            replayed: false,
        };
        self.idempotency
            .remember(
                idempotency_key,
                account_id,
                device_id,
                &receipt_hash(&receipt),
            )
            .await?;
        Ok(receipt)
    }
}

fn receipt_hash(receipt: &PushReceipt) -> String {
    let mut material = receipt.cursor.to_be_bytes().to_vec();
    for id in &receipt.applied {
        material.extend_from_slice(id.as_bytes());
    }
    fingerprint(&material)
}
