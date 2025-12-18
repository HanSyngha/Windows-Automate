// Guide commands - Tauri command handlers for guide system

use crate::guides::storage::{
    get_guide_index, list_guides, preview_guide, read_guide, save_guide, GuideEntry,
    GuideIndexEntry,
};
use crate::guides::search::search_guide;
use serde::{Deserialize, Serialize};

/// Guide creation request
#[derive(Debug, Deserialize)]
pub struct CreateGuideRequest {
    pub user_input: String,
}

/// Guide creation response
#[derive(Debug, Serialize)]
pub struct CreateGuideResponse {
    pub path: String,
    pub title: String,
}

/// List guides in a directory
#[tauri::command]
pub async fn guide_list(path: Option<String>) -> Result<Vec<GuideEntry>, String> {
    list_guides(path.as_deref()).map_err(|e| e.to_string())
}

/// Preview a guide (first 10 lines)
#[tauri::command]
pub async fn guide_preview(path: String) -> Result<String, String> {
    preview_guide(&path).map_err(|e| e.to_string())
}

/// Read full guide content
#[tauri::command]
pub async fn guide_read(path: String) -> Result<String, String> {
    read_guide(&path).map_err(|e| e.to_string())
}

/// Get guide index (depth-1 listing for system prompt)
#[tauri::command]
pub async fn guide_index() -> Result<Vec<GuideIndexEntry>, String> {
    get_guide_index().map_err(|e| e.to_string())
}

/// Search guides using Sub-LLM agent
#[tauri::command]
pub async fn guide_search(query: String) -> Result<String, String> {
    search_guide(&query).await.map_err(|e| e.to_string())
}

/// Create a new guide from user input
/// This uses LLM to convert natural language to structured markdown
#[tauri::command]
pub async fn guide_create(request: CreateGuideRequest) -> Result<CreateGuideResponse, String> {
    use crate::config::storage::load_config;
    use crate::llm::client::{chat_completion, Message, MessageContent};

    let config = load_config().map_err(|e| e.to_string())?;

    if config.api.api_key.is_empty() {
        return Err("API key not configured".to_string());
    }

    // System prompt for guide creation
    let system_prompt = r#"You are a guide creation assistant. Convert user input into a structured markdown guide.

Output format (JSON):
{
  "folder": "websites|applications|workflows",
  "filename": "kebab-case-name.md",
  "content": "markdown content with frontmatter"
}

Markdown template:
---
title: Guide Title
domain: example.com (if applicable)
tags: [tag1, tag2]
created: YYYY-MM-DD
---

# Guide Title

## Prerequisites
- List prerequisites

## Step-by-step Guide

### 1. First Step
- Details

### 2. Second Step
- Details

## Notes
- Additional notes

Rules:
- Choose appropriate folder based on content type
- Use descriptive, kebab-case filename
- Include frontmatter with title, tags, and date
- Write clear, actionable steps
- Today's date is: "#.to_string() + &chrono::Local::now().format("%Y-%m-%d").to_string();

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: MessageContent::Text(system_prompt),
        },
        Message {
            role: "user".to_string(),
            content: MessageContent::Text(request.user_input),
        },
    ];

    let response = chat_completion(&config.api, messages, None)
        .await
        .map_err(|e| e.to_string())?;

    let choice = response
        .choices
        .first()
        .ok_or_else(|| "No response from API".to_string())?;

    let response_text = choice
        .message
        .content
        .clone()
        .ok_or_else(|| "Empty response".to_string())?;

    // Parse JSON response
    let json_start = response_text.find('{').ok_or("Invalid response format")?;
    let json_end = response_text.rfind('}').ok_or("Invalid response format")?;
    let json_str = &response_text[json_start..=json_end];

    let parsed: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse response: {}", e))?;

    let folder = parsed
        .get("folder")
        .and_then(|v| v.as_str())
        .ok_or("Missing folder")?;
    let filename = parsed
        .get("filename")
        .and_then(|v| v.as_str())
        .ok_or("Missing filename")?;
    let content = parsed
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or("Missing content")?;

    let path = format!("{}/{}", folder, filename);

    // Save the guide
    save_guide(&path, content).map_err(|e| e.to_string())?;

    // Extract title from content
    let title = content
        .lines()
        .find(|l| l.starts_with("title:"))
        .map(|l| l[6..].trim().trim_matches('"').to_string())
        .or_else(|| {
            content
                .lines()
                .find(|l| l.starts_with("# "))
                .map(|l| l[2..].trim().to_string())
        })
        .unwrap_or_else(|| filename.to_string());

    Ok(CreateGuideResponse { path, title })
}
