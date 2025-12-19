// AutoMate - AI-powered Windows automation assistant
// Main library module

mod commands;
mod config;
mod guides;
mod input;
mod llm;
mod screen;

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{Manager, WindowEvent};

/// Application entry point
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Single instance plugin (must be first)
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Another instance tried to start - show and focus our window
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        // Plugins
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        // Setup
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }

            // Create tray menu
            let show_item = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            // Create tray icon with menu
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

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
        // Window events - hide instead of close to support tray
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // Only intercept main window close
                if window.label() == "main" {
                    // Hide the window instead of closing
                    let _ = window.hide();
                    // Prevent the window from actually closing
                    api.prevent_close();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
