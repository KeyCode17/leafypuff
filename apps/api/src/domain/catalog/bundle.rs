use uuid::Uuid;

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
