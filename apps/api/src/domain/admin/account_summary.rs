use uuid::Uuid;

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
