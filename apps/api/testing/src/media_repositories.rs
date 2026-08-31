use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use leafypuff_api::domain::media::{
    MediaError, MediaObject, MediaRepository, ObjectKey, ObjectStore,
};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryObjects {
    rows: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl InMemoryObjects {
    pub fn keys(&self) -> Vec<String> {
        let rows = self.rows.lock().expect("the object lock holds");
        let mut keys: Vec<String> = rows.keys().cloned().collect();
        keys.sort();
        keys
    }
}

#[async_trait]
impl ObjectStore for InMemoryObjects {
    async fn put(&self, key: &ObjectKey, ciphertext: Vec<u8>) -> Result<(), MediaError> {
        let mut rows = self.rows.lock().expect("the object lock holds");
        rows.insert(key.to_string(), ciphertext);
        Ok(())
    }

    async fn get(&self, key: &ObjectKey) -> Result<Vec<u8>, MediaError> {
        let rows = self.rows.lock().expect("the object lock holds");
        rows.get(&key.to_string())
            .cloned()
            .ok_or(MediaError::NotFound)
    }

    async fn delete(&self, key: &ObjectKey) -> Result<(), MediaError> {
        let mut rows = self.rows.lock().expect("the object lock holds");
        rows.remove(&key.to_string());
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryMedia {
    rows: Arc<Mutex<Vec<MediaObject>>>,
}

#[async_trait]
impl MediaRepository for InMemoryMedia {
    async fn record(&self, object: MediaObject) -> Result<(), MediaError> {
        let mut rows = self.rows.lock().expect("the media lock holds");
        rows.retain(|row| row.photo_id != object.photo_id || row.variant != object.variant);
        rows.push(object);
        Ok(())
    }

    async fn find(&self, account_id: Uuid, photo_id: Uuid) -> Result<Vec<MediaObject>, MediaError> {
        let rows = self.rows.lock().expect("the media lock holds");
        Ok(rows
            .iter()
            .filter(|row| row.account_id == account_id && row.photo_id == photo_id)
            .cloned()
            .collect())
    }

    async fn forget(&self, account_id: Uuid, photo_id: Uuid) -> Result<(), MediaError> {
        let mut rows = self.rows.lock().expect("the media lock holds");
        rows.retain(|row| row.account_id != account_id || row.photo_id != photo_id);
        Ok(())
    }
}
