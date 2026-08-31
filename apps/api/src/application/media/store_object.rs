use std::sync::Arc;

use uuid::Uuid;

use crate::domain::media::{
    MediaError, MediaObject, MediaRepository, ObjectKey, ObjectStore, Variant,
};
use crate::domain::sync::fingerprint;

pub struct StoreObject {
    objects: Arc<dyn ObjectStore>,
    media: Arc<dyn MediaRepository>,
}

impl StoreObject {
    pub const fn new(objects: Arc<dyn ObjectStore>, media: Arc<dyn MediaRepository>) -> Self {
        Self { objects, media }
    }

    pub async fn execute(
        &self,
        account_id: Uuid,
        entry_id: Uuid,
        photo_id: Uuid,
        variant: Variant,
        ciphertext: Vec<u8>,
    ) -> Result<(), MediaError> {
        let byte_len = i64::try_from(ciphertext.len()).unwrap_or(i64::MAX);
        let ciphertext_hash = fingerprint(&ciphertext);
        let key = ObjectKey::new(account_id, photo_id, variant);

        self.objects.put(&key, ciphertext).await?;
        self.media
            .record(MediaObject {
                photo_id,
                account_id,
                entry_id,
                variant,
                byte_len,
                ciphertext_hash,
                created_at_ms: chrono::Utc::now().timestamp_millis(),
            })
            .await
    }
}
