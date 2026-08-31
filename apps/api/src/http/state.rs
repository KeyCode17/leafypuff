use crate::infrastructure::StaticReadinessProbe;

#[derive(Clone, Copy)]
pub struct AppState {
    pub readiness: StaticReadinessProbe,
}

impl AppState {
    pub const fn new(readiness: StaticReadinessProbe) -> Self {
        Self { readiness }
    }
}
