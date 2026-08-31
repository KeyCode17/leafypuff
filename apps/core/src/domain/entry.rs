use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::mood::Mood;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntryId(pub Uuid);

impl EntryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for EntryId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub id: EntryId,
    pub date: String,
    pub mood: Mood,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
}
