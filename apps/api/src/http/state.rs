use crate::application::iam::IamServices;
use crate::infrastructure::DependencyProbe;

#[derive(Clone)]
pub struct AppState {
    pub readiness: DependencyProbe,
    pub iam: IamServices,
}

impl AppState {
    pub const fn new(readiness: DependencyProbe, iam: IamServices) -> Self {
        Self { readiness, iam }
    }
}
