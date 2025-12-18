// Commands module - Tauri command handlers

pub mod config;
pub mod guides;
pub mod input;
pub mod llm;
pub mod overlay;
pub mod screen;

/// Simple greet command for testing
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to AutoMate.", name)
}
