use crate::http::validated::ValidatedBody;

use super::dto::AssignRoleRequest;

impl ValidatedBody for AssignRoleRequest {
    fn validate(&self) -> Result<(), &'static str> {
        if self.account_id.is_nil() || self.role_id.is_nil() {
            return Err("account_id and role_id must be real uuids");
        }
        Ok(())
    }
}
