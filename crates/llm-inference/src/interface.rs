//! Backend-agnostic interfaces.

/// The context used for an LLM invocation.
pub struct LlmContext {
    /// The tools available to the model.
    pub tools: Vec<sauropod_schemas::ToolDefinition>,
    /// The system prompt for the model.
    pub system_prompt: String,
}
