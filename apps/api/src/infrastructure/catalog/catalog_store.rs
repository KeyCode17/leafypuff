use async_trait::async_trait;
use sea_orm::{
    ActiveValue, ColumnTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, QueryOrder, Statement, TransactionTrait,
};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::catalog::{CatalogBundle, CatalogError, CatalogStore};

use super::entity::catalog_bundles;

pub struct PgCatalogStore {
    connection: DatabaseConnection,
}

impl PgCatalogStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl CatalogStore for PgCatalogStore {
    async fn all(&self) -> Result<Vec<CatalogBundle>, CatalogError> {
        let rows = catalog_bundles::Entity::find()
            .order_by_desc(catalog_bundles::Column::Version)
            .all(&self.connection)
            .await
            .map_err(storage)?;
        Ok(rows.into_iter().map(bundle).collect())
    }

    async fn published(&self) -> Result<CatalogBundle, CatalogError> {
        let row = catalog_bundles::Entity::find()
            .filter(catalog_bundles::Column::PublishedAt.is_not_null())
            .one(&self.connection)
            .await
            .map_err(storage)?
            .ok_or(CatalogError::NonePublished)?;
        Ok(bundle(row))
    }

    async fn draft(&self, held: CatalogBundle) -> Result<CatalogBundle, CatalogError> {
        let payload: Value = serde_json::from_str(&held.payload)
            .map_err(|error| CatalogError::Malformed(error.to_string()))?;
        catalog_bundles::Entity::insert(catalog_bundles::ActiveModel {
            id: ActiveValue::Set(held.id),
            version: ActiveValue::Set(held.version),
            payload: ActiveValue::Set(payload),
            published_at: ActiveValue::Set(None),
            published_by: ActiveValue::Set(None),
            created_at: ActiveValue::Set(held.created_at_ms),
        })
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(held)
    }

    /// Unpublish then publish, in one transaction. The partial unique index makes two published
    /// rows impossible, so doing this in two statements without a transaction would fail rather
    /// than leave the catalog with none.
    async fn publish(
        &self,
        bundle_id: Uuid,
        actor_id: Uuid,
        at_ms: i64,
    ) -> Result<(), CatalogError> {
        let transaction = self.connection.begin().await.map_err(storage)?;
        transaction
            .execute(Statement::from_string(
                DatabaseBackend::Postgres,
                "UPDATE catalog_bundles SET published_at = NULL, published_by = NULL \
                 WHERE published_at IS NOT NULL",
            ))
            .await
            .map_err(storage)?;
        let affected = transaction
            .execute(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                "UPDATE catalog_bundles SET published_at = $1, published_by = $2 WHERE id = $3",
                [at_ms.into(), actor_id.into(), bundle_id.into()],
            ))
            .await
            .map_err(storage)?;
        if affected.rows_affected() == 0 {
            return Err(CatalogError::NotFound);
        }
        transaction.commit().await.map_err(storage)
    }

    async fn next_version(&self) -> Result<i32, CatalogError> {
        let highest = catalog_bundles::Entity::find()
            .order_by_desc(catalog_bundles::Column::Version)
            .one(&self.connection)
            .await
            .map_err(storage)?;
        Ok(highest.map_or(1, |row| row.version.saturating_add(1)))
    }
}

fn bundle(row: catalog_bundles::Model) -> CatalogBundle {
    CatalogBundle {
        id: row.id,
        version: row.version,
        payload: row.payload.to_string(),
        published_at_ms: row.published_at,
        published_by: row.published_by,
        created_at_ms: row.created_at,
    }
}

fn storage(error: DbErr) -> CatalogError {
    CatalogError::Storage(error.to_string())
}
