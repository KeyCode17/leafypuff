use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use sea_orm::{ActiveValue, EntityTrait, QueryOrder, QuerySelect};

use crate::domain::rbac::error::ERR_UNKNOWN_ACTION;
use crate::domain::rbac::{AuditAction, AuditEvent, AuditLog, RbacError};

use super::entity::audit_events;
use super::role_repository::storage;

/// Insert and select. There is no update path and no delete path on this adapter, which is what
/// makes the table evidence rather than something an operator could tidy after the fact.
pub struct PgAuditLog {
    connection: DatabaseConnection,
}

impl PgAuditLog {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl AuditLog for PgAuditLog {
    async fn record(&self, event: AuditEvent) -> Result<(), RbacError> {
        audit_events::Entity::insert(audit_events::ActiveModel {
            id: ActiveValue::Set(event.id),
            actor_id: ActiveValue::Set(event.actor_id),
            action: ActiveValue::Set(event.action.as_str().to_owned()),
            subject_id: ActiveValue::Set(event.subject_id),
            detail: ActiveValue::Set(event.detail),
            recorded_at: ActiveValue::Set(event.recorded_at_ms),
        })
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(())
    }

    async fn recent(&self, limit: u64) -> Result<Vec<AuditEvent>, RbacError> {
        let rows = audit_events::Entity::find()
            .order_by_desc(audit_events::Column::RecordedAt)
            .limit(limit)
            .all(&self.connection)
            .await
            .map_err(storage)?;

        rows.into_iter()
            .map(|row| {
                let action = AuditAction::parse(&row.action)
                    .ok_or_else(|| RbacError::Storage(ERR_UNKNOWN_ACTION.to_owned()))?;
                Ok(AuditEvent {
                    id: row.id,
                    actor_id: row.actor_id,
                    action,
                    subject_id: row.subject_id,
                    detail: row.detail,
                    recorded_at_ms: row.recorded_at,
                })
            })
            .collect()
    }
}
