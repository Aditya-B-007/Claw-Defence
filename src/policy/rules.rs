use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub version: String,
    pub max_tokens: Option<u32>,
    pub blocked_patterns: Vec<String>,
}

#[derive(Debug)]
pub enum PolicyDecision {
    Allow,
    Block(String),
}