#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Dependency unavailable: {0}")]
    DependencyUnavailable(String),
}
