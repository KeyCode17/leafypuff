use std::sync::Arc;

use uuid::Uuid;

use crate::domain::media::{MediaError, MediaRepository, ObjectKey, ObjectStore, Variant};

pub struct ForgetObject {
    objects: Arc<dyn ObjectStore>,
    media: Arc<dyn MediaRepository>,
}

impl ForgetObject {
    pub const fn new(objects: Arc<dyn ObjectStore>, media: Arc<dyn MediaRepository>) -> Self {
        Self { objects, media }
    }

    pub async fn execute(&self, account_id: Uuid, photo_id: Uuid) -> Result<(), MediaError> {
        for variant in Variant::ALL {
            self.objects
                .delete(&ObjectKey::new(account_id, photo_id, variant))
                .await?;
        }
        self.media.forget(account_id, photo_id).await
    }
}
