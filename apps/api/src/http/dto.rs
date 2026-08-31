use serde::Serialize;

use crate::domain::ReadinessReport;

#[derive(Serialize)]
pub struct ReadinessResponse {
    pub database: bool,
    pub object_storage: bool,
}

impl From<ReadinessReport> for ReadinessResponse {
    fn from(report: ReadinessReport) -> Self {
        Self {
            database: report.database,
            object_storage: report.object_storage,
        }
    }
}
