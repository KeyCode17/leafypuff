use leafypuff_core::domain::Mood;
use uuid::Uuid;

use super::envelope::FieldEnvelope;
use super::field::EncryptedField;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryRecord {
    pub id: Uuid,
    pub account_id: Uuid,
    pub date: String,
    pub mood: Mood,
    pub tags: Vec<String>,
    pub sticker_placements: String,
    pub revision: i64,
    pub device_updated_at_ms: i64,
    pub deleted_at_ms: Option<i64>,
    pub title: FieldEnvelope,
    pub body: FieldEnvelope,
}

impl EntryRecord {
    pub const fn is_tombstoned(&self) -> bool {
        self.deleted_at_ms.is_some()
    }

    pub const fn field(&self, field: EncryptedField) -> &FieldEnvelope {
        match field {
            EncryptedField::Title => &self.title,
            EncryptedField::Body => &self.body,
        }
    }

    pub const fn field_mut(&mut self, field: EncryptedField) -> &mut FieldEnvelope {
        match field {
            EncryptedField::Title => &mut self.title,
            EncryptedField::Body => &mut self.body,
        }
    }
}
