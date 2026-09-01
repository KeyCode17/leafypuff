use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use leafypuff_api::domain::rbac::{
    AuditEvent, AuditLog, Permission, PermissionReader, RbacError, Role, RoleRepository,
};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryRoles {
    catalog: Arc<Mutex<Vec<Role>>>,
    assigned: Arc<Mutex<HashMap<Uuid, Vec<Uuid>>>>,
}

impl InMemoryRoles {
    pub fn define(&self, role: Role) {
        self.catalog.lock().expect("the role lock holds").push(role);
    }

    pub fn hold(&self, account_id: Uuid, role_id: Uuid) {
        self.assigned
            .lock()
            .expect("the assignment lock holds")
            .entry(account_id)
            .or_default()
            .push(role_id);
    }

    fn held(&self, account_id: Uuid) -> Vec<Role> {
        let assigned = self.assigned.lock().expect("the assignment lock holds");
        let ids = assigned.get(&account_id).cloned().unwrap_or_default();
        self.catalog
            .lock()
            .expect("the role lock holds")
            .iter()
            .filter(|role| ids.contains(&role.id))
            .cloned()
            .collect()
    }
}

#[async_trait]
impl RoleRepository for InMemoryRoles {
    async fn all(&self) -> Result<Vec<Role>, RbacError> {
        Ok(self.catalog.lock().expect("the role lock holds").clone())
    }

    async fn for_account(&self, account_id: Uuid) -> Result<Vec<Role>, RbacError> {
        Ok(self.held(account_id))
    }

    async fn assign(&self, account_id: Uuid, role_id: Uuid) -> Result<(), RbacError> {
        self.hold(account_id, role_id);
        Ok(())
    }

    async fn revoke(&self, account_id: Uuid, role_id: Uuid) -> Result<(), RbacError> {
        let mut assigned = self.assigned.lock().expect("the assignment lock holds");
        if let Some(held) = assigned.get_mut(&account_id) {
            held.retain(|id| *id != role_id);
        }
        Ok(())
    }
}

#[async_trait]
impl PermissionReader for InMemoryRoles {
    async fn granted(&self, account_id: Uuid) -> Result<Vec<Permission>, RbacError> {
        let mut held: Vec<Permission> = Vec::new();
        for role in self.held(account_id) {
            for permission in role.permissions {
                if !held.contains(&permission) {
                    held.push(permission);
                }
            }
        }
        Ok(held)
    }
}

#[derive(Clone, Default)]
pub struct InMemoryAudit {
    rows: Arc<Mutex<Vec<AuditEvent>>>,
}

impl InMemoryAudit {
    pub fn snapshot(&self) -> Vec<AuditEvent> {
        self.rows.lock().expect("the audit lock holds").clone()
    }
}

#[async_trait]
impl AuditLog for InMemoryAudit {
    async fn record(&self, event: AuditEvent) -> Result<(), RbacError> {
        self.rows.lock().expect("the audit lock holds").push(event);
        Ok(())
    }

    async fn recent(&self, limit: u64) -> Result<Vec<AuditEvent>, RbacError> {
        let rows = self.rows.lock().expect("the audit lock holds");
        let mut recent = rows.clone();
        recent.sort_by_key(|event| std::cmp::Reverse(event.recorded_at_ms));
        recent.truncate(usize::try_from(limit).unwrap_or(usize::MAX));
        Ok(recent)
    }
}
