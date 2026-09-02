use std::sync::Arc;

use uuid::Uuid;

use crate::domain::media::{MediaError, ObjectKey, ObjectStore, Variant};

pub struct ReadAvatar {
    objects: Arc<dyn ObjectStore>,
}

impl ReadAvatar {
    pub const fn new(objects: Arc<dyn ObjectStore>) -> Self {
        Self { objects }
    }

    pub async fn execute(
        &self,
        account_id: Uuid,
        photo_id: Uuid,
        variant: Variant,
    ) -> Result<Vec<u8>, MediaError> {
        self.objects
            .get(&ObjectKey::new(account_id, photo_id, variant))
            .await
    }
}
