//! Schemas used in the Sauropod API.

pub mod observability;

pub mod task;

/// Response to a health check request.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HealthCheckResponse {}

/// An LLM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Model {
    /// The model ID.
    pub id: String,
    /// The user-friendly name of the model.
    pub name: String,
    /// The model URI.
    pub uri: String,
}

/// A tool definition.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolDefinition {
    /// A unique ID for the tool..
    pub id: String,
    /// The name of the tool.
    ///
    /// Must match `^[a-zA-Z0-9_-]{1,64}$`.
    pub name: String,
    /// The source that provides the tool.
    pub provider: String,
    /// A detailed description of what the tool does.
    pub description: String,
    /// A JSON Schema describing the parameters for the tool.
    ///
    /// See <https://json-schema.org/>.
    pub input_schema: serde_json::Value,
}

/// A model definition.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ModelDefinition {
    /// The human-friendly name of the model.
    pub name: String,
    /// The URI of the model.
    pub uri: String,
}

/// An error message.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Error {
    /// The error message.
    pub error: String,
}

/// Input and output schemas.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InputAndOutputSchema {
    /// The input schema for a task.
    pub input_schema: serde_json::Map<String, serde_json::Value>,
    /// The output schema for a task.
    pub output_schema: serde_json::Map<String, serde_json::Value>,
}
