use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use crate::domain::release::error::ERR_UNKNOWN_PLATFORM;
use crate::domain::release::{Campaign, CampaignStore, Platform, ReleaseError};

use super::entity::campaigns;
use super::gate_store::storage;

pub struct PgCampaignStore {
    connection: DatabaseConnection,
}

impl PgCampaignStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl CampaignStore for PgCampaignStore {
    async fn live(&self, platform: Platform, at_ms: i64) -> Result<Vec<Campaign>, ReleaseError> {
        let rows = campaigns::Entity::find()
            .filter(campaigns::Column::Platform.eq(platform.as_str()))
            .filter(campaigns::Column::Published.eq(true))
            .filter(campaigns::Column::StartsAt.lte(at_ms))
            .filter(campaigns::Column::EndsAt.gt(at_ms))
            .order_by_asc(campaigns::Column::StartsAt)
            .all(&self.connection)
            .await
            .map_err(storage)?;
        rows.into_iter().map(campaign).collect()
    }

    async fn all(&self) -> Result<Vec<Campaign>, ReleaseError> {
        let rows = campaigns::Entity::find()
            .order_by_desc(campaigns::Column::CreatedAt)
            .all(&self.connection)
            .await
            .map_err(storage)?;
        rows.into_iter().map(campaign).collect()
    }

    async fn upsert(&self, held: Campaign) -> Result<(), ReleaseError> {
        campaigns::Entity::insert(campaigns::ActiveModel {
            id: ActiveValue::Set(held.id),
            title: ActiveValue::Set(held.title),
            body: ActiveValue::Set(held.body),
            platform: ActiveValue::Set(held.platform.as_str().to_owned()),
            starts_at: ActiveValue::Set(held.starts_at_ms),
            ends_at: ActiveValue::Set(held.ends_at_ms),
            published: ActiveValue::Set(held.published),
            created_at: ActiveValue::Set(held.created_at_ms),
        })
        .on_conflict(
            OnConflict::column(campaigns::Column::Id)
                .update_columns([
                    campaigns::Column::Title,
                    campaigns::Column::Body,
                    campaigns::Column::Platform,
                    campaigns::Column::StartsAt,
                    campaigns::Column::EndsAt,
                    campaigns::Column::Published,
                ])
                .to_owned(),
        )
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(())
    }
}

fn campaign(row: campaigns::Model) -> Result<Campaign, ReleaseError> {
    let platform = Platform::parse(&row.platform)
        .ok_or_else(|| ReleaseError::Storage(ERR_UNKNOWN_PLATFORM.to_owned()))?;
    Ok(Campaign {
        id: row.id,
        title: row.title,
        body: row.body,
        platform,
        starts_at_ms: row.starts_at,
        ends_at_ms: row.ends_at,
        published: row.published,
        created_at_ms: row.created_at,
    })
}
