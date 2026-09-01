use crate::http::validated::ValidatedBody;

use super::dto::{SaveCampaignRequest, SetGateRequest};

const MAX_TITLE_LENGTH: usize = 120;
const MAX_BODY_LENGTH: usize = 4096;

impl ValidatedBody for SetGateRequest {
    fn validate(&self) -> Result<(), &'static str> {
        if self.minimum_build < 0 {
            return Err("minimum_build cannot be negative");
        }
        Ok(())
    }
}

impl ValidatedBody for SaveCampaignRequest {
    fn validate(&self) -> Result<(), &'static str> {
        if self.title.trim().is_empty() || self.title.len() > MAX_TITLE_LENGTH {
            return Err("title is empty or too long");
        }
        if self.body.trim().is_empty() || self.body.len() > MAX_BODY_LENGTH {
            return Err("body is empty or too long");
        }
        if self.ends_at_ms <= self.starts_at_ms {
            return Err("the window ends before it starts");
        }
        Ok(())
    }
}
