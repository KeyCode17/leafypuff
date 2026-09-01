use async_trait::async_trait;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::domain::rbac::error::ERR_UNKNOWN_PERMISSION;
use crate::domain::rbac::{Permission, PermissionReader, RbacError};

use super::entity::{account_roles, role_permissions};
use super::role_repository::storage;

pub struct PgPermissionReader {
    connection: DatabaseConnection,
}

impl PgPermissionReader {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl PermissionReader for PgPermissionReader {
    async fn granted(&self, account_id: Uuid) -> Result<Vec<Permission>, RbacError> {
        let assigned = account_roles::Entity::find()
            .filter(account_roles::Column::AccountId.eq(account_id))
            .all(&self.connection)
            .await
            .map_err(storage)?;
        let ids: Vec<Uuid> = assigned.into_iter().map(|row| row.role_id).collect();
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let grants = role_permissions::Entity::find()
            .filter(role_permissions::Column::RoleId.is_in(ids))
            .all(&self.connection)
            .await
            .map_err(storage)?;

        let mut held = Vec::with_capacity(grants.len());
        for grant in grants {
            let permission = Permission::parse(&grant.permission)
                .ok_or_else(|| RbacError::Storage(ERR_UNKNOWN_PERMISSION.to_owned()))?;
            if !held.contains(&permission) {
                held.push(permission);
            }
        }
        Ok(held)
    }
}
