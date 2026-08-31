use crate::infrastructure::DependencyProbe;

#[derive(Clone)]
pub struct AppState {
    pub readiness: DependencyProbe,
}

impl AppState {
    pub const fn new(readiness: DependencyProbe) -> Self {
        Self { readiness }
    }
}
