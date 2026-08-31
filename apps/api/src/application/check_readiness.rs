use crate::domain::{DomainError, ReadinessProbe, ReadinessReport};

pub struct CheckReadiness<P: ReadinessProbe> {
    probe: P,
}

impl<P: ReadinessProbe> CheckReadiness<P> {
    pub const fn new(probe: P) -> Self {
        Self { probe }
    }

    pub async fn execute(&self) -> Result<ReadinessReport, DomainError> {
        let report = self.probe.check().await?;
        if report.is_ready() {
            return Ok(report);
        }
        Err(DomainError::DependencyUnavailable(
            "One or more dependencies are not ready".to_owned(),
        ))
    }
}
