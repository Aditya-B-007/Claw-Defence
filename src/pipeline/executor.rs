use std::sync::Arc;
use thiserror::Error;

use crate::filters::r#trait::{Filter, FilterError};
use crate::models::request::ChatRequest;
use crate::models::response::ChatResponse;
use crate::policy::engine::PolicyEngine;
use crate::policy::rules::PolicyDecision;
use crate::proxy::client::ProxyClient;

#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("Filter rejected request: {0}")]
    FilterFailed(#[from] FilterError),
    #[error("Policy rejected request: {0}")]
    PolicyFailed(String),
    #[error("Proxy execution failed: {0}")]
    ProxyFailed(String),
}

pub struct PipelineExecutor {
    proxy: Arc<ProxyClient>,
    input_filters: Vec<Arc<dyn Filter>>,
    policy_engine: Arc<PolicyEngine>,
}

impl PipelineExecutor {
    pub fn new(
        proxy: Arc<ProxyClient>,
        input_filters: Vec<Arc<dyn Filter>>,
        policy_engine: Arc<PolicyEngine>,
    ) -> Self {
        Self {
            proxy,
            input_filters,
            policy_engine,
        }
    }

    pub async fn execute(&self, mut req: ChatRequest) -> Result<ChatResponse, PipelineError> {
        // 1. Run Input Filters sequentially on all messages
        for message in req.messages.iter_mut() {
            let mut content = message.content.clone();
            
            for filter in &self.input_filters {
                content = filter.apply(content).await?;
            }
            
            message.content = content;
        }

        // 2. Run Policy Engine
        match self.policy_engine.evaluate(&req) {
            PolicyDecision::Allow => {}
            PolicyDecision::Block(reason) => return Err(PipelineError::PolicyFailed(reason)),
        }

        // 3. Forward to Proxy
        self.proxy
            .forward_chat(req)
            .await
            .map_err(|e| PipelineError::ProxyFailed(e.to_string()))
    }
}