use chrono::NaiveDate;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
    TransactionTrait,
};

use crate::domain::error::ERR_TOO_MANY_PHOTOS;
use crate::domain::{CoreError, Entry, EntryId, EntryRepository, FieldSealer, PhotoRef};

use super::entity::{entries, photos, stickers, tags};
use super::mapper;

const ISO_DATE: &str = "%Y-%m-%d";

pub struct SqliteEntryRepository<S: FieldSealer> {
    connection: DatabaseConnection,
    sealer: S,
}

impl<S: FieldSealer> SqliteEntryRepository<S> {
    pub const fn new(connection: DatabaseConnection, sealer: S) -> Self {
        Self { connection, sealer }
    }
}

impl<S: FieldSealer + Send + Sync> EntryRepository for SqliteEntryRepository<S> {
    async fn save(&self, entry: Entry) -> Result<Entry, CoreError> {
        let id = entry.id.to_text();
        let mut numbered = Vec::with_capacity(entry.photos.len());
        for (index, photo) in entry.photos.iter().enumerate() {
            let ordinal = i32::try_from(index)
                .map_err(|_| CoreError::Invalid(ERR_TOO_MANY_PHOTOS.to_owned()))?;
            numbered.push(PhotoRef {
                ordinal,
                ..photo.clone()
            });
        }
        let saved = Entry {
            photos: numbered,
            ..entry
        };

        let transaction = self.connection.begin().await?;

        entries::Entity::insert(mapper::entry_row(&saved, &self.sealer)?)
            .on_conflict(
                OnConflict::column(entries::Column::Id)
                    .update_columns([
                        entries::Column::Date,
                        entries::Column::Mood,
                        entries::Column::Title,
                        entries::Column::TitleNonce,
                        entries::Column::Body,
                        entries::Column::BodyNonce,
                        entries::Column::Revision,
                        entries::Column::Weather,
                        entries::Column::Location,
                        entries::Column::UpdatedAt,
                    ])
                    .to_owned(),
            )
            .exec(&transaction)
            .await?;

        photos::Entity::delete_many()
            .filter(photos::Column::EntryId.eq(id.clone()))
            .exec(&transaction)
            .await?;
        stickers::Entity::delete_many()
            .filter(stickers::Column::EntryId.eq(id.clone()))
            .exec(&transaction)
            .await?;
        tags::Entity::delete_many()
            .filter(tags::Column::EntryId.eq(id.clone()))
            .exec(&transaction)
            .await?;

        for photo in &saved.photos {
            photos::Entity::insert(mapper::photo_row(&id, photo))
                .exec(&transaction)
                .await?;
        }
        for sticker in &saved.stickers {
            stickers::Entity::insert(mapper::sticker_row(&id, sticker))
                .exec(&transaction)
                .await?;
        }
        for tag in &saved.tags {
            tags::Entity::insert(mapper::tag_row(&id, tag))
                .exec(&transaction)
                .await?;
        }

        transaction.commit().await?;
        Ok(saved)
    }

    async fn by_id(&self, id: EntryId) -> Result<Option<Entry>, CoreError> {
        let row = entries::Entity::find_by_id(id.to_text())
            .one(&self.connection)
            .await?;
        match row {
            None => Ok(None),
            Some(found) => {
                Ok(
                    super::sqlite_hydrate::hydrate(&self.connection, vec![found], &self.sealer)
                        .await?
                        .into_iter()
                        .next(),
                )
            }
        }
    }

    async fn list_desc(&self, limit: u32) -> Result<Vec<Entry>, CoreError> {
        let rows = entries::Entity::find()
            .order_by_desc(entries::Column::Date)
            .order_by_desc(entries::Column::CreatedAt)
            .limit(u64::from(limit))
            .all(&self.connection)
            .await?;
        super::sqlite_hydrate::hydrate(&self.connection, rows, &self.sealer).await
    }

    async fn in_range(&self, from: NaiveDate, to: NaiveDate) -> Result<Vec<Entry>, CoreError> {
        let rows = entries::Entity::find()
            .filter(entries::Column::Date.gte(from.format(ISO_DATE).to_string()))
            .filter(entries::Column::Date.lte(to.format(ISO_DATE).to_string()))
            .order_by_desc(entries::Column::Date)
            .order_by_desc(entries::Column::CreatedAt)
            .all(&self.connection)
            .await?;
        super::sqlite_hydrate::hydrate(&self.connection, rows, &self.sealer).await
    }

    async fn on_date(&self, date: NaiveDate) -> Result<Vec<Entry>, CoreError> {
        let rows = entries::Entity::find()
            .filter(entries::Column::Date.eq(date.format(ISO_DATE).to_string()))
            .order_by_desc(entries::Column::CreatedAt)
            .all(&self.connection)
            .await?;
        super::sqlite_hydrate::hydrate(&self.connection, rows, &self.sealer).await
    }

    async fn delete_all(&self) -> Result<(), CoreError> {
        let transaction = self.connection.begin().await?;
        tags::Entity::delete_many().exec(&transaction).await?;
        stickers::Entity::delete_many().exec(&transaction).await?;
        photos::Entity::delete_many().exec(&transaction).await?;
        entries::Entity::delete_many().exec(&transaction).await?;
        transaction.commit().await?;
        Ok(())
    }
}
