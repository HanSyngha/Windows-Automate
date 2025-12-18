// Mouse control implementation (Windows)

use crate::commands::input::MouseButton;
use anyhow::Result;
use std::thread;
use std::time::Duration;
use windows::Win32::Foundation::*;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

/// Move mouse instantly to coordinates
pub fn instant_move(x: i32, y: i32) -> Result<()> {
    unsafe {
        SetCursorPos(x, y)?;
    }
    Ok(())
}

/// Move mouse smoothly using bezier curve
pub fn smooth_move(target_x: i32, target_y: i32, duration_ms: u64) -> Result<()> {
    unsafe {
        let mut current_pos = POINT::default();
        GetCursorPos(&mut current_pos)?;

        let start_x = current_pos.x as f64;
        let start_y = current_pos.y as f64;
        let end_x = target_x as f64;
        let end_y = target_y as f64;

        // Control points for bezier curve (adds natural curve to movement)
        let cp1_x = start_x + (end_x - start_x) * 0.3;
        let cp1_y = start_y + (end_y - start_y) * 0.1;
        let cp2_x = start_x + (end_x - start_x) * 0.7;
        let cp2_y = end_y - (end_y - start_y) * 0.1;

        let steps = (duration_ms / 16).max(10) as usize; // ~60fps
        let step_duration = Duration::from_millis(duration_ms / steps as u64);

        for i in 0..=steps {
            let t = i as f64 / steps as f64;

            // Cubic bezier formula with ease-in-out
            let eased_t = ease_in_out(t);
            let (x, y) = cubic_bezier(
                start_x, start_y, cp1_x, cp1_y, cp2_x, cp2_y, end_x, end_y, eased_t,
            );

            SetCursorPos(x as i32, y as i32)?;
            thread::sleep(step_duration);
        }

        // Ensure we end exactly at target
        SetCursorPos(target_x, target_y)?;
    }
    Ok(())
}

/// Ease in-out function for smooth acceleration/deceleration
fn ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Cubic bezier interpolation
fn cubic_bezier(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    t: f64,
) -> (f64, f64) {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    let x = uuu * x0 + 3.0 * uu * t * x1 + 3.0 * u * tt * x2 + ttt * x3;
    let y = uuu * y0 + 3.0 * uu * t * y1 + 3.0 * u * tt * y2 + ttt * y3;

    (x, y)
}

/// Click at coordinates
pub fn click(x: i32, y: i32, button: MouseButton, double: bool) -> Result<()> {
    // Move to position first
    smooth_move(x, y, 200)?;

    // Small delay after move
    thread::sleep(Duration::from_millis(50));

    let (down_flag, up_flag) = match button {
        MouseButton::Left => (MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP),
        MouseButton::Right => (MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP),
        MouseButton::Middle => (MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP),
    };

    unsafe {
        let clicks = if double { 2 } else { 1 };

        for _ in 0..clicks {
            let mut inputs = [
                INPUT {
                    r#type: INPUT_MOUSE,
                    Anonymous: INPUT_0 {
                        mi: MOUSEINPUT {
                            dwFlags: down_flag,
                            ..Default::default()
                        },
                    },
                },
                INPUT {
                    r#type: INPUT_MOUSE,
                    Anonymous: INPUT_0 {
                        mi: MOUSEINPUT {
                            dwFlags: up_flag,
                            ..Default::default()
                        },
                    },
                },
            ];

            SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32);

            if double {
                thread::sleep(Duration::from_millis(50));
            }
        }
    }

    Ok(())
}

/// Get current cursor position
pub fn get_position() -> Result<(i32, i32)> {
    unsafe {
        let mut point = POINT::default();
        GetCursorPos(&mut point)?;
        Ok((point.x, point.y))
    }
}
