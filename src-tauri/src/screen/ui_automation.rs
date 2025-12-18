// UI Automation implementation (Windows)

use crate::commands::screen::{BoundingRect, UIElement};
use anyhow::Result;
use uiautomation::UIAutomation;

/// Get UI element tree of the active window
pub fn get_active_window_tree(max_depth: usize) -> Result<UIElement> {
    let automation = UIAutomation::new()?;
    let focused = automation.get_focused_element()?;

    // Try to get the parent window
    let walker = automation.get_control_view_walker()?;
    let root = find_window_root(&walker, &focused)?;

    build_element_tree(&walker, &root, 0, max_depth)
}

/// Find the root window element from a focused element
fn find_window_root(
    walker: &uiautomation::UITreeWalker,
    element: &uiautomation::UIElement,
) -> Result<uiautomation::UIElement> {
    let mut current = element.clone();
    let automation = UIAutomation::new()?;
    let desktop = automation.get_root_element()?;

    loop {
        match walker.get_parent(&current) {
            Ok(parent) => {
                // Check if parent is desktop
                if parent.get_runtime_id()? == desktop.get_runtime_id()? {
                    return Ok(current);
                }
                current = parent;
            }
            Err(_) => return Ok(current),
        }
    }
}

/// Build UI element tree recursively
fn build_element_tree(
    walker: &uiautomation::UITreeWalker,
    element: &uiautomation::UIElement,
    depth: usize,
    max_depth: usize,
) -> Result<UIElement> {
    let name = element.get_name().unwrap_or_default();
    let class_name = element.get_classname().unwrap_or_default();
    let control_type = format!("{:?}", element.get_control_type()?);
    let is_enabled = element.is_enabled().unwrap_or(false);

    let rect = element.get_bounding_rectangle()?;
    let bounding_rect = BoundingRect {
        x: rect.get_left(),
        y: rect.get_top(),
        width: rect.get_width(),
        height: rect.get_height(),
    };

    let is_focused = element.has_keyboard_focus().unwrap_or(false);

    let mut children = Vec::new();

    if depth < max_depth {
        if let Ok(first_child) = walker.get_first_child(element) {
            children.push(build_element_tree(walker, &first_child, depth + 1, max_depth)?);

            let mut sibling = first_child;
            while let Ok(next) = walker.get_next_sibling(&sibling) {
                children.push(build_element_tree(walker, &next, depth + 1, max_depth)?);
                sibling = next;
            }
        }
    }

    Ok(UIElement {
        name,
        class_name,
        control_type,
        bounding_rect,
        is_enabled,
        is_focused,
        children,
    })
}
