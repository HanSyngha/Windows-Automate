// OpenAI-compatible API client

use crate::commands::config::ApiConfig;
use crate::llm::types::{Message, ToolDef};
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Chat completion request
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ToolDef>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// Chat completion response
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
    #[serde(default)]
    #[allow(dead_code)]
    pub usage: Option<Usage>,
}

/// Response choice
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
    #[allow(dead_code)]
    pub finish_reason: Option<String>,
}

/// Response message
#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    #[allow(dead_code)]
    pub role: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<crate::llm::types::ToolCall>>,
}

/// Token usage
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Test API connection
pub async fn test_connection(config: &ApiConfig) -> Result<String> {
    use crate::llm::types::MessageContent;

    let client = Client::new();

    let request = ChatRequest {
        model: config.model.clone(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::Text("Say 'Hello' in one word.".to_string()),
            tool_call_id: None,
        }],
        tools: None,
        tool_choice: None,
        max_tokens: 10,
        temperature: 0.0,
    };

    let response = client
        .post(format!(
            "{}/chat/completions",
            config.endpoint.trim_end_matches('/')
        ))
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let chat_response: ChatResponse = response.json().await?;
        let reply = chat_response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_else(|| "Connected".to_string());
        Ok(reply)
    } else {
        let error_text = response.text().await?;
        Err(anyhow::anyhow!("API error: {}", error_text))
    }
}

/// Send chat completion request
pub async fn chat_completion(
    config: &ApiConfig,
    messages: Vec<Message>,
    tools: Option<Vec<ToolDef>>,
) -> Result<ChatResponse> {
    let client = Client::new();

    let request = ChatRequest {
        model: config.model.clone(),
        messages,
        tools,
        tool_choice: Some("auto".to_string()),
        max_tokens: config.max_tokens,
        temperature: config.temperature,
    };

    let response = client
        .post(format!(
            "{}/chat/completions",
            config.endpoint.trim_end_matches('/')
        ))
        .header("Authorization", format!("Bearer {}", config.api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let chat_response: ChatResponse = response.json().await?;
        Ok(chat_response)
    } else {
        let error_text = response.text().await?;
        Err(anyhow::anyhow!("API error: {}", error_text))
    }
}
