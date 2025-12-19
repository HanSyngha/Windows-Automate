// Tool registry - manages all available tools

use super::{
    Tool, MouseMoveTool, MouseClickTool, MouseDoubleClickTool,
    KeyboardTypeTool, KeyboardPressTool, ScreenUpdateTool, WaitTool, ScrollTool,
};
use crate::llm::types::ToolDef;
use std::collections::HashMap;
use std::sync::Arc;

/// Registry of all available tools
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    /// Create a new registry with all primitive tools
    pub fn new() -> Self {
        let mut tools: HashMap<String, Arc<dyn Tool>> = HashMap::new();

        // Mouse tools
        tools.insert("mouse_move".to_string(), Arc::new(MouseMoveTool));
        tools.insert("mouse_click".to_string(), Arc::new(MouseClickTool));
        tools.insert("mouse_double_click".to_string(), Arc::new(MouseDoubleClickTool));

        // Keyboard tools
        tools.insert("keyboard_type".to_string(), Arc::new(KeyboardTypeTool));
        tools.insert("keyboard_press".to_string(), Arc::new(KeyboardPressTool));

        // Screen tools
        tools.insert("get_screen_update".to_string(), Arc::new(ScreenUpdateTool));
        tools.insert("wait".to_string(), Arc::new(WaitTool));

        // Scroll tool
        tools.insert("scroll".to_string(), Arc::new(ScrollTool));

        Self { tools }
    }

    /// Register a new tool
    pub fn register(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    /// Get a tool by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name).cloned()
    }

    /// Get all tool definitions for API
    pub fn get_tool_defs(&self) -> Vec<ToolDef> {
        self.tools.values().map(|t| t.to_tool_def()).collect()
    }

    /// List all tool names
    pub fn list_names(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Global tool registry instance
lazy_static::lazy_static! {
    pub static ref TOOL_REGISTRY: ToolRegistry = ToolRegistry::new();
}
