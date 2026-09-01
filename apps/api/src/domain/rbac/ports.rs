use async_trait::async_trait;
use uuid::Uuid;

use super::audit::AuditEvent;
use super::error::RbacError;
use super::permission::Permission;
use super::role::Role;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn all(&self) -> Result<Vec<Role>, RbacError>;
    async fn for_account(&self, account_id: Uuid) -> Result<Vec<Role>, RbacError>;
    async fn assign(&self, account_id: Uuid, role_id: Uuid) -> Result<(), RbacError>;
    async fn revoke(&self, account_id: Uuid, role_id: Uuid) -> Result<(), RbacError>;
}

#[async_trait]
pub trait PermissionReader: Send + Sync {
    async fn granted(&self, account_id: Uuid) -> Result<Vec<Permission>, RbacError>;
}

#[async_trait]
pub trait AuditLog: Send + Sync {
    async fn record(&self, event: AuditEvent) -> Result<(), RbacError>;
    async fn recent(&self, limit: u64) -> Result<Vec<AuditEvent>, RbacError>;
}
