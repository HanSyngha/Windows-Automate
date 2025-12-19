// Screen tools - screen updates and waiting

use super::Tool;
use crate::llm::types::{ToolContext, ToolResult};
use async_trait::async_trait;
use serde_json::{json, Value};

/// Screen update tool - requests updated screen information
pub struct ScreenUpdateTool;

#[async_trait]
impl Tool for ScreenUpdateTool {
    fn name(&self) -> &str {
        "get_screen_update"
    }

    fn description(&self) -> &str {
        "Request updated screen information"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {}
        })
    }

    async fn execute(&self, _params: Value, _ctx: &ToolContext) -> anyhow::Result<ToolResult> {
        // This is a signal to the runner to capture new screen
        Ok(ToolResult::success("Screen update requested"))
    }
}

/// Wait tool - pauses execution
pub struct WaitTool;

#[async_trait]
impl Tool for WaitTool {
    fn name(&self) -> &str {
        "wait"
    }

    fn description(&self) -> &str {
        "Wait for specified milliseconds"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "ms": { "type": "integer", "description": "Milliseconds to wait" }
            },
            "required": ["ms"]
        })
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> anyhow::Result<ToolResult> {
        let ms = params["ms"].as_u64().unwrap_or(1000);
        tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
        Ok(ToolResult::success(format!("Waited {}ms", ms)))
    }
}
