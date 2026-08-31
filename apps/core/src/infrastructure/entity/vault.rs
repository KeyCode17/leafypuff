use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "vault")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub passphrase_salt: Vec<u8>,
    pub passphrase_nonce: Vec<u8>,
    pub passphrase_ciphertext: Vec<u8>,
    pub recovery_nonce: Vec<u8>,
    pub recovery_ciphertext: Vec<u8>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
