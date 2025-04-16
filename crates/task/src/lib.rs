//! Sauropod workflow execution.

use std::collections::HashSet;
use std::sync::Arc;

use sauropod_llm_inference::LlmContext;
use sauropod_llm_inference::openai_api::{FinishReason, Message, Role};
use sauropod_prompt_templates::Template;
use tracing::Instrument;

pub const TASK_TOOL_PREFIX: &str = "task:";

/// Task to invoke an LLM.
pub struct Task {
    /// The underlying schema representation of the task.
    schema_representation: sauropod_schemas::task::Task,
    /// The output JSON schema for the task.
    output_schema: serde_json::Value,
    /// The input JSON schema for the task.
    input_schema: serde_json::Value,
    /// Whether to use structured output.
    use_structured_output: bool,
    /// The tools available to the task.
    tools: HashSet<String>,
}

/// Parse a JSON value from a string.
///
/// The string may either be a JSON string like `{"foo": "bar"}` or a Markdown-style JSON code block.
///
/// # Examples
///
/// Parsing a plain JSON string:
///
/// ```
/// use sauropod_task::parse_json_text;
///
/// let json = r#"{"name": "example", "value": 42}"#;
/// let result: serde_json::Value = parse_json_text(json).unwrap();
/// assert_eq!(result["name"], "example");
/// assert_eq!(result["value"], 42);
/// ```
///
/// Parsing a Markdown-style JSON code block:
///
/// ```
/// use sauropod_task::parse_json_text;
///
/// let json = r#"```json
/// {
///   "name": "example",
///   "value": 42
/// }
/// ```"#;
/// let result: serde_json::Value = parse_json_text(json).unwrap();
/// assert_eq!(result["name"], "example");
/// assert_eq!(result["value"], 42);
/// ```
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

impl Task {
    pub fn new(task: sauropod_schemas::task::Task) -> anyhow::Result<Self> {
        let use_structured_output = task.output_schema.is_some();
        let output_schema: serde_json::Value = serde_json::json!(
            task.output_schema
                .clone()
                .unwrap_or_else(crate::default_task_output_schema)
        );

        let input_schema = serde_json::json!(task.input_schema);
        let input_schema_interface = sauropod_json_schema::JsonSchemaInterface::new(&input_schema)?;
        if !input_schema_interface.is_object() {
            anyhow::bail!(
                "The input schema for tasks must be an object, received: {input_schema:?}"
            );
        }

        let input_schema_properties = input_schema_interface.properties_map()?;
        for variable in Template::new(&task.template.0).variables() {
            if !input_schema_properties.contains_key(variable) {
                anyhow::bail!("The input schema is missing the variable {}", variable);
            }
        }

        Ok(Self {
            schema_representation: task.clone(),
            output_schema,
            input_schema,
            use_structured_output,
            tools: task.available_tool_ids.into_iter().collect(),
        })
    }

    /// Create the response value.
    fn make_response(&self, output: String) -> anyhow::Result<serde_json::Value> {
        if !self.use_structured_output {
            return Ok(serde_json::json!({
                "output": output,
            }));
        }

        let Ok(object) = parse_json_text::<serde_json::Value>(&output) else {
            tracing::error!("The LLM output was not valid JSON: {output}");
            anyhow::bail!("The LLM output was not valid JSON: {output}");
        };

        // Validate the output against the schema
        if let Err(validation_error) = jsonschema::validate(&self.output_schema, &object) {
            tracing::error!(
                "The LLM output did not conform to the schema: {}\nThe output was:\n{output}",
                &validation_error
            );
            anyhow::bail!(
                "The LLM output did not conform to the schema: {}\nThe output was:\n{output}",
                validation_error
            )
        }
        Ok(object)
    }

    /// Execute the task.
    pub async fn execute(
        &self,
        input: serde_json::Value,
        context: Arc<sauropod_task_context::TaskContext>,
    ) -> anyhow::Result<serde_json::Value> {
        tracing::debug!("Executing invoke LLM task with input: {:#?}", input);
        if let Err(validation_error) = jsonschema::validate(&self.input_schema, &input) {
            tracing::error!("Error running task: {}", &validation_error);
            anyhow::bail!("Error running task: {}", validation_error)
        }
        let input = input
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("The input to the task was not a JSON object"))?;

        let template =
            sauropod_prompt_templates::Template::new(&self.schema_representation.template.0);
        let model = context.get_model();
        let mut tool_definitions: Vec<_> = context
            .tools
            .values()
            .flat_map(|x| {
                let definition = x.get_definition();
                if self.tools.contains(&definition.id) {
                    Some(definition)
                } else {
                    None
                }
            })
            .collect();

