use crate::openai_api::{CompletionRequest, Message, Role};

/// See <https://ai.google.dev/gemma/docs/capabilities/function-calling>.
const FUNCTION_CALL_PREAMBLE: &str = r#"

You have access to functions. If you decide to invoke any of the function(s),
 you MUST put it in the format of
{"call_function": function name, "parameters": dictionary of argument name and its value}

You SHOULD NOT include any other text in the response if you call a function
"#;

/// Completion request generator for Gemma 3.
pub(super) fn prepare_completion_request(
    model: sauropod_config::ModelConfig,
    context: crate::LlmContext,
) -> anyhow::Result<CompletionRequest> {
    let mut system_prompt = context.system_prompt.trim().to_string();
    system_prompt.push_str(FUNCTION_CALL_PREAMBLE);
    let tools: Vec<_> = context
        .tools
        .into_iter()
        .map(|tool_definition| {
            serde_json::json!({
                "name": tool_definition.name,
                "description": tool_definition.description,
                "parameters": tool_definition.input_schema
            })
        })
        .collect();
    system_prompt.push_str(&serde_json::to_string_pretty(&tools)?);

    Ok(CompletionRequest {
        model: model.model,
        messages: vec![Message {
            role: Role::System,
            content: Some(system_prompt),
            tool_calls: vec![],
            tool_call_id: None,
        }],
        ..CompletionRequest::default()
    })
}
