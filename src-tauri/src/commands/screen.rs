// Screen capture and UI automation commands

use serde::{Deserialize, Serialize};

/// UI element information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIElement {
    pub name: String,
    pub class_name: String,
    pub control_type: String,
    pub bounding_rect: BoundingRect,
    pub is_enabled: bool,
    pub is_focused: bool,
    pub children: Vec<UIElement>,
}

/// Bounding rectangle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// Capture screen as base64 encoded PNG
#[tauri::command]
pub async fn capture_screen() -> Result<String, String> {
    #[cfg(windows)]
    {
        crate::screen::capture::capture_screen_base64()
            .map_err(|e| e.to_string())
    }
    #[cfg(not(windows))]
    {
        Err("Screen capture is only supported on Windows".to_string())
    }
}

/// Get UI element tree of the active window
#[tauri::command]
pub async fn get_ui_tree(max_depth: Option<usize>) -> Result<UIElement, String> {
    #[cfg(windows)]
    {
        let depth = max_depth.unwrap_or(3);
        crate::screen::ui_automation::get_active_window_tree(depth)
            .map_err(|e| e.to_string())
    }
    #[cfg(not(windows))]
    {
        Err("UI Automation is only supported on Windows".to_string())
    }
}
