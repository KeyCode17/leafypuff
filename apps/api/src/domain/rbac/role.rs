use uuid::Uuid;

use super::permission::Permission;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<Permission>,
}

impl Role {
    pub fn grants(&self, permission: Permission) -> bool {
        self.permissions.contains(&permission)
    }
}
