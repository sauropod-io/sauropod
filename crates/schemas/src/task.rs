//! Task-related schemas.

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