        // Add allowed sub-tasks as tools to the list of tools
        let mut tools = context.tools.clone();
        for tool_id in &self.tools {
            if let Some(suffix) = tool_id.strip_prefix(TASK_TOOL_PREFIX) {
                if let Ok(task_id) = suffix.parse::<i64>() {
                    let task = context
                        .get_task(task_id)
                        .await?
                        .ok_or_else(|| anyhow::anyhow!("Task with ID {task_id} not found"))?;
                    let tool_definition = sauropod_schemas::ToolDefinition {
                        id: tool_id.clone(),
                        name: task.name.clone(),
                        description: "A task".to_string(),
                        input_schema: serde_json::json!(task.input_schema),
                        provider: "Task".to_string(),
                    };
                    tools.insert(
                        tool_id.to_string(),
                        Arc::new(TaskAsTool {
                            tool_id: tool_id.to_string(),
                            task_schema: task.clone(),
                            definition: tool_definition.clone(),
                        }),
                    );
                    tool_definitions.push(tool_definition);
                } else {
                    tracing::warn!("The task tool ID {tool_id} is not a valid task ID");
                }
            }
        }

        let llm_context = LlmContext {
            user_prompt: template.expand(input)?,
            tools: tool_definitions,
            system_prompt: context.system_prompt.clone(),
            output_schema: if self.use_structured_output {
                Some(&self.output_schema)
            } else {
                None
            },
        };

        let mut request =
            sauropod_llm_inference::prepare_completion_request(model.clone(), llm_context)?;
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
                            return self.make_response(choice.message.content.unwrap_or_default());
                        };
                        let Some(function_call) =
                            content.get("call_function").and_then(|x| x.as_str())
                        else {
                            return self.make_response(choice.message.content.unwrap_or_default());
                        };
                        let Some(tool) = tools.get(function_call).cloned() else {
                            anyhow::bail!(
                                "The LLM tried to call {} which isn't an available tool",
                                &function_call
                            );
                        };
                        if !self.tools.contains(function_call) {
                            anyhow::bail!(
                                "The LLM tried to call {} which it isn't allowed to use",
                                &function_call
                            );
                        }

                        let content: String = tool.run(parameters, context.clone()).await?;
                        request.messages.push(Message {
                            role: Role::User, // Gemma 3 uses the user role for function call responses
                            tool_call_id: None,
                            content: Some(content),
                            tool_calls: vec![],
                        })
                    } else {
                        return self.make_response(choice.message.content.unwrap_or_default());
                    }
                }
                FinishReason::Length => {
                    tracing::error!(
                        "The LLM stopped because it reached the maximum number of tokens"
                    );
                    return self.make_response(choice.message.content.unwrap_or_default());
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

                        let Some(tool) = tools.get(&function_call.name).cloned() else {
                            anyhow::bail!(
                                "The LLM tried to call {} which isn't an available tool",
                                &function_call.name
                            );
                        };

                        if !self.tools.contains(&function_call.name) {
                            anyhow::bail!(
                                "The LLM tried to call {} which it isn't allowed to use",
                                &function_call.name
                            );
                        }

                        let arguments = serde_json::from_str(&function_call.arguments)
                            .map_err(|e| {
                                anyhow::anyhow!(
                                    "The LLM tried to call {} but the arguments were invalid JSON: {e}",
                                    &function_call.name
                                )
                            })?;
                        let content = tool.run(arguments, context.clone()).await?;
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

    /// The input schema for the task.
    pub fn input_schema(&self) -> &serde_json::Value {
        &self.input_schema
    }

    /// The output schema for the task.
    pub fn output_schema(&self) -> &serde_json::Value {
        &self.output_schema
    }
}

/// A interface to call a task as a tool.
pub struct TaskAsTool {
    /// The ID of the tool.
    tool_id: String,
    /// The task to call.
    task_schema: sauropod_schemas::task::Task,
    /// The tool definition.
    definition: sauropod_schemas::ToolDefinition,
}

impl TaskAsTool {
    async fn run_task(
        self: Arc<Self>,
        input: serde_json::Value,
        task_context: Arc<sauropod_task_context::TaskContext>,
    ) -> anyhow::Result<String> {
        let task = Task::new(self.task_schema.clone())?;
        let result = task.execute(input, task_context).await?;
        Ok(serde_json::to_string(&result)?)
    }
}

impl sauropod_task_context::RunnableTool for TaskAsTool {
    fn get_id(&self) -> &str {
        &self.tool_id
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        task_context: Arc<sauropod_task_context::TaskContext>,
    ) -> std::pin::Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send>> {
        Box::pin(async move { self.run_task(input, task_context).await })
    }
}
impl sauropod_task_context::Tool for TaskAsTool {
    fn get_name(&self) -> &str {
        "Task"
    }

    fn get_definition(&self) -> sauropod_schemas::ToolDefinition {
        self.definition.clone()
    }
}

/// Check whether a task is valid.
pub fn validate_task(task: sauropod_schemas::task::Task) -> anyhow::Result<()> {
    // Check that the template is parseable into an input schema.
    let _ = Task::new(task)?;
    Ok(())
}

fn default_task_output_schema() -> serde_json::Map<String, serde_json::Value> {
    serde_json::json!({
        "type": "object",
        "properties": {
            "output": {
                "type": "string",
                "description": "The content of the LLM response.",
            },
        },
        "required": ["output"],
        "additionalProperties": false,
    })
    .as_object()
    .unwrap()
    .clone()
}
