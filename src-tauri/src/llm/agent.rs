// Agent logic - processes user messages and executes actions

use crate::commands::llm::ActionResponse;
use crate::config::storage::load_config;
use crate::guides::storage::get_guide_index;
use crate::llm::client::{
    chat_completion, ContentPart, FunctionDef, ImageUrl, Message, MessageContent, Tool,
};
use anyhow::Result;
use serde_json::json;

/// Base system prompt for the AI agent
const BASE_SYSTEM_PROMPT: &str = r#"You are an AI assistant that helps users automate tasks on their Windows computer.
You can see the user's screen and control the mouse and keyboard to perform actions.

When given a task, analyze the current screen state and determine the best action to take.
Think step by step and explain your reasoning before taking action.

Available tools:
- mouse_move(x, y): Move mouse to coordinates
- mouse_click(x, y, button): Click at coordinates (button: "left", "right", "middle")
- mouse_double_click(x, y): Double click at coordinates
- keyboard_type(text): Type text
- keyboard_press(keys): Press key combination (e.g., ["ctrl", "c"])
- scroll(direction, amount): Scroll (direction: "up", "down", "left", "right")
- wait(ms): Wait for milliseconds
- get_screen_update(): Request updated screen information
- guide_search(query): Search for relevant guides to help with the current task

IMPORTANT:
- Always explain what you see on screen and what you plan to do
- Be precise with coordinates - click on the center of UI elements
- If you're unsure, ask for clarification
- Stop immediately if the user interrupts
- When performing unfamiliar tasks, use guide_search to find helpful guides first

Response format when taking action:
{
  "thought": "Explanation of what I see and plan to do",
  "action": "tool_name",
  "params": { ... }
}"#;

/// Build system prompt with guide index
fn build_system_prompt() -> String {
    let mut prompt = BASE_SYSTEM_PROMPT.to_string();

    // Add guide index if available
    if let Ok(index) = get_guide_index() {
        if !index.is_empty() {
            prompt.push_str("\n\n## Available Guides\n");
            prompt.push_str("The following guides are available. Use guide_search to retrieve relevant content:\n");
            for entry in &index {
                prompt.push_str(&format!("- {}: {}\n", entry.path, entry.title));
            }
        }
    }

    prompt
}

/// Get available tools definition
fn get_tools() -> Vec<Tool> {
    vec![
        Tool {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "mouse_move".to_string(),
                description: "Move mouse cursor to specified coordinates".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "x": { "type": "integer", "description": "X coordinate" },
                        "y": { "type": "integer", "description": "Y coordinate" }
                    },
                    "required": ["x", "y"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "mouse_click".to_string(),
                description: "Click at specified coordinates".to_string(),
                parameters: json!({
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
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "mouse_double_click".to_string(),
                description: "Double click at specified coordinates".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "x": { "type": "integer", "description": "X coordinate" },
                        "y": { "type": "integer", "description": "Y coordinate" }
                    },
                    "required": ["x", "y"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "keyboard_type".to_string(),
                description: "Type text using keyboard".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "text": { "type": "string", "description": "Text to type" }
                    },
                    "required": ["text"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "keyboard_press".to_string(),
                description: "Press key or key combination".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "keys": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Keys to press (e.g., ['ctrl', 'c'])"
                        }
                    },
                    "required": ["keys"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "scroll".to_string(),
                description: "Scroll in specified direction".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "direction": {
                            "type": "string",
                            "enum": ["up", "down", "left", "right"]
                        },
                        "amount": { "type": "integer", "default": 3 }
                    },
                    "required": ["direction"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "wait".to_string(),
                description: "Wait for specified milliseconds".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "ms": { "type": "integer", "description": "Milliseconds to wait" }
                    },
                    "required": ["ms"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "get_screen_update".to_string(),
                description: "Request updated screen information".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: FunctionDef {
                name: "guide_search".to_string(),
                description: "Search for relevant guides to help with the current task. A sub-agent will autonomously search and return guide content or '없음' if not found.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Search query describing what you need help with (e.g., 'YouTube upload', 'VSCode shortcuts')"
                        }
                    },
                    "required": ["query"]
                }),
            },
        },
    ]
}

/// Process user message and return action response
pub async fn process_message(user_message: &str, include_screen: bool) -> Result<ActionResponse> {
    let config = load_config()?;

    if config.api.api_key.is_empty() {
        return Err(anyhow::anyhow!("API key not configured"));
    }

    // Build system prompt with guide index
    let system_prompt = build_system_prompt();

    // Build messages
    let mut messages = vec![Message {
        role: "system".to_string(),
        content: MessageContent::Text(system_prompt),
    }];

    // Add user message with optional screen capture
    if include_screen && config.api.supports_vision {
        #[cfg(windows)]
        {
            let screenshot = crate::screen::capture::capture_screen_base64()?;
            let ui_tree = crate::screen::ui_automation::get_active_window_tree(2)
                .map(|t| serde_json::to_string_pretty(&t).unwrap_or_default())
                .unwrap_or_default();

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
            });
        }
        #[cfg(not(windows))]
        {
            messages.push(Message {
                role: "user".to_string(),
                content: MessageContent::Text(user_message.to_string()),
            });
        }
    } else {
        messages.push(Message {
            role: "user".to_string(),
            content: MessageContent::Text(user_message.to_string()),
        });
    }

    // Get tools
    let tools = get_tools();

    // Call API
    let response = chat_completion(&config.api, messages, Some(tools)).await?;

    // Parse response
    let choice = response
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No response from API"))?;

    // Check for tool calls
    if let Some(tool_calls) = &choice.message.tool_calls {
        if let Some(tool_call) = tool_calls.first() {
            let params: serde_json::Value =
                serde_json::from_str(&tool_call.function.arguments).unwrap_or(json!({}));

            return Ok(ActionResponse {
                thought: choice
                    .message
                    .content
                    .clone()
                    .unwrap_or_else(|| "Executing action...".to_string()),
                action: tool_call.function.name.clone(),
                params,
            });
        }
    }

    // No tool call, return text response
    Ok(ActionResponse {
        thought: choice
            .message
            .content
            .clone()
            .unwrap_or_else(|| "I'm not sure what to do.".to_string()),
        action: "none".to_string(),
        params: json!({}),
    })
}
