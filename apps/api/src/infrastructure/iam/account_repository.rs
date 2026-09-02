use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::sea_query::{Alias, Expr, Value};
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::domain::iam::{Account, AccountRepository, IamError};

use super::entity::accounts;
use super::mapper;

const CITEXT: &str = "citext";

pub struct PgAccountRepository {
    connection: DatabaseConnection,
}

impl PgAccountRepository {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl AccountRepository for PgAccountRepository {
    async fn by_email(&self, email: &str) -> Result<Option<Account>, IamError> {
        let row = accounts::Entity::find()
            .filter(
                Expr::col(accounts::Column::Email).eq(Expr::val(email).cast_as(Alias::new(CITEXT))),
            )
            .one(&self.connection)
            .await
            .map_err(mapper::storage)?;
        Ok(row.map(mapper::account))
    }

    async fn by_id(&self, id: Uuid) -> Result<Option<Account>, IamError> {
        let row = accounts::Entity::find_by_id(id)
            .one(&self.connection)
            .await
            .map_err(mapper::storage)?;
        Ok(row.map(mapper::account))
    }

    async fn insert(&self, account: Account) -> Result<Account, IamError> {
        let now = Utc::now().fixed_offset();
        let row = accounts::ActiveModel {
            id: ActiveValue::Set(account.id),
            email: ActiveValue::Set(account.email),
            password_hash: ActiveValue::Set(account.password_hash),
            display_name: ActiveValue::Set(account.display_name),
            email_verified_at: ActiveValue::Set(
                account.email_verified_at.map(|at| at.fixed_offset()),
            ),
            pending_email: ActiveValue::Set(None),
            suspended_at: ActiveValue::Set(None),
            created_at: ActiveValue::Set(now),
            updated_at: ActiveValue::Set(now),
        };
        let inserted = accounts::Entity::insert(row)
            .exec_with_returning(&self.connection)
            .await
            .map_err(|error| mapper::insert_conflict(error, IamError::EmailAlreadyRegistered))?;
        Ok(mapper::account(inserted))
    }

    async fn mark_verified(&self, id: Uuid, at: DateTime<Utc>) -> Result<(), IamError> {
        let stamp = at.fixed_offset();
        accounts::Entity::update_many()
            .col_expr(accounts::Column::EmailVerifiedAt, stamp.into())
            .col_expr(accounts::Column::UpdatedAt, stamp.into())
            .filter(accounts::Column::Id.eq(id))
            .exec(&self.connection)
            .await
            .map_err(mapper::storage)?;
        Ok(())
    }

    async fn hold_pending_email(
        &self,
        id: Uuid,
        email: Option<String>,
        at: DateTime<Utc>,
    ) -> Result<(), IamError> {
        let stamp = at.fixed_offset();
        accounts::Entity::update_many()
            .col_expr(accounts::Column::PendingEmail, email.into())
            .col_expr(accounts::Column::UpdatedAt, stamp.into())
            .filter(accounts::Column::Id.eq(id))
            .exec(&self.connection)
            .await
            .map_err(mapper::storage)?;
        Ok(())
    }

    async fn adopt_pending_email(
        &self,
        id: Uuid,
        email: String,
        at: DateTime<Utc>,
    ) -> Result<(), IamError> {
        let stamp = at.fixed_offset();
        accounts::Entity::update_many()
            .col_expr(accounts::Column::Email, email.into())
            .col_expr(
                accounts::Column::PendingEmail,
                Expr::value(Value::String(None)),
            )
            .col_expr(accounts::Column::UpdatedAt, stamp.into())
            .filter(accounts::Column::Id.eq(id))
            .exec(&self.connection)
            .await
            .map_err(|error| mapper::insert_conflict(error, IamError::EmailAlreadyRegistered))?;
        Ok(())
    }

    async fn update_password(
        &self,
        id: Uuid,
        password_hash: String,
        at: DateTime<Utc>,
    ) -> Result<(), IamError> {
        let stamp = at.fixed_offset();
        accounts::Entity::update_many()
            .col_expr(accounts::Column::PasswordHash, password_hash.into())
            .col_expr(accounts::Column::UpdatedAt, stamp.into())
            .filter(accounts::Column::Id.eq(id))
            .exec(&self.connection)
            .await
            .map_err(mapper::storage)?;
        Ok(())
    }
}
