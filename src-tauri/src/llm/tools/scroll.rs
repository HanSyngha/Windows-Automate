// Scroll tool
// Note: Scroll is not yet implemented in input/mouse.rs
// This is a placeholder that will be connected when scroll is implemented

use super::Tool;
use crate::llm::types::{ToolContext, ToolResult};
use async_trait::async_trait;
use serde_json::{json, Value};

/// Scroll tool
pub struct ScrollTool;

#[async_trait]
impl Tool for ScrollTool {
    fn name(&self) -> &str {
        "scroll"
    }

    fn description(&self) -> &str {
        "Scroll in specified direction"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "direction": {
                    "type": "string",
                    "enum": ["up", "down", "left", "right"]
                },
                "amount": { "type": "integer", "default": 3 }
            },
            "required": ["direction"]
        })
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> anyhow::Result<ToolResult> {
        let direction = params["direction"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing direction"))?;
        let amount = params["amount"].as_i64().unwrap_or(3) as i32;

        // TODO: Implement scroll in input/mouse.rs
        // For now, return success with a note
        Ok(ToolResult::success(format!(
            "Scroll {} by {} (not yet implemented)",
            direction, amount
        )))
    }
}
