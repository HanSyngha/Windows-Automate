// Mouse tools - mouse movement and clicking

use super::Tool;
use crate::llm::types::{ToolContext, ToolResult};
use async_trait::async_trait;
use serde_json::{json, Value};

/// Mouse move tool
pub struct MouseMoveTool;

#[async_trait]
impl Tool for MouseMoveTool {
    fn name(&self) -> &str {
        "mouse_move"
    }

    fn description(&self) -> &str {
        "Move mouse cursor to specified coordinates"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "x": { "type": "integer", "description": "X coordinate" },
                "y": { "type": "integer", "description": "Y coordinate" }
            },
            "required": ["x", "y"]
        })
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> anyhow::Result<ToolResult> {
        let x = params["x"].as_i64().ok_or_else(|| anyhow::anyhow!("Missing x"))? as i32;
        let y = params["y"].as_i64().ok_or_else(|| anyhow::anyhow!("Missing y"))? as i32;

        #[cfg(windows)]
        {
            crate::input::mouse::smooth_move(x, y, 300)?;
        }

        Ok(ToolResult::success(format!("Moved mouse to ({}, {})", x, y)))
    }
}

/// Mouse click tool
pub struct MouseClickTool;

#[async_trait]
impl Tool for MouseClickTool {
    fn name(&self) -> &str {
        "mouse_click"
    }

    fn description(&self) -> &str {
        "Click at specified coordinates"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "x": { "type": "integer", "description": "X coordinate" },
                "y": { "type": "integer", "description": "Y coordinate" },
                "button": {
                    "type": "string",
                    "enum": ["left", "right", "middle"],
                    "default": "left"
                }
            },
            "required": ["x", "y"]
        })
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> anyhow::Result<ToolResult> {
        let x = params["x"].as_i64().ok_or_else(|| anyhow::anyhow!("Missing x"))? as i32;
        let y = params["y"].as_i64().ok_or_else(|| anyhow::anyhow!("Missing y"))? as i32;
        let button_str = params["button"].as_str().unwrap_or("left");

        #[cfg(windows)]
        {
            use crate::commands::input::MouseButton;
            let button = match button_str {
                "right" => MouseButton::Right,
                "middle" => MouseButton::Middle,
                _ => MouseButton::Left,
            };
            crate::input::mouse::click(x, y, button, false)?;
        }

        Ok(ToolResult::success(format!("Clicked {} at ({}, {})", button_str, x, y)))
    }
}

/// Mouse double click tool
pub struct MouseDoubleClickTool;

#[async_trait]
impl Tool for MouseDoubleClickTool {
    fn name(&self) -> &str {
        "mouse_double_click"
    }

    fn description(&self) -> &str {
        "Double click at specified coordinates"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "x": { "type": "integer", "description": "X coordinate" },
                "y": { "type": "integer", "description": "Y coordinate" }
            },
            "required": ["x", "y"]
        })
    }

    async fn execute(&self, params: Value, _ctx: &ToolContext) -> anyhow::Result<ToolResult> {
        let x = params["x"].as_i64().ok_or_else(|| anyhow::anyhow!("Missing x"))? as i32;
        let y = params["y"].as_i64().ok_or_else(|| anyhow::anyhow!("Missing y"))? as i32;

        #[cfg(windows)]
        {
            use crate::commands::input::MouseButton;
            crate::input::mouse::click(x, y, MouseButton::Left, true)?;
        }

        Ok(ToolResult::success(format!("Double-clicked at ({}, {})", x, y)))
    }
}
