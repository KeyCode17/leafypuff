use async_trait::async_trait;
use uuid::Uuid;

use super::bundle::CatalogBundle;
use super::error::CatalogError;

#[async_trait]
pub trait CatalogStore: Send + Sync {
    async fn all(&self) -> Result<Vec<CatalogBundle>, CatalogError>;
    async fn published(&self) -> Result<CatalogBundle, CatalogError>;
    async fn draft(&self, bundle: CatalogBundle) -> Result<CatalogBundle, CatalogError>;
    async fn publish(
        &self,
        bundle_id: Uuid,
        actor_id: Uuid,
        at_ms: i64,
    ) -> Result<(), CatalogError>;
    async fn next_version(&self) -> Result<i32, CatalogError>;
}
