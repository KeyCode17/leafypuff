use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use leafypuff_api::domain::catalog::{CatalogBundle, CatalogError, CatalogStore};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryCatalog {
    rows: Arc<Mutex<Vec<CatalogBundle>>>,
}

impl InMemoryCatalog {
    pub fn snapshot(&self) -> Vec<CatalogBundle> {
        self.rows.lock().expect("the catalog lock holds").clone()
    }
}

#[async_trait]
impl CatalogStore for InMemoryCatalog {
    async fn all(&self) -> Result<Vec<CatalogBundle>, CatalogError> {
        Ok(self.snapshot())
    }

    async fn published(&self) -> Result<CatalogBundle, CatalogError> {
        self.snapshot()
            .into_iter()
            .find(CatalogBundle::is_published)
            .ok_or(CatalogError::NonePublished)
    }

    async fn draft(&self, bundle: CatalogBundle) -> Result<CatalogBundle, CatalogError> {
        self.rows
            .lock()
            .expect("the catalog lock holds")
            .push(bundle.clone());
        Ok(bundle)
    }

    async fn publish(
        &self,
        bundle_id: Uuid,
        actor_id: Uuid,
        at_ms: i64,
    ) -> Result<(), CatalogError> {
        let mut rows = self.rows.lock().expect("the catalog lock holds");
        if !rows.iter().any(|row| row.id == bundle_id) {
            return Err(CatalogError::NotFound);
        }
        for row in rows.iter_mut() {
            let live = row.id == bundle_id;
            row.published_at_ms = if live { Some(at_ms) } else { None };
            row.published_by = if live { Some(actor_id) } else { None };
        }
        Ok(())
    }

    async fn next_version(&self) -> Result<i32, CatalogError> {
        Ok(self
            .snapshot()
            .iter()
            .map(|row| row.version)
            .max()
            .unwrap_or(0)
            .saturating_add(1))
    }
}
