use crate::openai_api::{CompletionRequest, Message, Role};

/// See <https://ai.google.dev/gemma/docs/capabilities/function-calling>.
const FUNCTION_CALL_PREAMBLE: &str = r#"

You have access to functions. If you decide to invoke any of the function(s),
 you MUST put it in the format of
{"call_function": function name, "parameters": dictionary of argument name and its value}

You SHOULD NOT include any other text in the response if you call a function
"#;

/// To create a response format that also allows for function calling, we need to
/// create a oneOf JSON schema allows both tool calling and the user's requested output format.
pub fn create_response_format(schema: &serde_json::Value) -> serde_json::Value {
    let mut schema = schema.clone();
    schema["description"] = "Output format the user requested".into();

    serde_json::json!({
        "oneOf": [
            {
                "type": "object",
                "description": "function call",
                "properties": {
                    "call_function": {
                        "type": "string",
                        "description": "function name"
                    },
                    "parameters": {
                        "type": "object",
                        "description": "dictionary of argument name and its value",
                    }
                },
                "required": ["call_function", "parameters"]
            },
            schema
        ]
    })
}

/// Completion request generator for Gemma 3.
pub(super) fn prepare_completion_request(
    model: sauropod_config::ModelConfig,
    context: crate::LlmContext,
) -> anyhow::Result<CompletionRequest> {
    let mut system_prompt = context.system_prompt.trim().to_string();
    if !context.tools.is_empty() {
        system_prompt.push_str(FUNCTION_CALL_PREAMBLE);
    }

    if let Some(output_format) = context.output_schema {
        system_prompt.push_str(&format!(
            "\n\nGive your response in the format: {}",
            serde_json::to_string_pretty(&output_format)?
        ));
    }

    let tools: Vec<_> = context
        .tools
        .into_iter()
        .map(|tool_definition| {
            serde_json::json!({
                "name": tool_definition.id,
                "description": tool_definition.description,
                "parameters": tool_definition.input_schema
            })
        })
        .collect();
    system_prompt.push_str(&serde_json::to_string_pretty(&tools)?);

    Ok(CompletionRequest {
        model: model.model,
        messages: vec![
            Message {
                role: Role::System,
                content: Some(system_prompt),
                tool_calls: vec![],
                tool_call_id: None,
            },
            Message {
                role: Role::User,
                content: Some(context.user_prompt),
                tool_calls: vec![],
                tool_call_id: None,
            },
        ],
        response_format: context.output_schema.map(|schema| {
            crate::openai_api::ResponseFormat::JsonSchema {
                json_schema: crate::openai_api::ResponseJsonSchema {
                    name: "output".to_string(),
                    schema: create_response_format(schema),
                    strict: true,
                },
            }
        }),
        ..CompletionRequest::default()
    })
}
