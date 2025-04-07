use crate::openai_api::{CompletionRequest, Message, Role};

/// Default completion request generator.
pub(super) fn prepare_completion_request(
    model: sauropod_config::ModelConfig,
    context: crate::LlmContext,
) -> anyhow::Result<CompletionRequest> {
    let tools: Vec<crate::openai_api::Tool> = context
        .tools
        .into_iter()
        .map(|tool_definition| crate::openai_api::Tool {
            function: crate::openai_api::Function::from(tool_definition),
            r#type: "function".to_string(),
        })
        .collect();

    Ok(CompletionRequest {
        model: model.model,
        messages: vec![
            Message {
                role: Role::System,
                content: Some(context.system_prompt.trim().to_string()),
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
        tools,
        response_format: context.output_schema.map(|schema| {
            crate::openai_api::ResponseFormat::JsonSchema {
                json_schema: crate::openai_api::ResponseJsonSchema {
                    name: "output".to_string(),
                    schema: schema.clone(),
                    strict: true,
                },
            }
        }),
        ..CompletionRequest::default()
    })
}
