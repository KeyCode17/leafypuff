use std::collections::HashMap;

use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::domain::rbac::error::ERR_UNKNOWN_PERMISSION;
use crate::domain::rbac::{Permission, RbacError, Role, RoleRepository};

use super::entity::{account_roles, role_permissions, roles};

pub struct PgRoleRepository {
    connection: DatabaseConnection,
}

impl PgRoleRepository {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    async fn assemble(&self, rows: Vec<roles::Model>) -> Result<Vec<Role>, RbacError> {
        let ids: Vec<Uuid> = rows.iter().map(|row| row.id).collect();
        let grants = role_permissions::Entity::find()
            .filter(role_permissions::Column::RoleId.is_in(ids))
            .all(&self.connection)
            .await
            .map_err(storage)?;

        let mut by_role: HashMap<Uuid, Vec<Permission>> = HashMap::new();
        for grant in grants {
            let permission = Permission::parse(&grant.permission)
                .ok_or_else(|| RbacError::Storage(ERR_UNKNOWN_PERMISSION.to_owned()))?;
            by_role.entry(grant.role_id).or_default().push(permission);
        }

        Ok(rows
            .into_iter()
            .map(|row| Role {
                permissions: by_role.remove(&row.id).unwrap_or_default(),
                id: row.id,
                name: row.name,
                description: row.description,
            })
            .collect())
    }
}

#[async_trait]
impl RoleRepository for PgRoleRepository {
    async fn all(&self) -> Result<Vec<Role>, RbacError> {
        let rows = roles::Entity::find()
            .all(&self.connection)
            .await
            .map_err(storage)?;
        self.assemble(rows).await
    }

    async fn for_account(&self, account_id: Uuid) -> Result<Vec<Role>, RbacError> {
        let assigned = account_roles::Entity::find()
            .filter(account_roles::Column::AccountId.eq(account_id))
            .all(&self.connection)
            .await
            .map_err(storage)?;
        let ids: Vec<Uuid> = assigned.into_iter().map(|row| row.role_id).collect();
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let rows = roles::Entity::find()
            .filter(roles::Column::Id.is_in(ids))
            .all(&self.connection)
            .await
            .map_err(storage)?;
        self.assemble(rows).await
    }

    async fn assign(&self, account_id: Uuid, role_id: Uuid) -> Result<(), RbacError> {
        account_roles::Entity::insert(account_roles::ActiveModel {
            account_id: ActiveValue::Set(account_id),
            role_id: ActiveValue::Set(role_id),
        })
        .on_conflict(
            OnConflict::columns([
                account_roles::Column::AccountId,
                account_roles::Column::RoleId,
            ])
            .do_nothing()
            .to_owned(),
        )
        .do_nothing()
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(())
    }

    async fn revoke(&self, account_id: Uuid, role_id: Uuid) -> Result<(), RbacError> {
        account_roles::Entity::delete_by_id((account_id, role_id))
            .exec(&self.connection)
            .await
            .map_err(storage)?;
        Ok(())
    }
}

pub fn storage(error: sea_orm::DbErr) -> RbacError {
    RbacError::Storage(error.to_string())
}
