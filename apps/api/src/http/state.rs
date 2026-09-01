use crate::application::admin::AdminServices;
use crate::application::catalog::CatalogServices;
use crate::application::iam::IamServices;
use crate::application::media::MediaServices;
use crate::application::privacy::PrivacyServices;
use crate::application::rbac::RbacServices;
use crate::application::release::ReleaseServices;
use crate::application::sync::SyncServices;
use crate::infrastructure::DependencyProbe;

/// Built with a struct literal rather than a constructor. Every field is one wired service, and a
/// positional argument list of that length is a place to transpose two of them silently.
#[derive(Clone)]
pub struct AppState {
    pub readiness: DependencyProbe,
    pub iam: IamServices,
    pub sync: SyncServices,
    pub media: MediaServices,
    pub rbac: RbacServices,
    pub admin: AdminServices,
    pub catalog: CatalogServices,
    pub privacy: PrivacyServices,
    pub release: ReleaseServices,
}
