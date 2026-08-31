use crate::application::iam::IamServices;
use crate::application::sync::SyncServices;
use crate::infrastructure::DependencyProbe;

#[derive(Clone)]
pub struct AppState {
    pub readiness: DependencyProbe,
    pub iam: IamServices,
    pub sync: SyncServices,
}

impl AppState {
    pub const fn new(readiness: DependencyProbe, iam: IamServices, sync: SyncServices) -> Self {
        Self {
            readiness,
            iam,
            sync,
        }
    }
}
