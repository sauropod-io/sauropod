//! Observability-related schemas.

use std::str::FromStr;

/// The log level.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum LogLevel {
    /// Debug log level.
    Debug,
    /// Info log level.
    Info,
    /// Warning log level.
    Warning,
    /// Error log level.
    Error,
}

/// A logged message.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LogMessage {
    /// The module where the log message was emitted.
    pub module: String,
    /// The number of milliseconds since epoch.
    pub timestamp_ms: i64,
    /// Structured logging fields.
    pub fields: std::collections::HashMap<String, serde_json::Value>,
    /// The line number where the log message was emitted.
    pub line: Option<u32>,
    /// The log level.
    pub level: LogLevel,
}

/// A list of logged messages.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LogResponse(pub Vec<LogMessage>);

/// A step of a task run.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum StepAction {
    /// A step that is a task.
    TaskId(i64),
    /// A step that is a tool.
    ToolId(String),
}

/// A step of a task.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Step {
    /// The ID of the step.
    pub step_id: i64,
    /// The ID of the parent.
    pub parent_step_id: Option<i64>,
    /// The name of the task if this step is a task run.
    pub task_name: Option<String>,
    /// The inputs to the step.
    pub inputs: serde_json::Value,
    /// The outputs from the step.
    pub outputs: serde_json::Value,
    /// An error message if the step failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The action the step took.
    pub step_action: StepAction,
    /// The start of the step in milliseconds since UTC epoch.
    pub start_time_ms: Option<i64>,
    /// The end time of the step in milliseconds since UTC epoch.
    pub end_time_ms: Option<i64>,
}

/// Status of a run of a step or task.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase", deny_unknown_fields)]
pub enum RunStatus {
    /// The run is running.
    Running,
    /// The run is completed.
    Completed,
    /// The run failed.
    Failed,
}

impl RunStatus {
    pub fn to_str(&self) -> &'static str {
        match self {
            RunStatus::Running => "running",
            RunStatus::Completed => "completed",
            RunStatus::Failed => "failed",
        }
    }
}

impl FromStr for RunStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "running" => Ok(RunStatus::Running),
            "completed" => Ok(RunStatus::Completed),
            "failed" => Ok(RunStatus::Failed),
            _ => Err(anyhow::format_err!("Invalid run status: {}", s)),
        }
    }
}

/// A run of a task.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TaskRun {
    /// The ID of the run.
    pub id: i64,
    /// The steps in the run.
    pub steps: Vec<Step>,
    /// The overall status of the run.
    pub status: RunStatus,
    /// The start of the run in milliseconds since UTC epoch.
    pub start_time_ms: Option<i64>,
    /// The end time of the run in milliseconds since UTC epoch.
    pub end_time_ms: Option<i64>,
}

/// Information about a task run.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TaskRunInfo {
    /// The ID of the run.
    pub id: i64,
    /// The status of the run.
    pub status: RunStatus,
    /// The start of the run in milliseconds since UTC epoch.
    pub start_time_ms: Option<i64>,
    /// The end time of the run in milliseconds since UTC epoch.
    pub end_time_ms: Option<i64>,
}
