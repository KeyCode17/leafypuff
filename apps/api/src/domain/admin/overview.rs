/// The numbers an operator needs to know the service is healthy. Every figure is a count or a
/// byte total; none of them is derived from anything the server could read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceOverview {
    pub account_count: i64,
    pub verified_account_count: i64,
    pub suspended_account_count: i64,
    pub entry_count: i64,
    pub tombstoned_entry_count: i64,
    pub device_count: i64,
    pub devices_synced_last_day: i64,
    pub field_conflict_count: i64,
    pub media_object_count: i64,
    pub media_bytes: i64,
}
