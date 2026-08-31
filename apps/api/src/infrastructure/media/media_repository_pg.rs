use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::domain::media::error::ERR_UNKNOWN_VARIANT;
use crate::domain::media::{MediaError, MediaObject, MediaRepository, Variant};

use super::entity::media_objects;

pub struct PgMediaRepository {
    connection: DatabaseConnection,
}

impl PgMediaRepository {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl MediaRepository for PgMediaRepository {
    async fn record(&self, object: MediaObject) -> Result<(), MediaError> {
        media_objects::Entity::insert(media_objects::ActiveModel {
            photo_id: ActiveValue::Set(object.photo_id),
            variant: ActiveValue::Set(object.variant.as_str().to_owned()),
            account_id: ActiveValue::Set(object.account_id),
            entry_id: ActiveValue::Set(object.entry_id),
            byte_len: ActiveValue::Set(object.byte_len),
            ciphertext_hash: ActiveValue::Set(object.ciphertext_hash),
            created_at: ActiveValue::Set(object.created_at_ms),
        })
        .on_conflict(
            OnConflict::columns([
                media_objects::Column::PhotoId,
                media_objects::Column::Variant,
            ])
            .update_columns([
                media_objects::Column::EntryId,
                media_objects::Column::ByteLen,
                media_objects::Column::CiphertextHash,
                media_objects::Column::CreatedAt,
            ])
            .to_owned(),
        )
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(())
    }

    async fn find(&self, account_id: Uuid, photo_id: Uuid) -> Result<Vec<MediaObject>, MediaError> {
        let rows = media_objects::Entity::find()
            .filter(media_objects::Column::AccountId.eq(account_id))
            .filter(media_objects::Column::PhotoId.eq(photo_id))
            .all(&self.connection)
            .await
            .map_err(storage)?;
        rows.into_iter().map(object).collect()
    }

    async fn forget(&self, account_id: Uuid, photo_id: Uuid) -> Result<(), MediaError> {
        media_objects::Entity::delete_many()
            .filter(media_objects::Column::AccountId.eq(account_id))
            .filter(media_objects::Column::PhotoId.eq(photo_id))
            .exec(&self.connection)
            .await
            .map_err(storage)?;
        Ok(())
    }
}

fn storage(error: sea_orm::DbErr) -> MediaError {
    MediaError::Storage(error.to_string())
}

fn object(row: media_objects::Model) -> Result<MediaObject, MediaError> {
    let variant = Variant::parse(&row.variant)
        .ok_or_else(|| MediaError::Storage(ERR_UNKNOWN_VARIANT.to_owned()))?;
    Ok(MediaObject {
        photo_id: row.photo_id,
        account_id: row.account_id,
        entry_id: row.entry_id,
        variant,
        byte_len: row.byte_len,
        ciphertext_hash: row.ciphertext_hash,
        created_at_ms: row.created_at,
    })
}
