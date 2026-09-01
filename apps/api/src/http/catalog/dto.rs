use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct BundleResponse {
    pub id: Uuid,
    pub version: i32,
    pub payload: serde_json::Value,
    pub published: bool,
    pub published_at_ms: Option<i64>,
    pub created_at_ms: i64,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DraftBundleRequest {
    pub payload: serde_json::Value,
}
