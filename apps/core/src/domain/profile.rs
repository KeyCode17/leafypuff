use super::crop::Framing;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Profile {
    pub display_name: Option<String>,
    pub avatar_photo_id: Option<String>,
    pub avatar_framing: Option<Framing>,
    pub updated_at_ms: i64,
}

impl Profile {
    pub const fn supersedes(&self, stored: &Self) -> bool {
        self.updated_at_ms > stored.updated_at_ms
    }

    pub fn wants_avatar(&self) -> bool {
        self.avatar_photo_id.is_some()
    }
}
