use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use leafypuff_api::domain::release::{
    Campaign, CampaignStore, Platform, ReleaseError, ReleaseGate, ReleaseGateStore,
};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryGates {
    rows: Arc<Mutex<HashMap<&'static str, ReleaseGate>>>,
}

impl InMemoryGates {
    pub fn set(&self, gate: ReleaseGate) {
        self.rows
            .lock()
            .expect("the gate lock holds")
            .insert(gate.platform.as_str(), gate);
    }
}

#[async_trait]
impl ReleaseGateStore for InMemoryGates {
    async fn read(&self, platform: Platform) -> Result<ReleaseGate, ReleaseError> {
        self.rows
            .lock()
            .expect("the gate lock holds")
            .get(platform.as_str())
            .cloned()
            .ok_or(ReleaseError::GateNotFound)
    }

    async fn all(&self) -> Result<Vec<ReleaseGate>, ReleaseError> {
        Ok(self
            .rows
            .lock()
            .expect("the gate lock holds")
            .values()
            .cloned()
            .collect())
    }

    async fn write(&self, gate: ReleaseGate, _actor_id: Uuid) -> Result<(), ReleaseError> {
        self.set(gate);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct InMemoryCampaigns {
    rows: Arc<Mutex<Vec<Campaign>>>,
}

impl InMemoryCampaigns {
    pub fn snapshot(&self) -> Vec<Campaign> {
        self.rows.lock().expect("the campaign lock holds").clone()
    }
}

#[async_trait]
impl CampaignStore for InMemoryCampaigns {
    async fn live(&self, platform: Platform, at_ms: i64) -> Result<Vec<Campaign>, ReleaseError> {
        Ok(self
            .snapshot()
            .into_iter()
            .filter(|row| row.platform == platform && row.is_live(at_ms))
            .collect())
    }

    async fn all(&self) -> Result<Vec<Campaign>, ReleaseError> {
        Ok(self.snapshot())
    }

    async fn upsert(&self, campaign: Campaign) -> Result<(), ReleaseError> {
        let mut rows = self.rows.lock().expect("the campaign lock holds");
        rows.retain(|row| row.id != campaign.id);
        rows.push(campaign);
        Ok(())
    }
}
