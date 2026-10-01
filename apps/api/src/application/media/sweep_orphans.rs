use std::sync::Arc;

use crate::domain::media::{MediaError, MediaRepository, ObjectKey, ObjectStore, Variant};

const SWEEP_BATCH: u64 = 500;

pub struct SweepOrphanedMedia {
    objects: Arc<dyn ObjectStore>,
    media: Arc<dyn MediaRepository>,
}

impl SweepOrphanedMedia {
    pub const fn new(objects: Arc<dyn ObjectStore>, media: Arc<dyn MediaRepository>) -> Self {
        Self { objects, media }
    }

    pub async fn execute(&self) -> Result<usize, MediaError> {
        let mut forgotten = 0;
        loop {
            let batch = self.media.orphaned_photos(SWEEP_BATCH).await?;
            if batch.is_empty() {
                return Ok(forgotten);
            }
            for (account_id, photo_id) in &batch {
                for variant in Variant::ALL {
                    self.objects
                        .delete(&ObjectKey::new(*account_id, *photo_id, variant))
                        .await?;
                }
                self.media.forget(*account_id, *photo_id).await?;
                forgotten += 1;
            }
            if (batch.len() as u64) < SWEEP_BATCH {
                return Ok(forgotten);
            }
        }
    }
}
