use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilterError {
    #[error("Prompt injection detected: {0}")]
    PromptInjection(String),
    #[error("Filter execution failed: {0}")]
    ExecutionError(String),
}

#[async_trait]
pub trait Filter: Send + Sync {
    fn name(&self) -> &str;
    async fn apply(&self, input: String) -> Result<String, FilterError>;
}