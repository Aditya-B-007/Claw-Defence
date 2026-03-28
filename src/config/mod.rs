use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub upstream_url: String,
    pub api_key: String,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenvy::dotenv().ok(); // Load from .env if present

        Self {
            host: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            upstream_url: env::var("UPSTREAM_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1/chat/completions".to_string()),
            api_key: env::var("UPSTREAM_API_KEY").expect("UPSTREAM_API_KEY environment variable is required"),
        }
    }
}