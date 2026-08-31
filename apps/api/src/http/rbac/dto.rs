use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct RoleResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Serialize)]
pub struct AuditEventResponse {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub subject_id: Option<Uuid>,
    pub detail: String,
    pub recorded_at_ms: i64,
}

#[derive(Serialize)]
pub struct GrantedResponse {
    pub permissions: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssignRoleRequest {
    pub account_id: Uuid,
    pub role_id: Uuid,
}
