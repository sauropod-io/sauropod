use std::sync::Arc;

use sauropod_llm_inference::LlmContext;
use sauropod_llm_inference::openai_api::{FinishReason, Message, Role};
use tracing::Instrument;

use crate::Task;

/// Task to invoke an LLM.
pub(crate) struct InvokeLlmTask {
    /// The model strength to use for the task.
    model_strength: sauropod_schemas::task::ModelStrength,
    /// The template environment for the task.
    template_env: minijinja::Environment<'static>,
    /// The input JSON schema for the task.
    input_schema: serde_json::Value,
    /// The output JSON schema for the task.
    output_schema: serde_json::Value,
}

/// Parse a JSON value from a string.
///
/// The string may either be a JSON string like `{"foo": "bar"}` or a Markdown-style JSON code block.
pub fn parse_json_text<'a, T>(json: &'a str) -> serde_json::Result<T>
where
    T: serde::Deserialize<'a>,
{
    if json.starts_with("```json") {
        serde_json::from_str::<'a, T>(
            json.trim_start_matches("```json")
                .trim_end_matches("```")
                .trim(),
        )
    } else {
        serde_json::from_str::<'a, T>(json)
    }
}

static TEMPLATE_NAME: &str = "template";

impl InvokeLlmTask {
    pub fn new(invoke_llm: sauropod_schemas::task::InvokeLLM) -> anyhow::Result<Self> {
        // Check the template before loading the model to avoid loading the model if the template is invalid
        let mut template_env = minijinja::Environment::new();
        template_env.add_template_owned(TEMPLATE_NAME, invoke_llm.template.0)?;

        let input_schema = sauropod_prompt_templates::template_to_inputs(
            template_env.get_template(TEMPLATE_NAME)?,
        )?;

        Ok(Self {
            model_strength: invoke_llm.model_strength,
            template_env,
            input_schema,
            // For now, just return a string.
            // In the future we will support structured outputs.
            output_schema: serde_json::json!({
                "type": "string"
            }),
        })
    }
}

#[async_trait::async_trait]
impl Task for InvokeLlmTask {
    async fn execute(
        &self,
        input: serde_json::Value,
        context: Arc<sauropod_task_context::TaskContext>,
    ) -> anyhow::Result<serde_json::Value> {
        tracing::debug!("Executing invoke LLM task with input: {:#?}", input);
        if let Err(validation_error) = jsonschema::validate(&self.input_schema, &input) {
            tracing::error!("Error running task: {}", &validation_error);
            anyhow::bail!("Error running task: {}", validation_error)
        }

        let template = self.template_env.get_template(TEMPLATE_NAME)?;
        let expanded_template = template.render(serde_json::json!(input))?;
        let model = context.get_model(self.model_strength)?;
        let llm_context = LlmContext {
            tools: context.tools.values().map(|x| x.get_definition()).collect(),
            system_prompt: context.system_prompt.clone(),
        };
        let mut request = sauropod_llm_inference::prepare_completion_request(model, llm_context)?;
        request.messages.push(Message {
            role: Role::User,
            content: Some(expanded_template),
            tool_calls: vec![],
            tool_call_id: None,
        });

        loop {
            let result = context
                .llm_engine
                .invoke(&request)
                .instrument(tracing::info_span!("executing invoke LLM task"))
                .await?;

            if result.choices.len() > 1 {
                tracing::warn!("Received multiple choices from the LLM, using the first one");
            }
            let Some(choice) = result.choices.into_iter().next() else {
                return Err(anyhow::anyhow!("LLM API did not return any choices"));
            };
            request.messages.push(choice.message.clone());
            tracing::debug!("LLM response: {:#?}", &request);

            match choice.finish_reason {
                FinishReason::Stop => {
                    // A raw JSON response may be a function call
                    if let Some(mut content) = choice.message.content.as_ref().and_then(|x| {
                        parse_json_text::<serde_json::Map<String, serde_json::Value>>(x).ok()
                    }) {
                        let Some(parameters) = content.remove("parameters") else {
                            return Ok(serde_json::json!(choice.message.content));
                        };
                        let Some(function_call) =
                            content.get("call_function").and_then(|x| x.as_str())
                        else {
                            return Ok(serde_json::json!(choice.message.content));
                        };
                        let Some(tool) = context.tools.get(function_call).cloned() else {
                            anyhow::bail!(
                                "The LLM tried to call {} which isn't an available tool",
                                &function_call
                            );
                        };

                        let content = tool.run(parameters).await?;
                        request.messages.push(Message {
                            role: Role::User, // Gemma 3 uses the user role for function call responses
                            tool_call_id: None,
                            content: Some(content),
                            tool_calls: vec![],
                        })
                    } else {
                        return Ok(serde_json::json!(choice.message.content));
                    }
                }
                FinishReason::Length => {
                    tracing::error!(
                        "The LLM stopped because it reached the maximum number of tokens"
                    );
                    return Ok(serde_json::json!(choice.message.content));
                }
                FinishReason::ToolCalls => {
                    let tool_calls = choice.message.tool_calls.clone();
                    for tool_call in &tool_calls {
                        let Some(function_call) = &tool_call.function else {
                            anyhow::bail!(
                                "The LLM tried to call a tool but didn't provide a function: {:?}",
                                tool_call
                            );
                        };

                        let Some(tool) = context.tools.get(&function_call.name).cloned() else {
                            anyhow::bail!(
                                "The LLM tried to call {} which isn't an available tool",
                                &function_call.name
                            );
                        };

                        let arguments = serde_json::from_str(&function_call.arguments)
                            .map_err(|e| {
                                anyhow::anyhow!(
                                    "The LLM tried to call {} but the arguments were invalid JSON: {e}",
                                    &function_call.name
                                )
                            })?;
                        let content = tool.run(arguments).await?;
                        request.messages.push(Message {
                            role: Role::Tool,
                            tool_call_id: Some(tool_call.id.clone()),
                            content: Some(content),
                            tool_calls: vec![],
                        });
                    }
                }
                FinishReason::ContentFilter => {
                    anyhow::bail!("The LLM stopped because of a content filter");
                }
            }
        }
    }

    fn input_schema(&self) -> &serde_json::Value {
        &self.input_schema
    }

    fn output_schema(&self) -> &serde_json::Value {
        &self.output_schema
    }
}
