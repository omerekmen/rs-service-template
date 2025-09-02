use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Rule violated: {0}")]
    RuleViolated(String),
}
