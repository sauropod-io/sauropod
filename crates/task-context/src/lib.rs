//! Sauropod task context..

use std::{collections::HashMap, sync::Arc};

use sauropod_config::ModelConfig;
use sauropod_database::DatabaseTypeWithId as _;
use sauropod_schemas::task::Task;
mod traits;
pub use traits::*;

/// The context in which a task is executed.
///
/// The context provides tools to the task.
pub struct TaskContext {
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

impl TaskContext {
    /// Create a new task context.
    pub fn new(
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
    pub async fn get_task(&self, id: i64) -> anyhow::Result<Option<sauropod_schemas::task::Task>> {
        match Task::get_by_id(id, &self.db).await? {
            Some(task) => Ok(Some(task)),
            None => {
                tracing::error!("Task with ID {id} not found");
                Ok(None)
            }
        }
    }
}
