// Keyboard control implementation (Windows)

use anyhow::Result;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

/// Type text with natural delay between keystrokes
pub fn type_text(text: &str, delay_ms: u64) -> Result<()> {
    for c in text.chars() {
        type_char(c)?;
        thread::sleep(Duration::from_millis(delay_ms));
    }
    Ok(())
}

/// Type a single character
fn type_char(c: char) -> Result<()> {
    unsafe {
        let mut inputs = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: c as u16,
                        dwFlags: KEYEVENTF_UNICODE,
                        ..Default::default()
                    },
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VIRTUAL_KEY(0),
                        wScan: c as u16,
                        dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                        ..Default::default()
                    },
                },
            },
        ];

        SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32);
    }
    Ok(())
}

/// Press key combination (e.g., ["ctrl", "c"])
pub fn press_keys(keys: &[String]) -> Result<()> {
    let key_map = build_key_map();

    // Press all keys down
    for key in keys {
        let vk = key_map
            .get(key.to_lowercase().as_str())
            .copied()
            .unwrap_or(VK_NONAME);

        if vk != VK_NONAME {
            key_down(vk)?;
        }
    }

    // Small delay
    thread::sleep(Duration::from_millis(50));

    // Release all keys in reverse order
    for key in keys.iter().rev() {
        let vk = key_map
            .get(key.to_lowercase().as_str())
            .copied()
            .unwrap_or(VK_NONAME);

        if vk != VK_NONAME {
            key_up(vk)?;
        }
    }

    Ok(())
}

/// Press a key down
fn key_down(vk: VIRTUAL_KEY) -> Result<()> {
    unsafe {
        let mut input = [INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    dwFlags: KEYBD_EVENT_FLAGS(0),
                    ..Default::default()
                },
            },
        }];

        SendInput(&mut input, std::mem::size_of::<INPUT>() as i32);
    }
    Ok(())
}

/// Release a key
fn key_up(vk: VIRTUAL_KEY) -> Result<()> {
    unsafe {
        let mut input = [INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    dwFlags: KEYEVENTF_KEYUP,
                    ..Default::default()
                },
            },
        }];

        SendInput(&mut input, std::mem::size_of::<INPUT>() as i32);
    }
    Ok(())
}

/// Build key name to virtual key code mapping
fn build_key_map() -> HashMap<&'static str, VIRTUAL_KEY> {
    let mut map = HashMap::new();

    // Modifiers
    map.insert("ctrl", VK_CONTROL);
    map.insert("control", VK_CONTROL);
    map.insert("alt", VK_MENU);
    map.insert("shift", VK_SHIFT);
    map.insert("win", VK_LWIN);
    map.insert("windows", VK_LWIN);

    // Special keys
    map.insert("enter", VK_RETURN);
    map.insert("return", VK_RETURN);
    map.insert("tab", VK_TAB);
    map.insert("escape", VK_ESCAPE);
    map.insert("esc", VK_ESCAPE);
    map.insert("backspace", VK_BACK);
    map.insert("delete", VK_DELETE);
    map.insert("del", VK_DELETE);
    map.insert("insert", VK_INSERT);
    map.insert("home", VK_HOME);
    map.insert("end", VK_END);
    map.insert("pageup", VK_PRIOR);
    map.insert("pagedown", VK_NEXT);
    map.insert("space", VK_SPACE);

    // Arrow keys
    map.insert("up", VK_UP);
    map.insert("down", VK_DOWN);
    map.insert("left", VK_LEFT);
    map.insert("right", VK_RIGHT);

    // Function keys
    map.insert("f1", VK_F1);
    map.insert("f2", VK_F2);
    map.insert("f3", VK_F3);
    map.insert("f4", VK_F4);
    map.insert("f5", VK_F5);
    map.insert("f6", VK_F6);
    map.insert("f7", VK_F7);
    map.insert("f8", VK_F8);
    map.insert("f9", VK_F9);
    map.insert("f10", VK_F10);
    map.insert("f11", VK_F11);
    map.insert("f12", VK_F12);

    // Letters (A-Z)
    for c in 'a'..='z' {
        let key = Box::leak(c.to_string().into_boxed_str());
        map.insert(key, VIRTUAL_KEY(c.to_ascii_uppercase() as u16));
    }

    // Numbers (0-9)
    for c in '0'..='9' {
        let key = Box::leak(c.to_string().into_boxed_str());
        map.insert(key, VIRTUAL_KEY(c as u16));
    }

    map
}
