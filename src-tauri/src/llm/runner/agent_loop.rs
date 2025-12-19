// Agent loop - main agent processing loop with tool execution

use crate::config::storage::load_config;
use crate::llm::agents::get_agent_tool_defs;
use crate::llm::client::chat_completion;
use crate::llm::prompts::build_main_agent_prompt;
use crate::llm::runner::ToolExecutor;
use crate::llm::tools::{
    KeyboardPressTool, KeyboardTypeTool, MouseClickTool, MouseDoubleClickTool, MouseMoveTool,
    ScreenUpdateTool, ScrollTool, Tool, WaitTool,
};
use crate::llm::types::{
    ActionResponse, ContentPart, ImageUrl, Message, MessageContent, ToolContext, ToolDef,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Maximum iterations to prevent infinite loops
const MAX_ITERATIONS: usize = 20;

/// Single step result in the agent loop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentStep {
    pub thought: String,
    pub action: String,
    pub params: serde_json::Value,
    pub result: Option<String>,
}

/// Full agent execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub steps: Vec<AgentStep>,
    pub final_response: String,
    pub success: bool,
}

/// Get all tool definitions (primitives + agents) for API
fn get_all_tool_defs() -> Vec<ToolDef> {
    let primitive_tools: Vec<Box<dyn Tool>> = vec![
        Box::new(MouseMoveTool),
        Box::new(MouseClickTool),
        Box::new(MouseDoubleClickTool),
        Box::new(KeyboardTypeTool),
        Box::new(KeyboardPressTool),
        Box::new(ScreenUpdateTool),
        Box::new(WaitTool),
        Box::new(ScrollTool),
    ];

    let mut defs: Vec<ToolDef> = primitive_tools.iter().map(|t| t.to_tool_def()).collect();
    defs.extend(get_agent_tool_defs());
    defs
}

/// Capture current screen state (Windows only)
#[cfg(windows)]
fn capture_screen_context() -> Result<(String, String)> {
    let screenshot = crate::screen::capture::capture_screen_base64()?;
    let ui_tree = crate::screen::ui_automation::get_active_window_tree(2)
        .map(|t| serde_json::to_string_pretty(&t).unwrap_or_default())
        .unwrap_or_default();
    Ok((screenshot, ui_tree))
}

/// Run the main agent loop
/// - Calls LLM, executes tools, feeds results back
/// - Continues until no tool call or max iterations
pub async fn run_agent_loop(user_message: &str, include_screen: bool) -> Result<AgentResult> {
    let config = load_config()?;

    if config.api.api_key.is_empty() {
        return Err(anyhow::anyhow!("API key not configured"));
    }

    let ctx = ToolContext {
        api_config: config.api.clone(),
    };

    let executor = ToolExecutor::new();
    let tools = get_all_tool_defs();

    // Build initial messages
    let system_prompt = build_main_agent_prompt();
    let mut messages = vec![Message {
        role: "system".to_string(),
        content: MessageContent::Text(system_prompt),
        tool_call_id: None,
    }];

    // Add user message with optional screen capture
    if include_screen && config.api.supports_vision {
        #[cfg(windows)]
        {
            let (screenshot, ui_tree) = capture_screen_context()?;
            messages.push(Message {
                role: "user".to_string(),
                content: MessageContent::Parts(vec![
                    ContentPart::Text {
                        text: format!(
                            "Current screen state:\n\nUI Elements:\n{}\n\nUser request: {}",
                            ui_tree, user_message
                        ),
                    },
                    ContentPart::ImageUrl {
                        image_url: ImageUrl {
                            url: screenshot,
                            detail: Some("high".to_string()),
                        },
                    },
                ]),
                tool_call_id: None,
            });
        }
        #[cfg(not(windows))]
        {
            messages.push(Message {
                role: "user".to_string(),
                content: MessageContent::Text(user_message.to_string()),
                tool_call_id: None,
            });
        }
    } else {
        messages.push(Message {
            role: "user".to_string(),
            content: MessageContent::Text(user_message.to_string()),
            tool_call_id: None,
        });
    }

    let mut steps: Vec<AgentStep> = Vec::new();

    // Main agent loop
    for _iteration in 0..MAX_ITERATIONS {
        // Call LLM
        let response = chat_completion(&config.api, messages.clone(), Some(tools.clone())).await?;

        let choice = response
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No response from API"))?;

        let thought = choice
            .message
            .content
            .clone()
            .unwrap_or_else(|| String::new());

        // Check for tool calls
        if let Some(tool_calls) = &choice.message.tool_calls {
            if tool_calls.is_empty() {
                // No tool calls - agent is done
                return Ok(AgentResult {
                    steps,
                    final_response: thought,
                    success: true,
                });
            }

            // Add assistant message to history
            messages.push(Message {
                role: "assistant".to_string(),
                content: MessageContent::Text(thought.clone()),
                tool_call_id: None,
            });

            // Execute each tool call
            for tool_call in tool_calls {
                let tool_name = &tool_call.function.name;
                let params: serde_json::Value =
                    serde_json::from_str(&tool_call.function.arguments).unwrap_or(json!({}));

                // Execute the tool
                let tool_result = executor.execute(tool_name, params.clone(), &ctx).await;

                let result_text = match &tool_result {
                    Ok(r) => {
                        if r.success {
                            r.output.clone()
                        } else {
                            format!("Error: {}", r.error.clone().unwrap_or_default())
                        }
                    }
                    Err(e) => format!("Error: {}", e),
                };

                // Record the step
                steps.push(AgentStep {
                    thought: thought.clone(),
                    action: tool_name.clone(),
                    params: params.clone(),
                    result: Some(result_text.clone()),
                });

                // Add tool result to messages
                messages.push(Message {
                    role: "tool".to_string(),
                    content: MessageContent::Text(result_text),
                    tool_call_id: Some(tool_call.id.clone()),
                });

                // Special handling for get_screen_update - capture new screen
                if tool_name == "get_screen_update" {
                    #[cfg(windows)]
                    if config.api.supports_vision {
                        if let Ok((screenshot, ui_tree)) = capture_screen_context() {
                            messages.push(Message {
                                role: "user".to_string(),
                                content: MessageContent::Parts(vec![
                                    ContentPart::Text {
                                        text: format!("Updated screen state:\n\nUI Elements:\n{}", ui_tree),
                                    },
                                    ContentPart::ImageUrl {
                                        image_url: ImageUrl {
                                            url: screenshot,
                                            detail: Some("high".to_string()),
                                        },
                                    },
                                ]),
                                tool_call_id: None,
                            });
                        }
                    }
                }
            }

            // Continue to next iteration
        } else {
            // No tool calls - agent is done
            return Ok(AgentResult {
                steps,
                final_response: thought,
                success: true,
            });
        }
    }

    // Max iterations reached
    Ok(AgentResult {
        steps,
        final_response: "Maximum iterations reached. Task may be incomplete.".to_string(),
        success: false,
    })
}

/// Legacy single-turn function for backward compatibility
/// This calls the new loop but returns only the first action
#[allow(dead_code)]
pub async fn process_message(user_message: &str, include_screen: bool) -> Result<ActionResponse> {
    let result = run_agent_loop(user_message, include_screen).await?;

    if let Some(first_step) = result.steps.first() {
        Ok(ActionResponse {
            thought: first_step.thought.clone(),
            action: first_step.action.clone(),
            params: first_step.params.clone(),
        })
    } else {
        Ok(ActionResponse {
            thought: result.final_response,
            action: "none".to_string(),
            params: json!({}),
        })
    }
}
