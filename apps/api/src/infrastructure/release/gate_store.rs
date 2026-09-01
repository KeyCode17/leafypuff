use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityTrait};
use uuid::Uuid;

use crate::domain::release::error::ERR_UNKNOWN_PLATFORM;
use crate::domain::release::{Platform, ReleaseError, ReleaseGate, ReleaseGateStore};

use super::entity::release_gates;

pub struct PgReleaseGateStore {
    connection: DatabaseConnection,
}

impl PgReleaseGateStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl ReleaseGateStore for PgReleaseGateStore {
    async fn read(&self, platform: Platform) -> Result<ReleaseGate, ReleaseError> {
        let row = release_gates::Entity::find_by_id(platform.as_str().to_owned())
            .one(&self.connection)
            .await
            .map_err(storage)?
            .ok_or(ReleaseError::GateNotFound)?;
        gate(row)
    }

    async fn all(&self) -> Result<Vec<ReleaseGate>, ReleaseError> {
        let rows = release_gates::Entity::find()
            .all(&self.connection)
            .await
            .map_err(storage)?;
        rows.into_iter().map(gate).collect()
    }

    async fn write(&self, held: ReleaseGate, actor_id: Uuid) -> Result<(), ReleaseError> {
        release_gates::Entity::insert(release_gates::ActiveModel {
            platform: ActiveValue::Set(held.platform.as_str().to_owned()),
            minimum_build: ActiveValue::Set(held.minimum_build),
            force_update: ActiveValue::Set(held.force_update),
            message: ActiveValue::Set(held.message),
            updated_at: ActiveValue::Set(held.updated_at_ms),
            updated_by: ActiveValue::Set(Some(actor_id)),
        })
        .on_conflict(
            OnConflict::column(release_gates::Column::Platform)
                .update_columns([
                    release_gates::Column::MinimumBuild,
                    release_gates::Column::ForceUpdate,
                    release_gates::Column::Message,
                    release_gates::Column::UpdatedAt,
                    release_gates::Column::UpdatedBy,
                ])
                .to_owned(),
        )
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(())
    }
}

fn gate(row: release_gates::Model) -> Result<ReleaseGate, ReleaseError> {
    let platform = Platform::parse(&row.platform)
        .ok_or_else(|| ReleaseError::Storage(ERR_UNKNOWN_PLATFORM.to_owned()))?;
    Ok(ReleaseGate {
        platform,
        minimum_build: row.minimum_build,
        force_update: row.force_update,
        message: row.message,
        updated_at_ms: row.updated_at,
        updated_by: row.updated_by,
    })
}

pub fn storage(error: DbErr) -> ReleaseError {
    ReleaseError::Storage(error.to_string())
}
