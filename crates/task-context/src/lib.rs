//! Sauropod task context..

use std::{collections::HashMap, sync::Arc};

use sauropod_config::ModelConfig;
use sauropod_database::{DatabaseId, DatabaseTypeWithId as _, TaskStep, UserId};
use sauropod_schemas::Task;
mod traits;
pub use traits::*;

/// The context in which a task is executed.
///
/// The context provides tools to the task.
pub struct TaskContext {
    /// The user running the task.
    pub user_id: UserId,
    /// The ID of the current run.
    pub run_id: DatabaseId,
    /// The LLM engine to use for the task.
    pub llm_engine: sauropod_llm_inference::EnginePointer,
    /// The system prompt to use for the task.
    pub system_prompt: String,
    /// The tools available to the task.
    pub tools: HashMap<String, Arc<dyn Tool>>,
    /// The model to use to invoke the task.
    pub model_config: ModelConfig,
    /// The database.
    pub db: Arc<sauropod_database::Database>,
}

const DEFAULT_SYSTEM_PROMPT: &str = r#"
You are an automation that executes user tasks.
Your response may be used as part of a larger system or as input to other automation tools.
Do not ask for clarification or additional information.
"#;

tokio::task_local! {
    /// The current parent task ID.
    pub static PARENT_TASK_ID: Option<DatabaseId>;
}

impl TaskContext {
    /// Create a new task context.
    pub fn new(
        user_id: UserId,
        run_id: DatabaseId,
        llm_engine: sauropod_llm_inference::EnginePointer,
        model_config: ModelConfig,
        tools: Vec<Arc<dyn Tool>>,
        db: Arc<sauropod_database::Database>,
    ) -> Arc<Self> {
        let mut tool_map = HashMap::with_capacity(tools.len());
        for tool in tools.into_iter() {
            tool_map.insert(tool.get_id().to_string(), tool);
        }

        Arc::new(Self {
            user_id,
            run_id,
            llm_engine,
            system_prompt: DEFAULT_SYSTEM_PROMPT.trim().to_string(),
            tools: tool_map,
            model_config,
            db,
        })
    }

    /// Get a model name.
    pub fn get_model(&self) -> &ModelConfig {
        &self.model_config
    }

    /// Get a task by ID.
    pub async fn get_task(&self, id: i64) -> anyhow::Result<Option<sauropod_schemas::Task>> {
        match Task::get_by_id(id, self.user_id, &self.db).await? {
            Some(task) => Ok(Some(task)),
            None => {
                tracing::error!("Task with ID {id} not found");
                Ok(None)
            }
        }
    }

    /// Create a task run step.
    pub async fn create_run_step_for_task(
        &self,
        task_id: DatabaseId,
        inputs: &serde_json::Value,
    ) -> anyhow::Result<DatabaseId> {
        let parent_step_id = PARENT_TASK_ID
            .try_with(|parent_id| *parent_id)
            .unwrap_or(None);

        Ok(TaskStep {
            step_id: 0,
            run_id: self.run_id,
            owner_id: self.user_id.0,
            parent_step_id,
            inputs: sqlx::types::Json(inputs.clone()),
            outputs: sqlx::types::Json(serde_json::json!(Option::<serde_json::Value>::None)),
            task_id: Some(task_id),
            tool_id: None,
            error: None,
            start_time: Some(chrono::Utc::now()),
            end_time: None,
        }
        .insert(&self.db)
        .await?)
    }

    /// Create a tool run step.
    pub async fn create_tool_step_for_task(
        &self,
        tool_id: String,
        inputs: &serde_json::Value,
    ) -> anyhow::Result<DatabaseId> {
        let parent_step_id = PARENT_TASK_ID
            .try_with(|parent_id| *parent_id)
            .unwrap_or(None);
        Ok(TaskStep {
            step_id: 0,
            run_id: self.run_id,
            owner_id: self.user_id.0,
            parent_step_id,
            inputs: sqlx::types::Json(inputs.clone()),
            outputs: sqlx::types::Json(serde_json::json!(Option::<serde_json::Value>::None)),
            task_id: None,
            tool_id: Some(tool_id),
            error: None,
            start_time: Some(chrono::Utc::now()),
            end_time: None,
        }
        .insert(&self.db)
        .await?)
    }

    /// Create a tool run step.
    pub async fn report_step_result<T: Clone + Into<serde_json::Value>>(
        &self,
        step_id: i64,
        result: anyhow::Result<T>,
    ) -> anyhow::Result<T> {
        match result {
            Ok(value) => {
                let json_value: serde_json::Value = value.clone().into();
                if let Err(db_err) = TaskStep::set_success(step_id, json_value, &self.db).await {
                    tracing::error!("Failed to record successful step result: {}", db_err);
                }
                Ok(value)
            }
            Err(err) => {
                let error_message = err.to_string();
                if let Err(db_err) = TaskStep::set_failure(step_id, error_message, &self.db).await {
                    tracing::error!("Failed to record step failure: {}", db_err);
                }
                Err(err)
            }
        }
    }
}
