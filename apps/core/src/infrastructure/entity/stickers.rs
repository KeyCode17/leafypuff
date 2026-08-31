use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "stickers")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub entry_id: String,
    pub kind: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub rotation: f32,
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
