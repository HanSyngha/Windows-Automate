// Guide search sub-agent - uses LLM to autonomously search guides

use crate::guides::storage::{list_guides, preview_guide, read_guide, GuideEntry};
use crate::llm::client::chat_completion;
use crate::llm::prompts::GUIDE_SEARCH_AGENT_PROMPT;
use crate::llm::tools::Tool;
use crate::llm::types::{
    FunctionDef, Message, MessageContent, ToolContext, ToolDef, ToolResult,
};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};

/// Guide search agent tool - appears as a tool to the main agent
pub struct GuideSearchAgentTool;

#[async_trait]
impl Tool for GuideSearchAgentTool {
    fn name(&self) -> &str {
        "guide_search"
    }

    fn description(&self) -> &str {
        "Search for relevant guides to help with the current task. A sub-agent will autonomously search and return guide content or '없음' if not found."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search query describing what you need help with (e.g., 'YouTube upload', 'VSCode shortcuts')"
                }
            },
            "required": ["query"]
        })
    }

    async fn execute(&self, params: Value, ctx: &ToolContext) -> anyhow::Result<ToolResult> {
        let query = params["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing query"))?;

        match run_guide_search_agent(query, ctx).await {
            Ok(result) => Ok(ToolResult::success(result)),
            Err(e) => Ok(ToolResult::error(e.to_string())),
        }
    }
}

/// Get tools available to the guide search sub-agent
fn get_guide_tools() -> Vec<ToolDef> {
    vec![
        ToolDef {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "guide_ls".to_string(),
                description: "List guides in a directory. Omit path for root.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Directory path (e.g., 'websites/'). Empty for root."
                        }
                    },
                    "required": []
                }),
            },
        },
        ToolDef {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "guide_preview".to_string(),
                description: "Preview first 10 lines of a guide file.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "Path to the guide file (e.g., 'websites/youtube-upload.md')"
                        }
                    },
                    "required": ["file_path"]
                }),
            },
        },
        ToolDef {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "guide_read".to_string(),
                description: "Read full content of a guide file.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "Path to the guide file (e.g., 'websites/youtube-upload.md')"
                        }
                    },
                    "required": ["file_path"]
                }),
            },
        },
    ]
}

/// Execute a guide tool call
fn execute_guide_tool(name: &str, args: &Value) -> String {
    match name {
        "guide_ls" => {
            let path = args
                .get("path")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty());

            match list_guides(path) {
                Ok(entries) => format_ls_result(&entries),
                Err(e) => format!("Error: {}", e),
            }
        }
        "guide_preview" => {
            let file_path = args.get("file_path").and_then(|v| v.as_str()).unwrap_or("");

            match preview_guide(file_path) {
                Ok(content) => content,
                Err(e) => format!("Error: {}", e),
            }
        }
        "guide_read" => {
            let file_path = args.get("file_path").and_then(|v| v.as_str()).unwrap_or("");

            match read_guide(file_path) {
                Ok(content) => content,
                Err(e) => format!("Error: {}", e),
            }
        }
        _ => format!("Unknown tool: {}", name),
    }
}

/// Format ls result for the agent
fn format_ls_result(entries: &[GuideEntry]) -> String {
    if entries.is_empty() {
        return "(empty directory)".to_string();
    }

    entries
        .iter()
        .map(|e| {
            if e.is_dir {
                format!("{}/", e.name)
            } else {
                e.name.clone()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Run the guide search sub-agent loop
async fn run_guide_search_agent(query: &str, ctx: &ToolContext) -> Result<String> {
    let tools = get_guide_tools();

    // Initial messages
    let mut messages = vec![
        Message {
            role: "system".to_string(),
            content: MessageContent::Text(GUIDE_SEARCH_AGENT_PROMPT.to_string()),
            tool_call_id: None,
        },
        Message {
            role: "user".to_string(),
            content: MessageContent::Text(format!("Find a guide for: {}", query)),
            tool_call_id: None,
        },
    ];

    // Agent loop - max 10 iterations to prevent infinite loops
    for _ in 0..10 {
        let response = chat_completion(&ctx.api_config, messages.clone(), Some(tools.clone())).await?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from API"))?;

        // Check if there are tool calls
        if let Some(tool_calls) = &choice.message.tool_calls {
            // Add assistant message with tool calls
            messages.push(Message {
                role: "assistant".to_string(),
                content: MessageContent::Text(choice.message.content.clone().unwrap_or_default()),
                tool_call_id: None,
            });

            // Execute each tool call and add results
            for tool_call in tool_calls {
                let args: Value =
                    serde_json::from_str(&tool_call.function.arguments).unwrap_or(json!({}));

                let result = execute_guide_tool(&tool_call.function.name, &args);

                messages.push(Message {
                    role: "tool".to_string(),
                    content: MessageContent::Text(result),
                    tool_call_id: Some(tool_call.id.clone()),
                });
            }
        } else {
            // No tool calls - agent is done, return the response
            let response_text = choice
                .message
                .content
                .clone()
                .unwrap_or_else(|| "없음".to_string());

            return Ok(response_text);
        }
    }

    // Max iterations reached
    Ok("없음".to_string())
}
