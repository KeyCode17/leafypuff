use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::account_summary::AccountSummary;
use super::error::AdminError;
use super::overview::ServiceOverview;

#[async_trait]
pub trait AccountDirectory: Send + Sync {
    async fn summaries(&self, limit: u64) -> Result<Vec<AccountSummary>, AdminError>;
    async fn summary(&self, account_id: Uuid) -> Result<AccountSummary, AdminError>;
    async fn set_suspended(
        &self,
        account_id: Uuid,
        at: Option<DateTime<Utc>>,
    ) -> Result<(), AdminError>;
}

#[async_trait]
pub trait ServiceMetrics: Send + Sync {
    async fn overview(&self, since_ms: i64) -> Result<ServiceOverview, AdminError>;
}
