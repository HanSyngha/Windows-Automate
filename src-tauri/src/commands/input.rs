// Input automation commands (mouse & keyboard)

use serde::{Deserialize, Serialize};

/// Mouse button type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Move mouse to coordinates with smooth animation
#[tauri::command]
pub async fn mouse_move(x: i32, y: i32, smooth: Option<bool>) -> Result<(), String> {
    #[cfg(windows)]
    {
        let use_smooth = smooth.unwrap_or(true);
        if use_smooth {
            crate::input::mouse::smooth_move(x, y, 300).map_err(|e| e.to_string())
        } else {
            crate::input::mouse::instant_move(x, y).map_err(|e| e.to_string())
        }
    }
    #[cfg(not(windows))]
    {
        Err("Mouse control is only supported on Windows".to_string())
    }
}

/// Click at coordinates
#[tauri::command]
pub async fn mouse_click(
    x: i32,
    y: i32,
    button: Option<MouseButton>,
    double: Option<bool>,
) -> Result<(), String> {
    #[cfg(windows)]
    {
        let btn = button.unwrap_or(MouseButton::Left);
        let is_double = double.unwrap_or(false);
        crate::input::mouse::click(x, y, btn, is_double).map_err(|e| e.to_string())
    }
    #[cfg(not(windows))]
    {
        Err("Mouse control is only supported on Windows".to_string())
    }
}

/// Type text
#[tauri::command]
pub async fn keyboard_type(text: &str, delay_ms: Option<u64>) -> Result<(), String> {
    #[cfg(windows)]
    {
        let delay = delay_ms.unwrap_or(30);
        crate::input::keyboard::type_text(text, delay).map_err(|e| e.to_string())
    }
    #[cfg(not(windows))]
    {
        Err("Keyboard control is only supported on Windows".to_string())
    }
}

/// Press key or key combination
#[tauri::command]
pub async fn keyboard_press(keys: Vec<String>) -> Result<(), String> {
    #[cfg(windows)]
    {
        crate::input::keyboard::press_keys(&keys).map_err(|e| e.to_string())
    }
    #[cfg(not(windows))]
    {
        Err("Keyboard control is only supported on Windows".to_string())
    }
}
