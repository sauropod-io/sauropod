//! Sauropod workflow execution.

use std::sync::Arc;

mod invoke_llm;

pub type TaskArc = Arc<dyn Task + Send + Sync + 'static>;

/// A task is the smallest unit of work in a workflow essentially it is a mapping from T -> U.
#[async_trait::async_trait]
pub trait Task {
    /// Execute the task.
    async fn execute(
        &self,
        input: serde_json::Value,
        context: Arc<sauropod_task_context::TaskContext>,
    ) -> anyhow::Result<serde_json::Value>;

    /// The input schema for the task.
    fn input_schema(&self) -> &serde_json::Value;

    /// The output schema for the task.
    fn output_schema(&self) -> &serde_json::Value;
}

/// Create a task from its schema representation.
pub fn task_from_schema(task: sauropod_schemas::task::Task) -> anyhow::Result<TaskArc> {
    Ok(Arc::new(invoke_llm::InvokeLlmTask::new(task)?))
}

/// Check whether a task is valid.
pub fn validate_task(task: sauropod_schemas::task::Task) -> anyhow::Result<()> {
    // Check that the template is parseable into an input schema.
    let _ = invoke_llm::InvokeLlmTask::new(task)?;
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
