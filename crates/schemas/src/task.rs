//! Task-related schemas.

/// A template written with Jinja2 syntax.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Template(
    #[cfg_attr(
        feature = "json_schema",
        schemars(example = "Classify this sentence: {{ input.sentence }}")
    )]
    pub String,
);

/// Enum for model strength.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum ModelStrength {
    /// Weak model.
    Weak,
    /// Strong model.
    Strong,
}

/// Run an LLM.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InvokeLLM {
    /// The template to use.
    pub template: Template,
    /// The model strength to use.
    pub model_strength: ModelStrength,
    /// The output schema.
    ///
    /// If unspecified the task will return an object with a single "output" string.
    pub output_schema: Option<serde_json::Map<String, serde_json::Value>>,
    /// The IDs of tools to make available to the LLM.
    #[serde(default)]
    pub available_tool_ids: Vec<String>,
}

/// A description of the action associated with a task.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum TaskAction {
    /// Invoke an LLM.
    InvokeLLM(InvokeLLM),
}

/// A task ID.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TaskId {
    /// The ID of the task.
    #[cfg_attr(feature = "json_schema", schemars(example = 12345))]
    pub task_id: i64,
}

/// A task is the smallest unit of work in a workflow.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Task {
    /// The name of the task.
    #[cfg_attr(feature = "json_schema", schemars(example = "Classify Email"))]
    pub name: String,
    /// The action that the task performs.
    pub action: TaskAction,
}
