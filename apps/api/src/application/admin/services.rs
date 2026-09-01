use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::domain::admin::{AccountDirectory, AccountSummary, AdminError, DIRECTORY_PAGE_SIZE};
use crate::domain::rbac::{AuditAction, AuditEvent, AuditLog, Permission, RbacError};

use crate::application::rbac::RbacServices;

#[derive(Clone)]
pub struct AdminServices {
    pub directory: Arc<dyn AccountDirectory>,
    pub audit: Arc<dyn AuditLog>,
    pub rbac: RbacServices,
}

impl AdminServices {
    pub async fn list(&self, actor_id: Uuid) -> Result<Vec<AccountSummary>, AdminError> {
        self.permitted(actor_id, Permission::AccountList).await?;
        self.directory.summaries(DIRECTORY_PAGE_SIZE).await
    }

    pub async fn read(
        &self,
        actor_id: Uuid,
        account_id: Uuid,
    ) -> Result<AccountSummary, AdminError> {
        self.permitted(actor_id, Permission::AccountRead).await?;
        self.directory.summary(account_id).await
    }

    pub async fn suspend(&self, actor_id: Uuid, account_id: Uuid) -> Result<(), AdminError> {
        self.permitted(actor_id, Permission::AccountSuspend).await?;
        self.directory
            .set_suspended(account_id, Some(Utc::now()))
            .await?;
        self.record(actor_id, AuditAction::AccountSuspended, account_id)
            .await
    }

    pub async fn restore(&self, actor_id: Uuid, account_id: Uuid) -> Result<(), AdminError> {
        self.permitted(actor_id, Permission::AccountRestore).await?;
        self.directory.set_suspended(account_id, None).await?;
        self.record(actor_id, AuditAction::AccountRestored, account_id)
            .await
    }

    async fn permitted(&self, actor_id: Uuid, permission: Permission) -> Result<(), AdminError> {
        self.rbac
            .require(actor_id, permission)
            .await
            .map_err(rbac_to_admin)
    }

    async fn record(
        &self,
        actor_id: Uuid,
        action: AuditAction,
        account_id: Uuid,
    ) -> Result<(), AdminError> {
        self.audit
            .record(AuditEvent {
                id: Uuid::new_v4(),
                actor_id,
                action,
                subject_id: None,
                detail: account_id.to_string(),
                recorded_at_ms: Utc::now().timestamp_millis(),
            })
            .await
            .map_err(rbac_to_admin)
    }
}

fn rbac_to_admin(error: RbacError) -> AdminError {
    match error {
        RbacError::Forbidden => AdminError::Forbidden,
        RbacError::RoleNotFound => AdminError::AccountNotFound,
        RbacError::Storage(reason) => AdminError::Storage(reason),
    }
}
