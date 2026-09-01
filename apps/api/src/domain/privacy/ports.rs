use async_trait::async_trait;
use uuid::Uuid;

use super::error::PrivacyError;
use super::request::DataRequest;

#[async_trait]
pub trait DataRequestStore: Send + Sync {
    async fn open(&self) -> Result<Vec<DataRequest>, PrivacyError>;
    async fn record(&self, request: DataRequest) -> Result<DataRequest, PrivacyError>;
    async fn find(&self, request_id: Uuid) -> Result<DataRequest, PrivacyError>;
    async fn mark_fulfilled(
        &self,
        request_id: Uuid,
        actor_id: Uuid,
        at_ms: i64,
    ) -> Result<(), PrivacyError>;
}

/// Erasure is one transaction: the account and everything keyed to it goes, and the identity in
/// audit_subjects is nulled. No audit row is ever updated or deleted, which is what lets FR-115
/// and FR-119 hold at the same time.
#[async_trait]
pub trait Eraser: Send + Sync {
    async fn erase(&self, account_id: Uuid) -> Result<(), PrivacyError>;
}
