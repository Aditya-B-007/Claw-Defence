use std::sync::Arc;
use crate::models::request::ChatRequest;
use crate::models::response::ChatResponse;
use crate::proxy::client::ProxyClient;

#[derive(Clone)]
pub struct PipelineExecutor {
    proxy: Arc<ProxyClient>,
}

impl PipelineExecutor {
    pub fn new(proxy: Arc<ProxyClient>) -> Self {
        Self { proxy }
    }

    pub async fn execute(&self, req: ChatRequest) -> Result<ChatResponse, String> {
        // Phase 1 MVP: Pass-through
        // In future phases:
        // 1. Run Input Filters
        // 2. Run Policy/Detection Engine
        // 3. Forward to Proxy
        // 4. Run Output Filters
        
        self.proxy
            .forward_chat(req)
            .await
            .map_err(|e| format!("Proxy error: {}", e))
    }
}