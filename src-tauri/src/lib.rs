// AutoMate - AI-powered Windows automation assistant
// Main library module

mod commands;
mod config;
mod guides;
mod input;
mod llm;
mod screen;

use tauri::Manager;

/// Application entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Plugins
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_global_shortcut::init())
        // Setup
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        // Commands
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::screen::capture_screen,
            commands::screen::get_ui_tree,
            commands::input::mouse_move,
            commands::input::mouse_click,
            commands::input::keyboard_type,
            commands::input::keyboard_press,
            commands::config::get_config,
            commands::config::save_config,
            commands::config::test_api_connection,
            commands::llm::send_message,
            commands::guides::guide_list,
            commands::guides::guide_preview,
            commands::guides::guide_read,
            commands::guides::guide_index,
            commands::guides::guide_search,
            commands::guides::guide_create,
            commands::overlay::overlay_show,
            commands::overlay::overlay_hide,
            commands::overlay::overlay_cursor_move,
            commands::overlay::overlay_click,
            commands::overlay::overlay_status,
            commands::overlay::overlay_set_control,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
