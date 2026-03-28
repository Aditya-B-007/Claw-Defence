use std::fs;
use thiserror::Error;
use crate::models::request::ChatRequest;
use crate::policy::rules::{PolicyConfig, PolicyDecision};

#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("Failed to load policy configuration: {0}")]
    LoadError(String),
}

pub struct PolicyEngine {
    config: PolicyConfig,
}

impl PolicyEngine {
    pub fn new(config_path: &str) -> Result<Self, PolicyError> {
        let file_content = fs::read_to_string(config_path)
            .map_err(|e| PolicyError::LoadError(e.to_string()))?;
            
        let config: PolicyConfig = serde_yaml::from_str(&file_content)
            .map_err(|e| PolicyError::LoadError(e.to_string()))?;
            
        Ok(Self { config })
    }

    pub fn evaluate(&self, req: &ChatRequest) -> PolicyDecision {
        // Enforce max_tokens
        if let Some(limit) = self.config.max_tokens {
            let requested_tokens = req.max_tokens.unwrap_or(0);
            if requested_tokens > limit {
                return PolicyDecision::Block(format!("Request exceeds max_tokens limit of {}", limit));
            }
        }

        // Enforce blocked patterns via Policy Config
        for message in &req.messages {
            let lower_content = message.content.to_lowercase();
            for pattern in &self.config.blocked_patterns {
                if lower_content.contains(&pattern.to_lowercase()) {
                    return PolicyDecision::Block(format!("Message violates policy pattern: {}", pattern));
                }
            }
        }

        PolicyDecision::Allow
    }
}