use sea_orm::ActiveValue;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::domain::crop::Framing;
use crate::domain::{CoreError, Profile};

use super::entity::profile;

const ROW_ID: i32 = 1;

pub struct ProfileStore {
    connection: DatabaseConnection,
}

impl ProfileStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn read(&self) -> Result<Profile, CoreError> {
        let row = profile::Entity::find_by_id(ROW_ID)
            .one(&self.connection)
            .await?;
        Ok(row.map(held).unwrap_or_default())
    }

    pub async fn save(&self, wanted: &Profile) -> Result<(), CoreError> {
        let row = profile::ActiveModel {
            id: ActiveValue::Set(ROW_ID),
            display_name: ActiveValue::Set(wanted.display_name.clone()),
            avatar_photo_id: ActiveValue::Set(wanted.avatar_photo_id.clone()),
            avatar_x: ActiveValue::Set(wanted.avatar_framing.map(|held| held.x)),
            avatar_y: ActiveValue::Set(wanted.avatar_framing.map(|held| held.y)),
            avatar_width: ActiveValue::Set(wanted.avatar_framing.map(|held| held.width)),
            updated_at_ms: ActiveValue::Set(wanted.updated_at_ms),
        };
        profile::Entity::insert(row)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(profile::Column::Id)
                    .update_columns([
                        profile::Column::DisplayName,
                        profile::Column::AvatarPhotoId,
                        profile::Column::AvatarX,
                        profile::Column::AvatarY,
                        profile::Column::AvatarWidth,
                        profile::Column::UpdatedAtMs,
                    ])
                    .to_owned(),
            )
            .exec(&self.connection)
            .await?;
        Ok(())
    }
}

fn held(row: profile::Model) -> Profile {
    Profile {
        display_name: row.display_name,
        avatar_photo_id: row.avatar_photo_id,
        avatar_framing: framing(row.avatar_x, row.avatar_y, row.avatar_width),
        updated_at_ms: row.updated_at_ms,
    }
}

fn framing(x: Option<f64>, y: Option<f64>, width: Option<f64>) -> Option<Framing> {
    Some(Framing {
        x: x?,
        y: y?,
        width: width?,
    })
}
