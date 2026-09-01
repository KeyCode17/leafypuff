use async_trait::async_trait;
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

use crate::domain::privacy::error::{ERR_UNKNOWN_KIND, ERR_UNKNOWN_STATUS};
use crate::domain::privacy::{
    DataRequest, DataRequestStore, PrivacyError, RequestKind, RequestStatus,
};

use super::entity::data_requests;

pub struct PgDataRequestStore {
    connection: DatabaseConnection,
}

impl PgDataRequestStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl DataRequestStore for PgDataRequestStore {
    async fn open(&self) -> Result<Vec<DataRequest>, PrivacyError> {
        let rows = data_requests::Entity::find()
            .filter(data_requests::Column::Status.eq(RequestStatus::Received.as_str()))
            .order_by_asc(data_requests::Column::RequestedAt)
            .all(&self.connection)
            .await
            .map_err(storage)?;
        rows.into_iter().map(request).collect()
    }

    async fn record(&self, held: DataRequest) -> Result<DataRequest, PrivacyError> {
        data_requests::Entity::insert(data_requests::ActiveModel {
            id: ActiveValue::Set(held.id),
            account_id: ActiveValue::Set(held.account_id),
            email: ActiveValue::Set(held.email.clone()),
            kind: ActiveValue::Set(held.kind.as_str().to_owned()),
            status: ActiveValue::Set(held.status.as_str().to_owned()),
            requested_at: ActiveValue::Set(held.requested_at_ms),
            fulfilled_at: ActiveValue::Set(held.fulfilled_at_ms),
            fulfilled_by: ActiveValue::Set(held.fulfilled_by),
        })
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(held)
    }

    async fn find(&self, request_id: Uuid) -> Result<DataRequest, PrivacyError> {
        let row = data_requests::Entity::find_by_id(request_id)
            .one(&self.connection)
            .await
            .map_err(storage)?
            .ok_or(PrivacyError::NotFound)?;
        request(row)
    }

    async fn mark_fulfilled(
        &self,
        request_id: Uuid,
        actor_id: Uuid,
        at_ms: i64,
    ) -> Result<(), PrivacyError> {
        let outcome = data_requests::Entity::update_many()
            .col_expr(
                data_requests::Column::Status,
                RequestStatus::Fulfilled.as_str().into(),
            )
            .col_expr(data_requests::Column::FulfilledAt, at_ms.into())
            .col_expr(data_requests::Column::FulfilledBy, actor_id.into())
            .filter(data_requests::Column::Id.eq(request_id))
            .filter(data_requests::Column::Status.eq(RequestStatus::Received.as_str()))
            .exec(&self.connection)
            .await
            .map_err(storage)?;
        if outcome.rows_affected == 0 {
            return Err(PrivacyError::AlreadyFulfilled);
        }
        Ok(())
    }
}

fn request(row: data_requests::Model) -> Result<DataRequest, PrivacyError> {
    let kind = RequestKind::parse(&row.kind)
        .ok_or_else(|| PrivacyError::Storage(ERR_UNKNOWN_KIND.to_owned()))?;
    let status = RequestStatus::parse(&row.status)
        .ok_or_else(|| PrivacyError::Storage(ERR_UNKNOWN_STATUS.to_owned()))?;
    Ok(DataRequest {
        id: row.id,
        account_id: row.account_id,
        email: row.email,
        kind,
        status,
        requested_at_ms: row.requested_at,
        fulfilled_at_ms: row.fulfilled_at,
        fulfilled_by: row.fulfilled_by,
    })
}

fn storage(error: DbErr) -> PrivacyError {
    PrivacyError::Storage(error.to_string())
}
