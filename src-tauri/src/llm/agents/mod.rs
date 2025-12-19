// Agents module - sub-agents that use LLM internally

mod guide_search;

pub use guide_search::*;

use crate::llm::tools::Tool;
use crate::llm::types::ToolDef;
use std::sync::Arc;

/// Get all agent tools
pub fn get_agent_tools() -> Vec<Arc<dyn Tool>> {
    vec![Arc::new(GuideSearchAgentTool)]
}

/// Get agent tool definitions for API
pub fn get_agent_tool_defs() -> Vec<ToolDef> {
    get_agent_tools().iter().map(|t| t.to_tool_def()).collect()
}
