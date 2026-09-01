pub mod data_requests {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "data_requests")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub account_id: Uuid,
        pub email: Option<String>,
        pub kind: String,
        pub status: String,
        pub requested_at: i64,
        pub fulfilled_at: Option<i64>,
        pub fulfilled_by: Option<Uuid>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
