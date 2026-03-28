use async_trait::async_trait;
use crate::filters::r#trait::{Filter, FilterError};

pub struct PromptInjectionFilter {
    blocked_phrases: Vec<&'static str>,
}

impl PromptInjectionFilter {
    pub fn new() -> Self {
        Self {
            blocked_phrases: vec![
                "ignore previous instructions",
                "reveal system prompt",
            ],
        }
    }
}

#[async_trait]
impl Filter for PromptInjectionFilter {
    fn name(&self) -> &str {
        "PromptInjectionFilter"
    }

    async fn apply(&self, input: String) -> Result<String, FilterError> {
        let lower_input = input.to_lowercase();
        
        for phrase in &self.blocked_phrases {
            if lower_input.contains(phrase) {
                return Err(FilterError::PromptInjection(phrase.to_string()));
            }
        }
        
        Ok(input)
    }
}