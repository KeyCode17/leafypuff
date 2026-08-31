use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "entries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub date: String,
    pub mood: String,
    pub title: Vec<u8>,
    pub title_nonce: Option<Vec<u8>>,
    pub body: Vec<u8>,
    pub body_nonce: Option<Vec<u8>>,
    pub revision: i32,
    pub weather: Option<String>,
    pub location: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::photos::Entity")]
    Photos,
    #[sea_orm(has_many = "super::stickers::Entity")]
    Stickers,
    #[sea_orm(has_many = "super::tags::Entity")]
    Tags,
}

impl Related<super::photos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Photos.def()
    }
}

impl Related<super::stickers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Stickers.def()
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tags.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
