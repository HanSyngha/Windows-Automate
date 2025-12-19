// LLM integration module
//
// Structure:
// - types: Common types (Message, Tool, Response)
// - client: API communication
// - prompts: System prompts and builders
// - tools: Primitive tools (mouse, keyboard, screen)
// - agents: Sub-agent tools (guide_search)
// - runner: Main agent loop and execution

pub mod types;
pub mod client;
pub mod prompts;
pub mod tools;
pub mod agents;
pub mod runner;

// Re-export commonly used items
pub use runner::process_message;
