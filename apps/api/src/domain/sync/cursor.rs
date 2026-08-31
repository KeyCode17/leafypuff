#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SyncCursor(pub i64);

impl SyncCursor {
    pub const START: Self = Self(0);

    pub const fn advanced_to(self, revision: i64) -> Self {
        if revision > self.0 {
            Self(revision)
        } else {
            self
        }
    }
}
