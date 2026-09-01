use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::application::rbac::RbacServices;
use crate::domain::media::ObjectStore;
use crate::domain::privacy::{
    DataRequest, DataRequestStore, Eraser, PrivacyError, RequestKind, RequestStatus,
};
use crate::domain::rbac::{AuditAction, AuditEvent, AuditLog, Permission, RbacError};

#[derive(Clone)]
pub struct PrivacyServices {
    pub requests: Arc<dyn DataRequestStore>,
    pub eraser: Arc<dyn Eraser>,
    pub objects: Arc<dyn ObjectStore>,
    pub audit: Arc<dyn AuditLog>,
    pub rbac: RbacServices,
}

impl PrivacyServices {
    pub async fn raise(
        &self,
        account_id: Uuid,
        email: Option<String>,
        kind: RequestKind,
    ) -> Result<DataRequest, PrivacyError> {
        self.requests
            .record(DataRequest {
                id: Uuid::new_v4(),
                account_id,
                email,
                kind,
                status: RequestStatus::Received,
                requested_at_ms: Utc::now().timestamp_millis(),
                fulfilled_at_ms: None,
                fulfilled_by: None,
            })
            .await
    }

    pub async fn open(&self, actor_id: Uuid) -> Result<Vec<DataRequest>, PrivacyError> {
        self.permitted(actor_id, Permission::DataRequestRead)
            .await?;
        self.requests.open().await
    }

    pub async fn fulfil(&self, actor_id: Uuid, request_id: Uuid) -> Result<(), PrivacyError> {
        self.permitted(actor_id, Permission::DataRequestFulfil)
            .await?;
        let held = self.requests.find(request_id).await?;
        if held.status == RequestStatus::Fulfilled {
            return Err(PrivacyError::AlreadyFulfilled);
        }

        let at = Utc::now().timestamp_millis();
        if held.kind == RequestKind::Erasure {
            self.eraser.erase(held.account_id).await?;
        }
        self.requests
            .mark_fulfilled(request_id, actor_id, at)
            .await?;

        self.audit
            .record(AuditEvent {
                id: Uuid::new_v4(),
                actor_id,
                action: AuditAction::DataRequestFulfilled,
                subject_id: None,
                detail: format!("{} {}", held.kind.as_str(), request_id),
                recorded_at_ms: at,
            })
            .await
            .map_err(rbac_to_privacy)
    }

    async fn permitted(&self, actor_id: Uuid, permission: Permission) -> Result<(), PrivacyError> {
        self.rbac
            .require(actor_id, permission)
            .await
            .map_err(rbac_to_privacy)
    }
}

fn rbac_to_privacy(error: RbacError) -> PrivacyError {
    match error {
        RbacError::Forbidden => PrivacyError::Forbidden,
        RbacError::RoleNotFound => PrivacyError::NotFound,
        RbacError::Storage(reason) => PrivacyError::Storage(reason),
    }
}
