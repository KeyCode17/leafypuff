use crate::http::validated::ValidatedBody;

use super::dto::{PushRequest, WrappedKeyRequest};

const MAX_RECORDS_PER_BATCH: usize = 200;
const MAX_BLOB_BASE64_LENGTH: usize = 4096;

impl ValidatedBody for PushRequest {
    fn validate(&self) -> Result<(), &'static str> {
        if self.records.is_empty() {
            return Err("records is empty");
        }
        if self.records.len() > MAX_RECORDS_PER_BATCH {
            return Err("records is over the batch ceiling");
        }
        Ok(())
    }
}

impl ValidatedBody for WrappedKeyRequest {
    fn validate(&self) -> Result<(), &'static str> {
        if self.blob.is_empty() || self.blob.len() > MAX_BLOB_BASE64_LENGTH {
            return Err("blob is empty or over the ceiling");
        }
        if self.salt.is_empty() || self.salt.len() > MAX_BLOB_BASE64_LENGTH {
            return Err("salt is empty or over the ceiling");
        }
        Ok(())
    }
}
