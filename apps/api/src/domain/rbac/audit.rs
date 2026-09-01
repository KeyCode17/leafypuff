use std::fmt;

use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditAction {
    RoleAssigned,
    RoleRevoked,
    AccountSuspended,
    AccountRestored,
    CatalogPublished,
    ReleaseGateChanged,
    DataRequestFulfilled,
    SyncConflictRecorded,
}

impl AuditAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoleAssigned => "role.assigned",
            Self::RoleRevoked => "role.revoked",
            Self::AccountSuspended => "account.suspended",
            Self::AccountRestored => "account.restored",
            Self::CatalogPublished => "catalog.published",
            Self::ReleaseGateChanged => "release.gate_changed",
            Self::DataRequestFulfilled => "data_request.fulfilled",
            Self::SyncConflictRecorded => "sync.conflict_recorded",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        [
            Self::RoleAssigned,
            Self::RoleRevoked,
            Self::AccountSuspended,
            Self::AccountRestored,
            Self::CatalogPublished,
            Self::ReleaseGateChanged,
            Self::DataRequestFulfilled,
            Self::SyncConflictRecorded,
        ]
        .into_iter()
        .find(|held| held.as_str() == raw)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub id: Uuid,
    pub actor_id: Uuid,
    pub action: AuditAction,
    pub subject_id: Option<Uuid>,
    pub detail: String,
    pub recorded_at_ms: i64,
}

impl fmt::Debug for AuditEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AuditEvent")
            .field("id", &self.id)
            .field("actor_id", &self.actor_id)
            .field("action", &self.action)
            .field("subject_id", &self.subject_id)
            .field("detail_len", &self.detail.len())
            .field("recorded_at_ms", &self.recorded_at_ms)
            .finish()
    }
}
