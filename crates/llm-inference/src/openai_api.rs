//! OpenAI API interface.

/// An interface to OpenAI-compatible APIs.
#[derive(Clone)]
pub struct OpenAiInterface {
    /// Completions URL
    completions_url: String,
    /// Models URL
    models_url: String,
    /// HTTP client
    client: reqwest::Client,
}

/// Message role in a chat conversation
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}

/// A function call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FunctionCall {
    pub arguments: String,
    pub name: String,
}

/// A tool call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolCall {
    pub function: Option<FunctionCall>,
    pub r#type: String,
    pub id: String,
}

/// A single message in a chat conversation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

/// A tool.
#[derive(Debug, serde::Serialize)]
pub struct Function {
    name: String,
    description: String,
    parameters: serde_json::Value,
    strict: bool,
}

impl From<sauropod_tool_spec::ToolDefinition> for Function {
    fn from(def: sauropod_tool_spec::ToolDefinition) -> Self {
        Function {
            name: def.name,
            description: def.description,
            parameters: def.input_schema,
            strict: false,
        }
    }
}

/// A tool.
#[derive(Debug, serde::Serialize)]
pub struct Tool {
    pub function: Function,
    pub r#type: String,
}

/// Response JSON schema format field.
#[derive(Debug, serde::Serialize)]
pub struct ResponseJsonSchema {
    pub name: String,
    pub strict: bool,
    pub schema: serde_json::Value,
}

/// Response format field.
#[derive(Debug, serde::Serialize)]
#[serde(tag = "type")]
pub enum ResponseFormat {
    #[serde(rename = "json_schema")]
    JsonSchema { json_schema: ResponseJsonSchema },
}

/// Request to the chat completions endpoint.
#[derive(Debug, serde::Serialize, Default)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

/// Reason for the completion call to finish.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum FinishReason {
    /// The model hit a stopping point.
    #[serde(rename = "stop")]
    Stop,
    /// The model has reached the maximum number of tokens.
    #[serde(rename = "length")]
    Length,
    /// Content filter.
    #[serde(rename = "content_filter")]
    ContentFilter,
    /// The model has errored.
    #[serde(rename = "tool_calls")]
    ToolCalls,
}

/// Choice returned by the API.
#[derive(Debug, serde::Deserialize)]
#[allow(unused)]
pub struct Choice {
    pub message: Message,
    pub index: u32,
    pub finish_reason: FinishReason,
}

/// Response from the chat completions endpoint.
#[derive(Debug, serde::Deserialize)]
#[allow(unused)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

/// Response from the models endpoint.
#[derive(Debug, serde::Deserialize)]
#[allow(unused)]
pub struct ModelData {
    pub id: String,
    object: String,
    created: i64,
    owned_by: String,
}

/// Response from the models endpoint.
#[derive(Debug, serde::Deserialize)]
pub struct ModelsResponse {
    #[allow(unused)]
    pub object: String,
    pub data: Option<Vec<ModelData>>,
}

impl OpenAiInterface {
    /// Create a new OpenAI interface.
    pub fn new(url: String) -> Self {
        let completions_url = format!("{}/chat/completions", &url);
        Self {
            completions_url,
            models_url: format!("{}/models", &url),
            client: reqwest::Client::new(),
        }
    }

    /// Call the chat completions endpoint to generate a completion.
    pub async fn completions(
        &self,
        request: &CompletionRequest,
    ) -> anyhow::Result<CompletionResponse> {
        tracing::debug!("Request to {}: {:#?}", &self.completions_url, &request);
        let response = self
            .client
            .post(&self.completions_url)
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await?;
            tracing::error!(
                "Error from {} (code {status}): {body}",
                &self.completions_url
            );
            anyhow::bail!(
                "Error from {} (code {status}): {body}",
                &self.completions_url
            );
        }

        let completion = response.json().await?;
        tracing::debug!(
            "Response from {}: {:#?}",
            &self.completions_url,
            &completion
        );
        Ok(completion)
    }

    /// Get a list of available models.
    pub async fn models(&self) -> anyhow::Result<ModelsResponse> {
        let response = self
            .client
            .get(&self.models_url)
            .send()
            .await?
            .error_for_status()?;

        Ok(response.json().await?)
    }
}
