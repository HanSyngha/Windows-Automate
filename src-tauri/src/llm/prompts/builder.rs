// Dynamic prompt builder

use crate::guides::storage::get_guide_index;
use super::system::MAIN_AGENT_PROMPT;

/// Build the main agent system prompt with dynamic content
pub fn build_main_agent_prompt() -> String {
    let mut prompt = MAIN_AGENT_PROMPT.to_string();

    // Add available tools description
    prompt.push_str("\n\nAvailable tools:\n");
    prompt.push_str("- mouse_move(x, y): Move mouse to coordinates\n");
    prompt.push_str("- mouse_click(x, y, button): Click at coordinates (button: \"left\", \"right\", \"middle\")\n");
    prompt.push_str("- mouse_double_click(x, y): Double click at coordinates\n");
    prompt.push_str("- keyboard_type(text): Type text\n");
    prompt.push_str("- keyboard_press(keys): Press key combination (e.g., [\"ctrl\", \"c\"])\n");
    prompt.push_str("- scroll(direction, amount): Scroll (direction: \"up\", \"down\", \"left\", \"right\")\n");
    prompt.push_str("- wait(ms): Wait for milliseconds\n");
    prompt.push_str("- get_screen_update(): Request updated screen information\n");
    prompt.push_str("- guide_search(query): Search for relevant guides to help with the current task\n");

    // Add guide index if available
    if let Ok(index) = get_guide_index() {
        if !index.is_empty() {
            prompt.push_str("\n## Available Guides\n");
            prompt.push_str("The following guides are available. Use guide_search to retrieve relevant content:\n");
            for entry in &index {
                prompt.push_str(&format!("- {}: {}\n", entry.path, entry.title));
            }
        }
    }

    prompt
}

/// Prompt builder with customization options
#[allow(dead_code)]
pub struct PromptBuilder {
    base_prompt: String,
    tools_description: Option<String>,
    guides_section: bool,
    custom_sections: Vec<String>,
}

impl PromptBuilder {
    pub fn new(base: &str) -> Self {
        Self {
            base_prompt: base.to_string(),
            tools_description: None,
            guides_section: false,
            custom_sections: Vec::new(),
        }
    }

    pub fn with_tools(mut self, description: &str) -> Self {
        self.tools_description = Some(description.to_string());
        self
    }

    pub fn with_guides(mut self) -> Self {
        self.guides_section = true;
        self
    }

    pub fn add_section(mut self, section: &str) -> Self {
        self.custom_sections.push(section.to_string());
        self
    }

    pub fn build(self) -> String {
        let mut prompt = self.base_prompt;

        if let Some(tools) = self.tools_description {
            prompt.push_str("\n\n");
            prompt.push_str(&tools);
        }

        if self.guides_section {
            if let Ok(index) = get_guide_index() {
                if !index.is_empty() {
                    prompt.push_str("\n\n## Available Guides\n");
                    for entry in &index {
                        prompt.push_str(&format!("- {}: {}\n", entry.path, entry.title));
                    }
                }
            }
        }

        for section in self.custom_sections {
            prompt.push_str("\n\n");
            prompt.push_str(&section);
        }

        prompt
    }
}
