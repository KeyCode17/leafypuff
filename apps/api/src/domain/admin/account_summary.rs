use uuid::Uuid;

/// What an operator is allowed to see about someone. Counts and dates, never a title, a body, a
/// tag, a mood or a storage key — the server could not read the first two even if this said
/// otherwise, and the rest are the person's, not the operator's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountSummary {
    pub account_id: Uuid,
    pub email: String,
    pub verified: bool,
    pub suspended: bool,
    pub entry_count: i64,
    pub first_entry_date: Option<String>,
    pub last_entry_date: Option<String>,
    pub media_object_count: i64,
    pub media_bytes: i64,
}
