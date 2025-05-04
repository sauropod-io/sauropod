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

/// Content type for text in a message
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TextContent {
    pub text: String,
}

/// Content type for image URL in a message
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageUrlContent {
    pub image_url: String,
}

/// Content type in a message
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ContentItem {
    #[serde(rename = "text")]
    Text(TextContent),
    #[serde(rename = "image_url")]
    ImageUrl(ImageUrlContent),
}

impl From<String> for ContentItem {
    fn from(text: String) -> Self {
        ContentItem::Text(TextContent { text })
    }
}

/// `content` field of a message.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MessageContent {
    /// Just a single text item.
    #[serde(rename = "text")]
    Text(String),
    /// A vector of content items.
    #[serde(rename = "image_url")]
    ContentVec(Vec<ContentItem>),
}

impl From<String> for MessageContent {
    fn from(text: String) -> Self {
        Self::ContentVec(vec![ContentItem::Text(TextContent { text })])
    }
}

/// A single message in a chat conversation.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: Option<MessageContent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl Message {
    /// Helper to create a message with simple text content.
    pub fn with_text(role: Role, text: String) -> Self {
        Self::with_content(role, vec![ContentItem::from(text)])
    }

    /// Helper to create a message a content array.
    pub fn with_content(role: Role, content: Vec<ContentItem>) -> Self {
        Self {
            role,
            content: Some(MessageContent::ContentVec(content)),
            tool_calls: vec![],
            tool_call_id: None,
        }
    }
}

/// A tool.
#[derive(Debug, serde::Serialize)]
pub struct Function {
    name: String,
    description: String,
    parameters: serde_json::Value,
    strict: bool,
}

impl From<sauropod_schemas::ToolDefinition> for Function {
    fn from(def: sauropod_schemas::ToolDefinition) -> Self {
        Function {
            name: def.id,
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
    pub object: String,
    pub created: i64,
    pub owned_by: String,
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
    pub fn new(client: reqwest::Client, url: String) -> Self {
        let completions_url = format!("{}/chat/completions", &url);
        Self {
            completions_url,
            models_url: format!("{}/models", &url),
            client,
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
        let response = self.client.get(&self.models_url).send().await?;

        if response.status().is_success() {
            return Ok(response.json().await?);
        }

        let status = response.status();
        let body = response.text().await?;
        tracing::error!("Error from {} (code {status}): {body}", &self.models_url);
        anyhow::bail!("Error from {} (code {status}): {body}", &self.models_url);
    }
}
