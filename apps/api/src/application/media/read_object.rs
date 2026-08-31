use std::sync::Arc;

use uuid::Uuid;

use crate::domain::media::{MediaError, MediaRepository, ObjectKey, ObjectStore, Variant};

pub struct ReadObject {
    objects: Arc<dyn ObjectStore>,
    media: Arc<dyn MediaRepository>,
}

impl ReadObject {
    pub const fn new(objects: Arc<dyn ObjectStore>, media: Arc<dyn MediaRepository>) -> Self {
        Self { objects, media }
    }

    pub async fn execute(
        &self,
        account_id: Uuid,
        photo_id: Uuid,
        variant: Variant,
    ) -> Result<Vec<u8>, MediaError> {
        let known = self.media.find(account_id, photo_id).await?;
        if !known.iter().any(|object| object.variant == variant) {
            return Err(MediaError::NotFound);
        }
        self.objects
            .get(&ObjectKey::new(account_id, photo_id, variant))
            .await
    }
}
