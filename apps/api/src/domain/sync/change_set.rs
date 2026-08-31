use std::fmt;

use super::cursor::SyncCursor;
use super::entry_record::EntryRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSet {
    pub records: Vec<EntryRecord>,
    pub cursor: SyncCursor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyKind {
    Passphrase,
    Recovery,
}

impl KeyKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passphrase => "passphrase",
            Self::Recovery => "recovery",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "passphrase" => Some(Self::Passphrase),
            "recovery" => Some(Self::Recovery),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct WrappedKeyRow {
    pub kind: KeyKind,
    pub blob: Vec<u8>,
    pub salt: Vec<u8>,
    pub updated_at_ms: i64,
}

impl fmt::Debug for WrappedKeyRow {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WrappedKeyRow")
            .field("kind", &self.kind)
            .field("blob_len", &self.blob.len())
            .field("salt_len", &self.salt.len())
            .field("updated_at_ms", &self.updated_at_ms)
            .finish()
    }
}
