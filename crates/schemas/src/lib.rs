//! Schemas used in the Sauropod API.

pub mod observability;

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

/// A template.
///
/// Variables in templates are defined using `${variableName}` syntax.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Template(
    #[cfg_attr(
        feature = "json_schema",
        schemars(example = "Classify this sentence: ${sentence}")
    )]
    pub String,
);

/// ID of a task.
pub type TaskId = i64;

/// A task is the smallest unit of work in a workflow.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Task {
    /// The name of the task.
    #[cfg_attr(feature = "json_schema", schemars(example = "Classify Email"))]
    pub name: String,
    /// The template to use.
    pub template: Template,
    /// The output schema.
    ///
    /// If unspecified the task will return an object with a single "output" string.
    pub output_schema: Option<serde_json::Map<String, serde_json::Value>>,
    /// The input schema of a task.
    ///
    /// Each key in this object should be a variable name in `template`.
    #[serde(default)]
    pub input_schema: serde_json::Map<String, serde_json::Value>,
    /// The IDs of tools to make available to the LLM.
    #[serde(default)]
    pub available_tool_ids: Vec<String>,
}

/// Minimal information describing a stored task.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TaskInfo {
    /// The ID of the task.
    ///
    /// This ID can be used to retrieve the contents of the task.
    #[cfg_attr(feature = "json_schema", schemars(example = 12345))]
    pub id: i64,
    /// The name of the task.
    #[cfg_attr(feature = "json_schema", schemars(example = "Classify Email"))]
    pub name: String,
}
