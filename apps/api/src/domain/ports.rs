use super::error::DomainError;
use super::readiness::ReadinessReport;

pub trait ReadinessProbe {
    fn check(&self) -> impl Future<Output = Result<ReadinessReport, DomainError>>;
}
