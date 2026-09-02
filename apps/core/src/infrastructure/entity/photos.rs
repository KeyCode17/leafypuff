use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "photos")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub entry_id: String,
    pub path: String,
    pub ordinal: i32,
    pub taken_at: Option<String>,
    pub crop_x: Option<f64>,
    pub crop_y: Option<f64>,
    pub crop_width: Option<f64>,
    pub place_x: Option<f64>,
    pub place_y: Option<f64>,
    pub place_size: Option<f64>,
    pub place_rotation: Option<f64>,
    pub place_crop_x: Option<f64>,
    pub place_crop_y: Option<f64>,
    pub place_crop_width: Option<f64>,
    pub place_ratio: Option<f64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::entries::Entity",
        from = "Column::EntryId",
        to = "super::entries::Column::Id",
        on_delete = "Cascade"
    )]
    Entries,
}

impl Related<super::entries::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entries.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
