// Overlay commands - control visual effects overlay window

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

/// Overlay event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum OverlayEvent {
    #[serde(rename = "cursor_move")]
    CursorMove { x: i32, y: i32 },

    #[serde(rename = "click")]
    Click {
        x: i32,
        y: i32,
        button: String,
    },

    #[serde(rename = "status")]
    Status {
        status: String,
        message: Option<String>,
    },

    #[serde(rename = "control")]
    Control { controlling: bool },
}

/// Show overlay window
#[tauri::command]
pub async fn overlay_show(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("overlay") {
        window.show().map_err(|e| e.to_string())?;

        // Set overlay to be click-through on Windows
        #[cfg(windows)]
        {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{
                GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT,
            };

            if let Ok(hwnd) = window.hwnd() {
                unsafe {
                    let hwnd = HWND(hwnd.0);
                    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                    SetWindowLongW(
                        hwnd,
                        GWL_EXSTYLE,
                        ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32,
                    );
                }
            }
        }
    }
    Ok(())
}

/// Hide overlay window
#[tauri::command]
pub async fn overlay_hide(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("overlay") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Send cursor move event to overlay
#[tauri::command]
pub async fn overlay_cursor_move(app: AppHandle, x: i32, y: i32) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("overlay") {
        let event = serde_json::json!({
            "type": "cursor_move",
            "payload": { "x": x, "y": y }
        });
        window.emit("overlay-event", event).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Send click event to overlay
#[tauri::command]
pub async fn overlay_click(
    app: AppHandle,
    x: i32,
    y: i32,
    button: Option<String>,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("overlay") {
        let event = serde_json::json!({
            "type": "click",
            "payload": {
                "x": x,
                "y": y,
                "button": button.unwrap_or_else(|| "left".to_string())
            }
        });
        window.emit("overlay-event", event).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Send status update to overlay
#[tauri::command]
pub async fn overlay_status(
    app: AppHandle,
    status: String,
    message: Option<String>,
) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("overlay") {
        let event = serde_json::json!({
            "type": "status",
            "payload": {
                "status": status,
                "message": message
            }
        });
        window.emit("overlay-event", event).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Set AI control state
#[tauri::command]
pub async fn overlay_set_control(app: AppHandle, controlling: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("overlay") {
        let event = serde_json::json!({
            "type": "control",
            "payload": { "controlling": controlling }
        });
        window.emit("overlay-event", event).map_err(|e| e.to_string())?;
    }

    // Also show/hide overlay based on control state
    if controlling {
        overlay_show(app).await?;
    } else {
        overlay_hide(app).await?;
    }

    Ok(())
}
