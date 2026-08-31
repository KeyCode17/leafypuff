use async_trait::async_trait;
use uuid::Uuid;

use super::error::MediaError;
use super::media_object::MediaObject;
use super::object_key::ObjectKey;

#[async_trait]
pub trait ObjectStore: Send + Sync {
    async fn put(&self, key: &ObjectKey, ciphertext: Vec<u8>) -> Result<(), MediaError>;
    async fn get(&self, key: &ObjectKey) -> Result<Vec<u8>, MediaError>;
    async fn delete(&self, key: &ObjectKey) -> Result<(), MediaError>;
}

#[async_trait]
pub trait MediaRepository: Send + Sync {
    async fn record(&self, object: MediaObject) -> Result<(), MediaError>;
    async fn find(&self, account_id: Uuid, photo_id: Uuid) -> Result<Vec<MediaObject>, MediaError>;
    async fn forget(&self, account_id: Uuid, photo_id: Uuid) -> Result<(), MediaError>;
}
