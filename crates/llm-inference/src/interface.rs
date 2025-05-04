//! Backend-agnostic interfaces.

/// The context used for an LLM invocation.
pub struct LlmContext<'a> {
    /// The user prompt.
    pub user_prompt: Vec<crate::openai_api::ContentItem>,
    /// The tools available to the model.
    pub tools: Vec<sauropod_schemas::ToolDefinition>,
    /// The system prompt for the model.
    pub system_prompt: String,
    /// The schema for structured output.
    pub output_schema: Option<&'a serde_json::Value>,
}
