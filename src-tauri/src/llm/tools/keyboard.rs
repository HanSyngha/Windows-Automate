// Keyboard tools - text input and key presses

use super::Tool;
use crate::llm::types::{ToolContext, ToolResult};
use async_trait::async_trait;
use serde_json::{json, Value};

/// Keyboard type tool - types text
pub struct KeyboardTypeTool;

#[async_trait]
impl Tool for KeyboardTypeTool {
    fn name(&self) -> &str {
        "keyboard_type"
    }

    fn description(&self) -> &str {
        "Type text using keyboard"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "text": { "type": "string", "description": "Text to type" }
            },
            "required": ["text"]
        })
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> anyhow::Result<ToolResult> {
        let text = params["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing text"))?;

        #[cfg(windows)]
        {
            crate::input::keyboard::type_text(text, 30)?;
        }

        Ok(ToolResult::success(format!("Typed: {}", text)))
    }
}

/// Keyboard press tool - presses key combinations
pub struct KeyboardPressTool;

#[async_trait]
impl Tool for KeyboardPressTool {
    fn name(&self) -> &str {
        "keyboard_press"
    }

    fn description(&self) -> &str {
        "Press key or key combination"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "keys": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Keys to press (e.g., ['ctrl', 'c'])"
                }
            },
            "required": ["keys"]
        })
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> anyhow::Result<ToolResult> {
        let keys: Vec<String> = params["keys"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Missing keys"))?
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();

        #[cfg(windows)]
        {
            crate::input::keyboard::press_keys(&keys)?;
        }

        Ok(ToolResult::success(format!("Pressed: {:?}", keys)))
    }
}
