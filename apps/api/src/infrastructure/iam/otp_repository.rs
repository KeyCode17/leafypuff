use chrono::{DateTime, Utc};
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, TransactionTrait,
};
use uuid::Uuid;

use crate::domain::iam::{IamError, OtpCode, OtpPurpose, OtpRepository};

use super::entity::otp_codes;
use super::mapper;

pub struct PgOtpRepository {
    connection: DatabaseConnection,
}

impl PgOtpRepository {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

impl OtpRepository for PgOtpRepository {
    async fn insert(&self, code: OtpCode) -> Result<(), IamError> {
        let now = Utc::now().fixed_offset();
        let transaction = self.connection.begin().await.map_err(mapper::storage)?;

        otp_codes::Entity::delete_many()
            .filter(otp_codes::Column::AccountId.eq(code.account_id))
            .filter(otp_codes::Column::Purpose.eq(code.purpose.as_str()))
            .filter(otp_codes::Column::ConsumedAt.is_null())
            .exec(&transaction)
            .await
            .map_err(mapper::storage)?;

        otp_codes::Entity::insert(otp_codes::ActiveModel {
            id: ActiveValue::Set(code.id),
            account_id: ActiveValue::Set(code.account_id),
            code_hash: ActiveValue::Set(code.code_hash),
            purpose: ActiveValue::Set(code.purpose.as_str().to_owned()),
            attempts: ActiveValue::Set(code.attempts),
            expires_at: ActiveValue::Set(code.expires_at.fixed_offset()),
            consumed_at: ActiveValue::Set(code.consumed_at.map(|at| at.fixed_offset())),
            created_at: ActiveValue::Set(now),
        })
        .exec(&transaction)
        .await
        .map_err(mapper::storage)?;

        transaction.commit().await.map_err(mapper::storage)
    }

    async fn open_for(
        &self,
        account_id: Uuid,
        purpose: OtpPurpose,
    ) -> Result<Option<OtpCode>, IamError> {
        let row = otp_codes::Entity::find()
            .filter(otp_codes::Column::AccountId.eq(account_id))
            .filter(otp_codes::Column::Purpose.eq(purpose.as_str()))
            .filter(otp_codes::Column::ConsumedAt.is_null())
            .one(&self.connection)
            .await
            .map_err(mapper::storage)?;
        row.map(mapper::otp_code).transpose()
    }

    async fn record_attempt(&self, id: Uuid) -> Result<(), IamError> {
        otp_codes::Entity::update_many()
            .col_expr(
                otp_codes::Column::Attempts,
                Expr::col(otp_codes::Column::Attempts).add(1),
            )
            .filter(otp_codes::Column::Id.eq(id))
            .exec(&self.connection)
            .await
            .map_err(mapper::storage)?;
        Ok(())
    }

    async fn consume(&self, id: Uuid, at: DateTime<Utc>) -> Result<(), IamError> {
        otp_codes::Entity::update_many()
            .col_expr(otp_codes::Column::ConsumedAt, at.fixed_offset().into())
            .filter(otp_codes::Column::Id.eq(id))
            .filter(otp_codes::Column::ConsumedAt.is_null())
            .exec(&self.connection)
            .await
            .map_err(mapper::storage)?;
        Ok(())
    }
}
