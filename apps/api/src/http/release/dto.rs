use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct GateResponse {
    pub platform: String,
    pub minimum_build: i32,
    pub force_update: bool,
    pub behind: bool,
    pub blocked: bool,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct CampaignResponse {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub platform: String,
    pub starts_at_ms: i64,
    pub ends_at_ms: i64,
    pub published: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetGateRequest {
    pub platform: String,
    pub minimum_build: i32,
    pub force_update: bool,
    pub message: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveCampaignRequest {
    pub id: Option<Uuid>,
    pub title: String,
    pub body: String,
    pub platform: String,
    pub starts_at_ms: i64,
    pub ends_at_ms: i64,
    pub published: bool,
}
