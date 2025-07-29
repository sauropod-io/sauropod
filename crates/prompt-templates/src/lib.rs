//! Prompt templates for LLMs.

use anyhow::Context as _;

#[derive(serde::Serialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RenderContextRole {
    User,
    System,
    Assistant,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct Tool {
    /// The name of the tool.
    pub name: String,
    /// The description of the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters of the tool in JSON format.
    pub parameters: serde_json::Map<String, serde_json::Value>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct RenderContextMessage {
    /// The role of the message.
    pub role: RenderContextRole,
    /// The content of the message.
    pub content: String,
    /// Tools that the model may use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<String>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct RenderContext {
    /// The messages to render.
    pub messages: Vec<RenderContextMessage>,
    /// Whether to add a generation prompt.
    pub add_generation_prompt: bool,
    /// Tools that the model may use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,
}

impl RenderContext {
    /// Creates a new `RenderContext` from the `CreateResponse` request.
    pub fn from_create_response(
        request: &sauropod_openai_api::CreateResponse,
        system_prompt: Option<&str>,
    ) -> anyhow::Result<crate::RenderContext> {
        let function_tools = request.response_properties.tools.as_ref().map(|tools| {
            tools
                .iter()
                .filter_map(|tool| {
                    if let sauropod_openai_api::Tool::FunctionTool {
                        description,
                        name,
                        parameters,
                        ..
                    } = tool
                    {
                        Some(Tool {
                            name: name.clone(),
                            description: description.clone(),
                            parameters: parameters.clone().unwrap_or_default(),
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        });
        let function_tool_string = function_tools
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| anyhow::anyhow!("Failed to serialize function tools: {}", e))?;

        // Combine the model's prompt with the instructions in the request
        let instructions = if let Some(system_prompt) = system_prompt {
            let env = minijinja::Environment::new();
            let system_prompt = env
                .render_str(
                    system_prompt.trim(),
                    serde_json::json!({
                        "tools": function_tool_string,
                    }),
                )
                .context("Failed to process system prompt template")?;
            if let Some(instructions) = request.instructions.as_deref() {
                Some(
                    format!("{system_prompt}\n{instructions}")
                        .trim()
                        .to_string(),
                )
            } else {
                Some(system_prompt.trim().to_string())
            }
        } else {
            request.instructions.clone()
        };

        let mut messages = Vec::new();
        let mut result = Ok(());
        if let Some(input) = &request.input {
            input.for_each(|message| match message {
                sauropod_openai_api::InputItem::EasyInputMessage(msg) => {
                    let role = match msg.role {
                        sauropod_openai_api::EasyInputMessageRole::Assistant => {
                            crate::RenderContextRole::Assistant
                        }
                        sauropod_openai_api::EasyInputMessageRole::User => {
                            crate::RenderContextRole::User
                        }
                        sauropod_openai_api::EasyInputMessageRole::System => {
                            crate::RenderContextRole::System
                        }
                        sauropod_openai_api::EasyInputMessageRole::Developer => {
                            tracing::warn!("Unhandled developer role");
                            crate::RenderContextRole::System
                        }
                    };

                    msg.content.for_each(|content| match content {
                        sauropod_openai_api::InputContent::InputTextContent(text_content) => {
                            messages.push(crate::RenderContextMessage {
                                role,
                                content: text_content.text.to_string(),
                                tools: None,
                            })
                        }
                        sauropod_openai_api::InputContent::InputImageContent(_) => {
                            result = Err(anyhow::anyhow!("InputImageContent not handled"));
                        }
                        sauropod_openai_api::InputContent::InputFileContent(_) => {
                            result = Err(anyhow::anyhow!("InputFileContent not handled"));
                        }
                    });
                }
                sauropod_openai_api::InputItem::Item(sauropod_openai_api::Item::InputMessage(
                    msg,
                )) => {
                    let role = match msg.role {
                        sauropod_openai_api::InputMessageRole::User => {
                            crate::RenderContextRole::User
                        }
                        sauropod_openai_api::InputMessageRole::System => {
                            crate::RenderContextRole::System
                        }
                        sauropod_openai_api::InputMessageRole::Developer => {
                            tracing::warn!("Unhandled developer role");
                            crate::RenderContextRole::System
                        }
                    };

                    msg.content.0.iter().for_each(|content| match content {
                        sauropod_openai_api::InputContent::InputTextContent(text_content) => {
                            messages.push(crate::RenderContextMessage {
                                role,
                                content: text_content.text.to_string(),
                                tools: None,
                            })
                        }
                        sauropod_openai_api::InputContent::InputImageContent(_) => {
                            result = Err(anyhow::anyhow!("InputImageContent not handled"));
                        }
                        sauropod_openai_api::InputContent::InputFileContent(_) => {
                            result = Err(anyhow::anyhow!("InputFileContent not handled"));
                        }
                    });
                }
                sauropod_openai_api::InputItem::Item(
                    sauropod_openai_api::Item::OutputMessage { content, .. },
                ) => {
                    for item in content {
                        match item {
                            sauropod_openai_api::OutputContent::OutputTextContent(text_content) => {
                                messages.push(crate::RenderContextMessage {
                                    role: crate::RenderContextRole::Assistant,
                                    content: text_content.text.to_string(),
                                    tools: None,
                                })
                            }
                            sauropod_openai_api::OutputContent::RefusalContent(_) => {
                                result = Err(anyhow::anyhow!("RefusalContent not handled"));
                            }
                        }
                    }
                }
                sauropod_openai_api::InputItem::Item(
                    sauropod_openai_api::Item::FunctionToolCall { arguments, .. },
                ) => {
                    // TODO support multiple model formats
                    messages.push(crate::RenderContextMessage {
                        role: crate::RenderContextRole::Assistant,
                        content: format!(r#"```tool_call{arguments}```"#),
                        tools: None,
                    });
                }
                sauropod_openai_api::InputItem::Item(
                    sauropod_openai_api::Item::FunctionCallOutputItemParam { output, .. },
                ) => {
                    // TODO support multiple model formats
                    messages.push(crate::RenderContextMessage {
                        role: crate::RenderContextRole::User,
                        content: format!("```tool_call_result\n{output}\n```"),
                        tools: None,
                    });
                }
                sauropod_openai_api::InputItem::Item(item) => {
                    result = Err(anyhow::anyhow!("Item not handled: {:#?}", item));
                }
                sauropod_openai_api::InputItem::ItemReferenceParam(item_ref) => {
                    result = Err(anyhow::anyhow!(
                        "Item reference not handled: {:#?}",
                        item_ref
                    ));
                }
            });
        }

        if let Some(system_message_index) = messages
            .iter()
            .position(|m| m.role == crate::RenderContextRole::System)
        {
            messages[system_message_index].tools = function_tool_string.clone();
            if let Some(instructions) = &instructions {
                // If instructions are provided, append them to the system message
                messages[system_message_index].content = instructions.to_string();
            }
        } else {
            // If no system message is present, add a default one
            messages.insert(
                0,
                crate::RenderContextMessage {
                    role: crate::RenderContextRole::System,
                    content: instructions.clone().unwrap_or_default(),
                    tools: function_tool_string.clone(),
                },
            );
        }

        result?;

        Ok(crate::RenderContext {
            messages,
            tools: function_tools,
            add_generation_prompt: true,
        })
    }
}

/// A prompt template.
pub struct PromptTemplate {
    template_name: &'static str,
    environment: minijinja::Environment<'static>,
}

fn raise_exception(msg: String) -> Result<(), minijinja::Error> {
    Err(minijinja::Error::new(
        minijinja::ErrorKind::UndefinedError,
        msg,
    ))
}

impl PromptTemplate {
    const DEFAULT_TEMPLATE_NAME: &'static str = "chat_template";

    /// Create a new `PromptTemplate` with the given template string.
    pub fn new(
        template: impl Into<std::borrow::Cow<'static, str>>,
    ) -> Result<Self, minijinja::Error> {
        let mut environment = minijinja::Environment::new();
        environment.add_function("raise_exception", raise_exception);
        environment.add_template_owned(Self::DEFAULT_TEMPLATE_NAME, template)?;
        Ok(PromptTemplate {
            template_name: Self::DEFAULT_TEMPLATE_NAME,
            environment,
        })
    }

    /// Render the prompt template with the given context.
    pub fn render(&self, context: &RenderContext) -> Result<String, minijinja::Error> {
        self.environment
            .get_template(self.template_name)
            .unwrap()
            .render(context)
    }
}
