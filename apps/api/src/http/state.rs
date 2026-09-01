use crate::application::admin::AdminServices;
use crate::application::catalog::CatalogServices;
use crate::application::iam::IamServices;
use crate::application::media::MediaServices;
use crate::application::rbac::RbacServices;
use crate::application::sync::SyncServices;
use crate::infrastructure::DependencyProbe;

#[derive(Clone)]
pub struct AppState {
    pub readiness: DependencyProbe,
    pub iam: IamServices,
    pub sync: SyncServices,
    pub media: MediaServices,
    pub rbac: RbacServices,
    pub admin: AdminServices,
    pub catalog: CatalogServices,
}

impl AppState {
    pub const fn new(
        readiness: DependencyProbe,
        iam: IamServices,
        sync: SyncServices,
        media: MediaServices,
        rbac: RbacServices,
        admin: AdminServices,
        catalog: CatalogServices,
    ) -> Self {
        Self {
            readiness,
            iam,
            sync,
            media,
            rbac,
            admin,
            catalog,
        }
    }
}
