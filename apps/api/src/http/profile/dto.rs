use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::iam::Profile;

#[derive(Debug, Deserialize)]
pub struct ProfileRequest {
    pub sealed_profile: Option<String>,
    pub avatar_photo_id: Option<Uuid>,
    pub updated_at_ms: i64,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub sealed_profile: Option<String>,
    pub avatar_photo_id: Option<Uuid>,
    pub updated_at_ms: i64,
}

impl From<ProfileRequest> for Profile {
    fn from(request: ProfileRequest) -> Self {
        Self {
            sealed_profile: request.sealed_profile,
            avatar_photo_id: request.avatar_photo_id,
            updated_at_ms: request.updated_at_ms,
        }
    }
}

impl From<Profile> for ProfileResponse {
    fn from(profile: Profile) -> Self {
        Self {
            sealed_profile: profile.sealed_profile,
            avatar_photo_id: profile.avatar_photo_id,
            updated_at_ms: profile.updated_at_ms,
        }
    }
}
