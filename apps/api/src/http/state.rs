use crate::application::iam::IamServices;
use crate::application::media::MediaServices;
use crate::application::sync::SyncServices;
use crate::infrastructure::DependencyProbe;

#[derive(Clone)]
pub struct AppState {
    pub readiness: DependencyProbe,
    pub iam: IamServices,
    pub sync: SyncServices,
    pub media: MediaServices,
}

impl AppState {
    pub const fn new(
        readiness: DependencyProbe,
        iam: IamServices,
        sync: SyncServices,
        media: MediaServices,
    ) -> Self {
        Self {
            readiness,
            iam,
            sync,
            media,
        }
    }
}
