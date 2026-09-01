use crate::domain::entry::EntryId;

use super::error::CryptoError;

pub const FIELD_TITLE: &str = "title";
pub const FIELD_BODY: &str = "body";
pub const FIELD_PHOTO: &str = "photo";
pub const FIELD_COVER: &str = "cover";

pub struct FieldContext<'a> {
    pub entry_id: EntryId,
    pub field_name: &'a str,
    pub field_updated_at_ms: i64,
}

impl FieldContext<'_> {
    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, CryptoError> {
        let name = self.field_name.as_bytes();
        if name.is_empty() {
            return Err(CryptoError::Payload);
        }
        let name_len = u8::try_from(name.len()).map_err(|_| CryptoError::Payload)?;
        let mut bytes = Vec::with_capacity(16 + 1 + name.len() + 8);
        bytes.extend_from_slice(self.entry_id.0.as_bytes());
        bytes.push(name_len);
        bytes.extend_from_slice(name);
        bytes.extend_from_slice(&self.field_updated_at_ms.to_le_bytes());
        Ok(bytes)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{FIELD_BODY, FIELD_TITLE, FieldContext};
    use crate::domain::entry::EntryId;

    #[test]
    fn different_fields_of_one_entry_produce_different_bytes() {
        let entry_id = EntryId::new();
        let title = FieldContext {
            entry_id,
            field_name: FIELD_TITLE,
            field_updated_at_ms: 1,
        };
        let body = FieldContext {
            entry_id,
            field_name: FIELD_BODY,
            field_updated_at_ms: 1,
        };
        assert_ne!(
            title.to_bytes().expect("aad"),
            body.to_bytes().expect("aad")
        );
    }

    #[test]
    fn a_field_name_boundary_cannot_be_shifted() {
        let entry_id = EntryId::new();
        let left = FieldContext {
            entry_id,
            field_name: "ab",
            field_updated_at_ms: 0,
        };
        let right = FieldContext {
            entry_id,
            field_name: "a",
            field_updated_at_ms: 0,
        };
        assert_ne!(
            left.to_bytes().expect("aad"),
            right.to_bytes().expect("aad")
        );
    }

    #[test]
    fn an_empty_field_name_is_rejected() {
        let context = FieldContext {
            entry_id: EntryId::new(),
            field_name: "",
            field_updated_at_ms: 0,
        };
        assert!(context.to_bytes().is_err());
    }
}
