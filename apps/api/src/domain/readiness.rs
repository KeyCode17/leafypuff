#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadinessReport {
    pub database: bool,
    pub object_storage: bool,
}

impl ReadinessReport {
    pub const fn is_ready(self) -> bool {
        self.database && self.object_storage
    }
}
