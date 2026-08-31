use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
};
use uuid::Uuid;

use crate::domain::iam::{IamError, RefreshToken, RefreshTokenRepository};

use super::entity::refresh_tokens;
use super::mapper;

pub struct PgRefreshTokenRepository {
    connection: DatabaseConnection,
}

impl PgRefreshTokenRepository {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl RefreshTokenRepository for PgRefreshTokenRepository {
    async fn insert(&self, token: RefreshToken) -> Result<(), IamError> {
        let now = Utc::now().fixed_offset();
        let transaction = self.connection.begin().await.map_err(mapper::storage)?;

        refresh_tokens::Entity::update_many()
            .col_expr(refresh_tokens::Column::RevokedAt, now.into())
            .filter(refresh_tokens::Column::AccountId.eq(token.account_id))
            .filter(refresh_tokens::Column::DeviceId.eq(token.device_id.clone()))
            .filter(refresh_tokens::Column::RevokedAt.is_null())
            .exec(&transaction)
            .await
            .map_err(mapper::storage)?;

        refresh_tokens::Entity::insert(refresh_tokens::ActiveModel {
            id: ActiveValue::Set(token.id),
            account_id: ActiveValue::Set(token.account_id),
            device_id: ActiveValue::Set(token.device_id),
            token_hash: ActiveValue::Set(token.token_hash),
            issued_at: ActiveValue::Set(now),
            expires_at: ActiveValue::Set(token.expires_at.fixed_offset()),
            revoked_at: ActiveValue::Set(token.revoked_at.map(|at| at.fixed_offset())),
        })
        .exec(&transaction)
        .await
        .map_err(mapper::storage)?;

        transaction.commit().await.map_err(mapper::storage)
    }

    async fn by_hash(&self, hash: &str) -> Result<Option<RefreshToken>, IamError> {
        let row = refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::TokenHash.eq(hash))
            .one(&self.connection)
            .await
            .map_err(mapper::storage)?;
        Ok(row.map(mapper::refresh_token))
    }

    async fn revoke(&self, id: Uuid, at: DateTime<Utc>) -> Result<(), IamError> {
        refresh_tokens::Entity::update_many()
            .col_expr(refresh_tokens::Column::RevokedAt, at.fixed_offset().into())
            .filter(refresh_tokens::Column::Id.eq(id))
            .filter(refresh_tokens::Column::RevokedAt.is_null())
            .exec(&self.connection)
            .await
            .map_err(mapper::storage)?;
        Ok(())
    }
}
