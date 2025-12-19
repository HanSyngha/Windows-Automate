// Tool executor - executes tools by name

use crate::llm::agents::GuideSearchAgentTool;
use crate::llm::tools::{
    KeyboardPressTool, KeyboardTypeTool, MouseClickTool, MouseDoubleClickTool, MouseMoveTool,
    ScreenUpdateTool, ScrollTool, Tool, WaitTool,
};
use crate::llm::types::{ToolContext, ToolResult};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Tool executor - manages and executes all tools
pub struct ToolExecutor {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolExecutor {
    /// Create a new executor with all available tools
    pub fn new() -> Self {
        let mut tools: HashMap<String, Arc<dyn Tool>> = HashMap::new();

        // Primitive tools
        tools.insert("mouse_move".to_string(), Arc::new(MouseMoveTool));
        tools.insert("mouse_click".to_string(), Arc::new(MouseClickTool));
        tools.insert("mouse_double_click".to_string(), Arc::new(MouseDoubleClickTool));
        tools.insert("keyboard_type".to_string(), Arc::new(KeyboardTypeTool));
        tools.insert("keyboard_press".to_string(), Arc::new(KeyboardPressTool));
        tools.insert("get_screen_update".to_string(), Arc::new(ScreenUpdateTool));
        tools.insert("wait".to_string(), Arc::new(WaitTool));
        tools.insert("scroll".to_string(), Arc::new(ScrollTool));

        // Agent tools
        tools.insert("guide_search".to_string(), Arc::new(GuideSearchAgentTool));

        Self { tools }
    }

    /// Execute a tool by name
    pub async fn execute(
        &self,
        tool_name: &str,
        params: Value,
        ctx: &ToolContext,
    ) -> anyhow::Result<ToolResult> {
        let tool = self
            .tools
            .get(tool_name)
            .ok_or_else(|| anyhow::anyhow!("Unknown tool: {}", tool_name))?;

        tool.execute(params, ctx).await
    }

    /// Check if a tool exists
    #[allow(dead_code)]
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    /// Get all tool names
    #[allow(dead_code)]
    pub fn list_tools(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ToolExecutor {
    fn default() -> Self {
        Self::new()
    }
}
