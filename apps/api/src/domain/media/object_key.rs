use std::fmt;

use uuid::Uuid;

use super::variant::Variant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectKey {
    account_id: Uuid,
    photo_id: Uuid,
    variant: Variant,
}

impl ObjectKey {
    pub const fn new(account_id: Uuid, photo_id: Uuid, variant: Variant) -> Self {
        Self {
            account_id,
            photo_id,
            variant,
        }
    }

    pub const fn account_id(&self) -> Uuid {
        self.account_id
    }

    pub const fn photo_id(&self) -> Uuid {
        self.photo_id
    }

    pub const fn variant(&self) -> Variant {
        self.variant
    }
}

impl fmt::Display for ObjectKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "accounts/{}/photos/{}/{}",
            self.account_id.simple(),
            self.photo_id.simple(),
            self.variant.as_str()
        )
    }
}
