use reqwest::Client;
use crate::config::AppConfig;
use crate::models::request::ChatRequest;
use crate::models::response::ChatResponse;

#[derive(Clone)]
pub struct ProxyClient {
    client: Client,
    upstream_url: String,
    api_key: String,
}

impl ProxyClient {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: Client::new(),
            upstream_url: config.upstream_url.clone(),
            api_key: config.api_key.clone(),
        }
    }

    pub async fn forward_chat(&self, req: ChatRequest) -> Result<ChatResponse, reqwest::Error> {
        let response = self.client
            .post(&self.upstream_url)
            .bearer_auth(&self.api_key)
            .json(&req)
            .send()
            .await?;

        response.error_for_status()?.json::<ChatResponse>().await
    }
}