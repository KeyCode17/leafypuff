pub mod entries {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "entries")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub account_id: Uuid,
        pub date: String,
        pub mood: String,
        pub tags: Json,
        pub sticker_placements: String,
        pub photo_refs: String,
        pub weather: Option<String>,
        pub location: Option<String>,
        pub revision: i64,
        pub device_updated_at: i64,
        pub deleted_at: Option<i64>,
        pub title_ciphertext: Vec<u8>,
        pub title_nonce: Vec<u8>,
        pub title_updated_at: i64,
        pub title_device_id: Uuid,
        pub body_ciphertext: Vec<u8>,
        pub body_nonce: Vec<u8>,
        pub body_updated_at: i64,
        pub body_device_id: Uuid,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod sync_checkpoints {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "sync_checkpoints")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub account_id: Uuid,
        #[sea_orm(primary_key, auto_increment = false)]
        pub device_id: Uuid,
        pub cursor: i64,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod sync_requests {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "sync_requests")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub idempotency_key: String,
        pub account_id: Uuid,
        pub device_id: Uuid,
        pub response_hash: String,
        pub created_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod sync_field_conflicts {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "sync_field_conflicts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub account_id: Uuid,
        pub entry_id: Uuid,
        pub field: String,
        pub winner_updated_at: i64,
        pub loser_updated_at: i64,
        pub loser_device_id: Uuid,
        pub loser_ciphertext_hash: String,
        pub loser_byte_len: i64,
        pub created_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod wrapped_content_keys {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "wrapped_content_keys")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub account_id: Uuid,
        #[sea_orm(primary_key, auto_increment = false)]
        pub kind: String,
        pub blob: Vec<u8>,
        pub salt: Vec<u8>,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
