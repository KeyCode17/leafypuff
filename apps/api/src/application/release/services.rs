use std::sync::Arc;

use chrono::Utc;
use uuid::Uuid;

use crate::application::rbac::RbacServices;
use crate::domain::rbac::{AuditAction, AuditEvent, AuditLog, Permission, RbacError};
use crate::domain::release::{
    Campaign, CampaignStore, Platform, ReleaseError, ReleaseGate, ReleaseGateStore,
};

#[derive(Clone)]
pub struct ReleaseServices {
    pub gates: Arc<dyn ReleaseGateStore>,
    pub campaigns: Arc<dyn CampaignStore>,
    pub audit: Arc<dyn AuditLog>,
    pub rbac: RbacServices,
}

impl ReleaseServices {
    pub async fn gate(&self, platform: Platform) -> Result<ReleaseGate, ReleaseError> {
        self.gates.read(platform).await
    }

    pub async fn live_campaigns(&self, platform: Platform) -> Result<Vec<Campaign>, ReleaseError> {
        self.campaigns
            .live(platform, Utc::now().timestamp_millis())
            .await
    }

    pub async fn all_gates(&self, actor_id: Uuid) -> Result<Vec<ReleaseGate>, ReleaseError> {
        self.permitted(actor_id, Permission::ReleaseRead).await?;
        self.gates.all().await
    }

    pub async fn all_campaigns(&self, actor_id: Uuid) -> Result<Vec<Campaign>, ReleaseError> {
        self.permitted(actor_id, Permission::ReleaseRead).await?;
        self.campaigns.all().await
    }

    pub async fn set_gate(
        &self,
        actor_id: Uuid,
        platform: Platform,
        minimum_build: i32,
        force_update: bool,
        message: Option<String>,
    ) -> Result<(), ReleaseError> {
        self.permitted(actor_id, Permission::ReleaseWrite).await?;
        let at = Utc::now().timestamp_millis();
        self.gates
            .write(
                ReleaseGate {
                    platform,
                    minimum_build,
                    force_update,
                    message,
                    updated_at_ms: at,
                    updated_by: Some(actor_id),
                },
                actor_id,
            )
            .await?;
        self.audit
            .record(AuditEvent {
                id: Uuid::new_v4(),
                actor_id,
                action: AuditAction::ReleaseGateChanged,
                subject_id: None,
                detail: format!(
                    "{} build {minimum_build} force {force_update}",
                    platform.as_str()
                ),
                recorded_at_ms: at,
            })
            .await
            .map_err(rbac_to_release)
    }

    pub async fn save_campaign(
        &self,
        actor_id: Uuid,
        campaign: Campaign,
    ) -> Result<(), ReleaseError> {
        self.permitted(actor_id, Permission::ReleaseWrite).await?;
        self.campaigns.upsert(campaign).await
    }

    async fn permitted(&self, actor_id: Uuid, permission: Permission) -> Result<(), ReleaseError> {
        self.rbac
            .require(actor_id, permission)
            .await
            .map_err(rbac_to_release)
    }
}

fn rbac_to_release(error: RbacError) -> ReleaseError {
    match error {
        RbacError::Forbidden => ReleaseError::Forbidden,
        RbacError::RoleNotFound => ReleaseError::GateNotFound,
        RbacError::Storage(reason) => ReleaseError::Storage(reason),
    }
}
