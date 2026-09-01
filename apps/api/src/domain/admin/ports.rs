use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::account_summary::AccountSummary;
use super::error::AdminError;

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
