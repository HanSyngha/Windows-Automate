// LLM interaction commands

use crate::llm::runner::AgentResult;
use crate::llm::types::ActionResponse;
use tauri::{AppHandle, Manager};

/// Send message to LLM and run full agent loop
/// Returns complete execution result with all steps
#[tauri::command]
pub async fn send_message(
    app: AppHandle,
    message: &str,
    include_screen: Option<bool>,
) -> Result<AgentResult, String> {
    let with_screen = include_screen.unwrap_or(true);

    // Minimize main window before screen capture so it doesn't appear in screenshot
    if with_screen {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.minimize();
            // Small delay to ensure window is minimized
            tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
        }
    }

    let result = crate::llm::runner::run_agent_loop(message, with_screen)
        .await
        .map_err(|e| e.to_string());

    // Restore main window after processing
    if with_screen {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.unminimize();
            let _ = window.set_focus();
        }
    }

    result
}

/// Legacy single-turn message (for backward compatibility)
#[allow(dead_code)]
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
