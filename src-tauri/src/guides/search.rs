// Guide search - Sub-LLM agent for autonomous guide exploration

use crate::config::storage::load_config;
use crate::guides::storage::{list_guides, preview_guide, read_guide, GuideEntry};
use crate::llm::client::{chat_completion, FunctionDef, Message, MessageContent, Tool};
use anyhow::Result;
use serde_json::json;

/// System prompt for the guide search sub-agent
const GUIDE_SEARCH_SYSTEM_PROMPT: &str = r#"You are a guide search agent. Your task is to find relevant guides based on the user's query.

You have access to the following tools:
- guide_ls(path): List guides in a directory. Use "" or omit path to list root folders.
- guide_preview(file_path): Read first 10 lines of a guide file.
- guide_read(file_path): Read the full content of a guide file.

Search Strategy:
1. Use guide_ls() to see available folders (categories like websites/, applications/, workflows/)
2. Based on the query, identify which folder might contain relevant guides
3. Use guide_ls("folder_name/") to see files in that folder
4. Use guide_preview to check if a file matches the query
5. If it matches, use guide_read to get the full content

Response Rules:
- If you find a relevant guide, respond with the FULL guide content only (no extra text)
- If no relevant guide exists, respond with exactly: 없음
- Do not add explanations or commentary to your response
- Be efficient - don't read files that are clearly unrelated based on their names"#;

/// Get tools for the guide search sub-agent
fn get_guide_tools() -> Vec<Tool> {
    vec![
        Tool {
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
        Tool {
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
        Tool {
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
fn execute_guide_tool(name: &str, args: &serde_json::Value) -> String {
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
            let file_path = args
                .get("file_path")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            match preview_guide(file_path) {
                Ok(content) => content,
                Err(e) => format!("Error: {}", e),
            }
        }
        "guide_read" => {
            let file_path = args
                .get("file_path")
                .and_then(|v| v.as_str())
                .unwrap_or("");

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

/// Search for guides using Sub-LLM agent
pub async fn search_guide(query: &str) -> Result<String> {
    let config = load_config()?;

    if config.api.api_key.is_empty() {
        return Err(anyhow::anyhow!("API key not configured"));
    }

    let tools = get_guide_tools();

    // Initial messages
    let mut messages = vec![
        Message {
            role: "system".to_string(),
            content: MessageContent::Text(GUIDE_SEARCH_SYSTEM_PROMPT.to_string()),
        },
        Message {
            role: "user".to_string(),
            content: MessageContent::Text(format!("Find a guide for: {}", query)),
        },
    ];

    // Agent loop - max 10 iterations to prevent infinite loops
    for _ in 0..10 {
        let response = chat_completion(&config.api, messages.clone(), Some(tools.clone())).await?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from API"))?;

        // Check if there are tool calls
        if let Some(tool_calls) = &choice.message.tool_calls {
            // Add assistant message with tool calls
            messages.push(Message {
                role: "assistant".to_string(),
                content: MessageContent::Text(
                    choice.message.content.clone().unwrap_or_default(),
                ),
            });

            // Execute each tool call and add results
            for tool_call in tool_calls {
                let args: serde_json::Value =
                    serde_json::from_str(&tool_call.function.arguments).unwrap_or(json!({}));

                let result = execute_guide_tool(&tool_call.function.name, &args);

                messages.push(Message {
                    role: "tool".to_string(),
                    content: MessageContent::Text(result),
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
