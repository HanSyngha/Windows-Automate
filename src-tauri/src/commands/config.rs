// Configuration commands

use serde::{Deserialize, Serialize};

/// API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub endpoint: String,
    pub api_key: String,
    pub model: String,
    pub supports_vision: bool,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://api.openai.com/v1".to_string(),
            api_key: String::new(),
            model: "gpt-4o".to_string(),
            supports_vision: true,
            max_tokens: 4096,
            temperature: 0.7,
        }
    }
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api: ApiConfig,
    pub language: String,
    pub theme: String,
    pub global_shortcut: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api: ApiConfig::default(),
            language: "ko".to_string(),
            theme: "dark".to_string(),
            global_shortcut: "Shift+Alt+A".to_string(),
        }
    }
}

/// Get current configuration
#[tauri::command]
pub async fn get_config() -> Result<AppConfig, String> {
    crate::config::storage::load_config()
        .map_err(|e| e.to_string())
}

/// Save configuration
#[tauri::command]
pub async fn save_config(config: AppConfig) -> Result<(), String> {
    crate::config::storage::save_config(&config)
        .map_err(|e| e.to_string())
}

/// Test API connection
#[tauri::command]
pub async fn test_api_connection(config: ApiConfig) -> Result<String, String> {
    crate::llm::client::test_connection(&config)
        .await
        .map_err(|e| e.to_string())
}
