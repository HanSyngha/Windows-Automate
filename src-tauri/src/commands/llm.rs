// LLM interaction commands

use crate::llm::runner::AgentResult;
use crate::llm::types::ActionResponse;

/// Send message to LLM and run full agent loop
/// Returns complete execution result with all steps
#[tauri::command]
pub async fn send_message(
    message: &str,
    include_screen: Option<bool>,
) -> Result<AgentResult, String> {
    let with_screen = include_screen.unwrap_or(true);

    crate::llm::runner::run_agent_loop(message, with_screen)
        .await
        .map_err(|e| e.to_string())
}

/// Legacy single-turn message (for backward compatibility)
#[tauri::command]
pub async fn send_message_single(
    message: &str,
    include_screen: Option<bool>,
) -> Result<ActionResponse, String> {
    let with_screen = include_screen.unwrap_or(true);

    crate::llm::process_message(message, with_screen)
        .await
        .map_err(|e| e.to_string())
}
