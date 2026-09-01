use uuid::Uuid;

/// A catalog bundle is opaque json the server stores and hands back. The twelve moods and eight
/// stickers stay compiled into the app; this is how a later addition reaches a device that has
/// already shipped, not how the built-in set is defined.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogBundle {
    pub id: Uuid,
    pub version: i32,
    pub payload: String,
    pub published_at_ms: Option<i64>,
    pub published_by: Option<Uuid>,
    pub created_at_ms: i64,
}

impl CatalogBundle {
    pub const fn is_published(&self) -> bool {
        self.published_at_ms.is_some()
    }
}
