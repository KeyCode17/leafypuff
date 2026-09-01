use serde::Serialize;
use uuid::Uuid;

/// The wire shape an operator sees. Every field is a count, a date or a flag; there is no field
/// here that could hold an entry's title, body, tag, mood or storage key, and a test asserts the
/// exact key set so adding one is a deliberate act rather than an accident.
#[derive(Serialize)]
pub struct AccountSummaryResponse {
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
