use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Message in a conversation with the LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Response from the LLM server (OpenAI-compatible format)
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

/// Client for talking to a local LLM server (OpenAI-compatible API)
pub struct LlmClient {
    base_url: String,
    client: reqwest::Client,
}

impl LlmClient {
    pub fn new(base_url: &str) -> Self {
        LlmClient {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Check if the LLM server is reachable
    pub async fn is_available(&self) -> bool {
        self.client
            .get(format!("{}/v1/models", self.base_url))
            .timeout(std::time::Duration::from_secs(3))
            .send()
            .await
            .is_ok()
    }

    /// Send a conversation and get a response
    pub async fn chat(&self, messages: &[Message]) -> Result<String> {
        let body = serde_json::json!({
            "messages": messages,
            "temperature": 0.7,
            "max_tokens": 2048,
        });

        let response = self
            .client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&body)
            .send()
            .await
            .context("Failed to reach LLM server")?;

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Failed to parse LLM response")?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .context("LLM returned no response")
    }
}
