use std::sync::Arc;

use uuid::Uuid;

use crate::domain::media::{MediaError, ObjectKey, ObjectStore, Variant};

pub struct StoreAvatar {
    objects: Arc<dyn ObjectStore>,
}

impl StoreAvatar {
    pub const fn new(objects: Arc<dyn ObjectStore>) -> Self {
        Self { objects }
    }

    pub async fn execute(
        &self,
        account_id: Uuid,
        photo_id: Uuid,
        variant: Variant,
        ciphertext: Vec<u8>,
    ) -> Result<(), MediaError> {
        self.objects
            .put(&ObjectKey::new(account_id, photo_id, variant), ciphertext)
            .await
    }
}
