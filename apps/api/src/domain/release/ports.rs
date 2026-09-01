use async_trait::async_trait;
use uuid::Uuid;

use super::error::ReleaseError;
use super::gate::{Campaign, Platform, ReleaseGate};

#[async_trait]
pub trait ReleaseGateStore: Send + Sync {
    async fn read(&self, platform: Platform) -> Result<ReleaseGate, ReleaseError>;
    async fn all(&self) -> Result<Vec<ReleaseGate>, ReleaseError>;
    async fn write(&self, gate: ReleaseGate, actor_id: Uuid) -> Result<(), ReleaseError>;
}

#[async_trait]
pub trait CampaignStore: Send + Sync {
    async fn live(&self, platform: Platform, at_ms: i64) -> Result<Vec<Campaign>, ReleaseError>;
    async fn all(&self) -> Result<Vec<Campaign>, ReleaseError>;
    async fn upsert(&self, campaign: Campaign) -> Result<(), ReleaseError>;
}
