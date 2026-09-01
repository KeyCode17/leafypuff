use crate::http::validated::ValidatedBody;

use super::dto::DraftBundleRequest;

impl ValidatedBody for DraftBundleRequest {
    fn validate(&self) -> Result<(), &'static str> {
        if !self.payload.is_object() {
            return Err("payload must be a json object");
        }
        Ok(())
    }
}
