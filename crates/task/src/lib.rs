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
pub async fn task_from_schema(task: sauropod_schemas::task::Task) -> anyhow::Result<TaskArc> {
    match task.action {
        sauropod_schemas::task::TaskAction::InvokeLLM(invoke_llm) => {
            Ok(Arc::new(invoke_llm::InvokeLlmTask::new(invoke_llm).await?))
        }
    }
}

/// Get the JSON Schema from a task's schema representation.
pub fn input_schema_from_task_schema(
    task: &sauropod_schemas::task::Task,
) -> anyhow::Result<serde_json::Value> {
    Ok(match &task.action {
        sauropod_schemas::task::TaskAction::InvokeLLM(invoke_llm) => {
            sauropod_prompt_templates::template_string_to_inputs(&invoke_llm.template.0)?
        }
    })
}
