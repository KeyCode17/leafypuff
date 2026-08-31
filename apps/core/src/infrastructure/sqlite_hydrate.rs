use std::collections::HashMap;

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder};

use crate::domain::{CoreError, Entry};

use super::entity::{entries, photos, stickers, tags};
use super::mapper;

pub(super) async fn hydrate(
    connection: &DatabaseConnection,
    rows: Vec<entries::Model>,
) -> Result<Vec<Entry>, CoreError> {
    let ids: Vec<String> = rows.iter().map(|row| row.id.clone()).collect();

    let mut photo_rows: HashMap<String, Vec<photos::Model>> = HashMap::new();
    for row in photos::Entity::find()
        .filter(photos::Column::EntryId.is_in(ids.clone()))
        .order_by_asc(photos::Column::Ordinal)
        .all(connection)
        .await?
    {
        photo_rows
            .entry(row.entry_id.clone())
            .or_default()
            .push(row);
    }

    let mut sticker_rows: HashMap<String, Vec<stickers::Model>> = HashMap::new();
    for row in stickers::Entity::find()
        .filter(stickers::Column::EntryId.is_in(ids.clone()))
        .all(connection)
        .await?
    {
        sticker_rows
            .entry(row.entry_id.clone())
            .or_default()
            .push(row);
    }

    let mut tag_rows: HashMap<String, Vec<tags::Model>> = HashMap::new();
    for row in tags::Entity::find()
        .filter(tags::Column::EntryId.is_in(ids))
        .order_by_asc(tags::Column::Tag)
        .all(connection)
        .await?
    {
        tag_rows.entry(row.entry_id.clone()).or_default().push(row);
    }

    rows.into_iter()
        .map(|row| {
            let key = row.id.clone();
            mapper::assemble(
                row,
                photo_rows.remove(&key).unwrap_or_default(),
                sticker_rows.remove(&key).unwrap_or_default(),
                tag_rows.remove(&key).unwrap_or_default(),
            )
        })
        .collect()
}
