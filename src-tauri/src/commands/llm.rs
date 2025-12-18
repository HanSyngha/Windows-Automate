// LLM interaction commands

use serde::{Deserialize, Serialize};

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// LLM action response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResponse {
    pub thought: String,
    pub action: String,
    pub params: serde_json::Value,
}

/// Send message to LLM and get action response
#[tauri::command]
pub async fn send_message(
    message: &str,
    include_screen: Option<bool>,
) -> Result<ActionResponse, String> {
    let with_screen = include_screen.unwrap_or(true);

    crate::llm::agent::process_message(message, with_screen)
        .await
        .map_err(|e| e.to_string())
}
