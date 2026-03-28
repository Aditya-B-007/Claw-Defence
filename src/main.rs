use axum::{routing::post, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub mod config;
pub mod filters;

pub mod proxy {
    pub mod client;
    pub mod handler;
}

pub mod policy {
    pub mod engine;
    pub mod rules;
}

pub mod detection {
    pub mod analyzer;
    pub mod heuristics;
}

pub mod pipeline {
    pub mod executor;
}

pub mod models {
    pub mod request;
    pub mod response;
}

pub mod utils {
    pub mod logger;
}

use crate::config::AppConfig;
use crate::pipeline::executor::PipelineExecutor;
use crate::proxy::client::ProxyClient;
use crate::proxy::handler::handle_chat;

#[tokio::main]
async fn main() {
    utils::logger::init();
    info!("Starting Claw Defence Gateway");

    let config = AppConfig::load();
    let proxy_client = Arc::new(ProxyClient::new(&config));
    let pipeline_executor = Arc::new(PipelineExecutor::new(proxy_client));

    let app = Router::new()
        .route("/v1/chat", post(handle_chat))
        .with_state(pipeline_executor);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind to port");
    
    info!("Listening on {}", addr);
    axum::serve(listener, app).await.expect("Server failed");
}