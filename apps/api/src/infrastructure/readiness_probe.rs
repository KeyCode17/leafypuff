use crate::domain::{DomainError, ReadinessProbe, ReadinessReport};

#[derive(Debug, Clone, Copy)]
pub struct StaticReadinessProbe {
    report: ReadinessReport,
}

impl StaticReadinessProbe {
    pub const fn new(report: ReadinessReport) -> Self {
        Self { report }
    }
}

impl ReadinessProbe for StaticReadinessProbe {
    async fn check(&self) -> Result<ReadinessReport, DomainError> {
        Ok(self.report)
    }
}
