pub mod media_objects {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "media_objects")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub photo_id: Uuid,
        #[sea_orm(primary_key, auto_increment = false)]
        pub variant: String,
        pub account_id: Uuid,
        pub entry_id: Uuid,
        pub byte_len: i64,
        pub ciphertext_hash: String,
        pub created_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
