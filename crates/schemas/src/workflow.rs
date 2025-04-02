//! Workflow-related schemas.

use std::collections::BTreeMap;

use crate::task::TaskId;

/// A connection.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", untagged, deny_unknown_fields)]
pub enum Connection {
    /// A parameter passed in from outside the workflow.
    Parameter {
        /// The name of the parameter.
        parameter: String,
        /// The task parameters to connect the workflow parameter to.
        to: Vec<String>,
    },
    /// A connection between the output of one task and the input of another.
    Task {
        /// The ID of the task that produces the output.
        from: String,
        /// The ID of the task that consumes the output.
        to: Vec<String>,
    },
}

/// A task a workflow will perform.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", untagged, deny_unknown_fields)]
pub enum WorkflowAction {
    /// Run a task.
    RunTask(TaskId),
    /// Run a workflow.
    RunWorkflow(WorkflowId),
    /// Run a tool.
    RunTool {
        #[serde(rename = "toolId")]
        tool_id: String,
    },
}

/// A workflow ID.
#[derive(Debug, Copy, Hash, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct WorkflowId {
    /// The ID of the workflow.
    #[cfg_attr(feature = "json_schema", schemars(example = 12345))]
    pub workflow_id: i64,
}

/// A workflow.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Workflow {
    /// The name of the workflow.
    #[cfg_attr(feature = "json_schema", schemars(example = "Process shipping emails"))]
    pub name: String,
    /// The tasks in the workflow.
    ///
    /// The keys are the IDs of the tasks.
    pub actions: BTreeMap<String, WorkflowAction>,
    /// A mapping of connections between tasks.
    ///
    /// # Example
    ///
    /// ```json
    /// [
    ///   {
    ///     // Make the workflow parameter `url` available to the task `foo`.
    ///     "parameter": "url",
    ///     "to": ["foo.url"]
    ///   },
    ///   {
    ///     // Pipe the output of the `foo` task to the  `content` parameter of task `bar`.
    ///     "from": "foo.output",
    ///     "to": ["bar.content"]
    ///   }
    /// ]
    /// ```
    #[serde(default)]
    pub connections: Vec<Connection>,
}

/// A parsed template.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ParsedTemplate {
    /// The variables extracted from the template.
    pub variables: serde_json::Map<String, serde_json::Value>,
}

/// Minimal information describing a stored object.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ObjectInfo {
    /// The ID of the object.
    ///
    /// This ID can be used to retrieve the contents of the object.
    pub id: i64,
    /// The name of the object.
    pub name: String,
}
