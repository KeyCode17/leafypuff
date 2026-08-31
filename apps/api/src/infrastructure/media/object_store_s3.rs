use async_trait::async_trait;
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;

use crate::domain::media::{MAX_OBJECT_BYTES, MediaError, ObjectKey, ObjectStore};

pub struct S3ObjectStore {
    client: Client,
    bucket: String,
}

impl S3ObjectStore {
    pub const fn new(client: Client, bucket: String) -> Self {
        Self { client, bucket }
    }
}

#[async_trait]
impl ObjectStore for S3ObjectStore {
    async fn put(&self, key: &ObjectKey, ciphertext: Vec<u8>) -> Result<(), MediaError> {
        if ciphertext.len() > MAX_OBJECT_BYTES {
            return Err(MediaError::TooLarge);
        }
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key.to_string())
            .body(ByteStream::from(ciphertext))
            .send()
            .await
            .map_err(|error| MediaError::Storage(error.to_string()))?;
        Ok(())
    }

    async fn get(&self, key: &ObjectKey) -> Result<Vec<u8>, MediaError> {
        let object = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key.to_string())
            .send()
            .await
            .map_err(|error| match error.into_service_error() {
                aws_sdk_s3::operation::get_object::GetObjectError::NoSuchKey(_) => {
                    MediaError::NotFound
                }
                other => MediaError::Storage(other.to_string()),
            })?;
        let bytes = object
            .body
            .collect()
            .await
            .map_err(|error| MediaError::Storage(error.to_string()))?;
        Ok(bytes.to_vec())
    }

    async fn delete(&self, key: &ObjectKey) -> Result<(), MediaError> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key.to_string())
            .send()
            .await
            .map_err(|error| MediaError::Storage(error.to_string()))?;
        Ok(())
    }
}
