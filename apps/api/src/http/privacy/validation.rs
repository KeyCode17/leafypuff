use crate::http::validated::ValidatedBody;

use super::dto::RaiseRequest;

const MAX_EMAIL_LENGTH: usize = 254;

impl ValidatedBody for RaiseRequest {
    fn validate(&self) -> Result<(), &'static str> {
        if self.kind.trim().is_empty() {
            return Err("kind is empty");
        }
        match &self.email {
            Some(address) if address.len() > MAX_EMAIL_LENGTH => Err("email is too long"),
            _ => Ok(()),
        }
    }
}
