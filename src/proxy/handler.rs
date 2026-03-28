use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use tracing::error;

use crate::models::request::ChatRequest;
use crate::pipeline::executor::{PipelineError, PipelineExecutor};

pub async fn handle_chat(
    State(executor): State<Arc<PipelineExecutor>>,
    Json(payload): Json<ChatRequest>,
) -> Result<Response, StatusCode> {
    match executor.execute(payload).await {
        Ok(response) => Ok(Json(response).into_response()),
        Err(e) => {
            error!("Pipeline execution failed: {}", e);
            
            match e {
                PipelineError::FilterFailed(_) | PipelineError::PolicyFailed(_) => {
                    Err(StatusCode::BAD_REQUEST)
                }
                PipelineError::ProxyFailed(_) => {
                    Err(StatusCode::BAD_GATEWAY)
                }
            }
        }
    }
}