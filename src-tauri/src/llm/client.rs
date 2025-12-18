// OpenAI-compatible API client

use crate::commands::config::ApiConfig;
use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Chat message for API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: MessageContent,
}

/// Message content (text or multimodal)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    Text(String),
    Parts(Vec<ContentPart>),
}

/// Content part for multimodal messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContentPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "image_url")]
    ImageUrl { image_url: ImageUrl },
}

/// Image URL structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUrl {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// Chat completion request
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,
    pub max_tokens: u32,
    pub temperature: f32,
}

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionDef,
}

/// Function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Chat completion response
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub usage: Option<Usage>,
}

/// Response choice
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
    pub finish_reason: Option<String>,
}

/// Response message
#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    pub role: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Tool call in response
#[derive(Debug, Clone, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub call_type: String,
    pub function: FunctionCall,
}

/// Function call details
#[derive(Debug, Clone, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// Token usage
#[derive(Debug, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Test API connection
pub async fn test_connection(config: &ApiConfig) -> Result<String> {
    let client = Client::new();

    let request = ChatRequest {
        model: config.model.clone(),
        messages: vec![Message {
            role: "user".to_string(),
            content: MessageContent::Text("Say 'Hello' in one word.".to_string()),
        }],
        tools: None,
        tool_choice: None,
        max_tokens: 10,
        temperature: 0.0,
    };

    let response = client
        .post(format!("{}/chat/completions", config.endpoint.trim_end_matches('/')))
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
    tools: Option<Vec<Tool>>,
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
        .post(format!("{}/chat/completions", config.endpoint.trim_end_matches('/')))
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
