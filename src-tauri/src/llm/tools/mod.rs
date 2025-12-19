// Tools module - primitive tools for system interaction

mod mouse;
mod keyboard;
mod screen;
mod scroll;
mod registry;

pub use mouse::*;
pub use keyboard::*;
pub use screen::*;
pub use scroll::*;

use crate::llm::types::{ToolContext, ToolDef, ToolResult};
use async_trait::async_trait;
use serde_json::Value;

/// Trait for all tools (both primitive and agent-based)
#[async_trait]
pub trait Tool: Send + Sync {
    /// Tool name (used in API calls)
    fn name(&self) -> &str;

    /// Tool description for LLM
    fn description(&self) -> &str;

    /// JSON schema for parameters
    fn parameters(&self) -> Value;

    /// Execute the tool with given parameters
    async fn execute(&self, params: Value, ctx: &ToolContext) -> anyhow::Result<ToolResult>;

    /// Convert to API tool definition
    fn to_tool_def(&self) -> ToolDef {
        ToolDef {
            tool_type: "function".to_string(),
            function: crate::llm::types::FunctionDef {
                name: self.name().to_string(),
                description: self.description().to_string(),
                parameters: self.parameters(),
            },
        }
    }
}
