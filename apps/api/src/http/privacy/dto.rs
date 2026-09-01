use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct DataRequestResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub email: Option<String>,
    pub kind: String,
    pub status: String,
    pub requested_at_ms: i64,
    pub fulfilled_at_ms: Option<i64>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RaiseRequest {
    pub kind: String,
    pub email: Option<String>,
}
