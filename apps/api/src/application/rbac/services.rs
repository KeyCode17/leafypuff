use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::domain::rbac::{
    AuditAction, AuditEvent, AuditLog, Permission, PermissionReader, RbacError, Role,
    RoleRepository,
};

pub const AUDIT_PAGE_SIZE: u64 = 200;

#[derive(Clone)]
pub struct RbacServices {
    pub roles: Arc<dyn RoleRepository>,
    pub permissions: Arc<dyn PermissionReader>,
    pub audit: Arc<dyn AuditLog>,
}

impl RbacServices {
    /// The one place a permission is checked. Every handler that changes state calls this as its
    /// first line, so a route added later cannot forget by omission — it has no other way in.
    pub async fn require(&self, account_id: Uuid, permission: Permission) -> Result<(), RbacError> {
        let granted = self.permissions.granted(account_id).await?;
        if granted.contains(&permission) {
            return Ok(());
        }
        Err(RbacError::Forbidden)
    }

    pub async fn granted(&self, account_id: Uuid) -> Result<Vec<Permission>, RbacError> {
        self.permissions.granted(account_id).await
    }

    pub async fn all_roles(&self) -> Result<Vec<Role>, RbacError> {
        self.roles.all().await
    }

    pub async fn assign(
        &self,
        actor_id: Uuid,
        account_id: Uuid,
        role_id: Uuid,
    ) -> Result<(), RbacError> {
        self.require(actor_id, Permission::RoleWrite).await?;
        self.roles.assign(account_id, role_id).await?;
        self.record(actor_id, AuditAction::RoleAssigned, role_id)
            .await
    }

    pub async fn revoke(
        &self,
        actor_id: Uuid,
        account_id: Uuid,
        role_id: Uuid,
    ) -> Result<(), RbacError> {
        self.require(actor_id, Permission::RoleWrite).await?;
        self.roles.revoke(account_id, role_id).await?;
        self.record(actor_id, AuditAction::RoleRevoked, role_id)
            .await
    }

    pub async fn recent_events(&self, actor_id: Uuid) -> Result<Vec<AuditEvent>, RbacError> {
        self.require(actor_id, Permission::AuditRead).await?;
        self.audit.recent(AUDIT_PAGE_SIZE).await
    }

    async fn record(
        &self,
        actor_id: Uuid,
        action: AuditAction,
        role_id: Uuid,
    ) -> Result<(), RbacError> {
        self.audit
            .record(AuditEvent {
                id: Uuid::new_v4(),
                actor_id,
                action,
                subject_id: None,
                detail: role_id.to_string(),
                recorded_at_ms: Utc::now().timestamp_millis(),
            })
            .await
    }
}
