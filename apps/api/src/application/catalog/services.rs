use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::application::rbac::RbacServices;
use crate::domain::catalog::{CatalogBundle, CatalogError, CatalogStore, MAX_PAYLOAD_BYTES};
use crate::domain::rbac::{AuditAction, AuditEvent, AuditLog, Permission, RbacError};

#[derive(Clone)]
pub struct CatalogServices {
    pub store: Arc<dyn CatalogStore>,
    pub audit: Arc<dyn AuditLog>,
    pub rbac: RbacServices,
}

impl CatalogServices {
    /// The device path. It carries no permission check on purpose: a signed-in app asking which
    /// catalog to render is not an operator action, and gating it would make the moods depend on
    /// a role nobody would think to grant.
    pub async fn published(&self) -> Result<CatalogBundle, CatalogError> {
        self.store.published().await
    }

    pub async fn list(&self, actor_id: Uuid) -> Result<Vec<CatalogBundle>, CatalogError> {
        self.permitted(actor_id, Permission::CatalogRead).await?;
        self.store.all().await
    }

    pub async fn draft(
        &self,
        actor_id: Uuid,
        payload: String,
    ) -> Result<CatalogBundle, CatalogError> {
        self.permitted(actor_id, Permission::CatalogWrite).await?;
        if payload.len() > MAX_PAYLOAD_BYTES {
            return Err(CatalogError::Malformed(
                "payload is over the ceiling".to_owned(),
            ));
        }
        let version = self.store.next_version().await?;
        self.store
            .draft(CatalogBundle {
                id: Uuid::new_v4(),
                version,
                payload,
                published_at_ms: None,
                published_by: None,
                created_at_ms: Utc::now().timestamp_millis(),
            })
            .await
    }

    pub async fn publish(&self, actor_id: Uuid, bundle_id: Uuid) -> Result<(), CatalogError> {
        self.permitted(actor_id, Permission::CatalogPublish).await?;
        let at = Utc::now().timestamp_millis();
        self.store.publish(bundle_id, actor_id, at).await?;
        self.audit
            .record(AuditEvent {
                id: Uuid::new_v4(),
                actor_id,
                action: AuditAction::CatalogPublished,
                subject_id: None,
                detail: bundle_id.to_string(),
                recorded_at_ms: at,
            })
            .await
            .map_err(rbac_to_catalog)
    }

    async fn permitted(&self, actor_id: Uuid, permission: Permission) -> Result<(), CatalogError> {
        self.rbac
            .require(actor_id, permission)
            .await
            .map_err(rbac_to_catalog)
    }
}

fn rbac_to_catalog(error: RbacError) -> CatalogError {
    match error {
        RbacError::Forbidden => CatalogError::Forbidden,
        RbacError::RoleNotFound => CatalogError::NotFound,
        RbacError::Storage(reason) => CatalogError::Storage(reason),
    }
}
