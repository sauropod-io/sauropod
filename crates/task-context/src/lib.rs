//! Sauropod task context..

use std::{collections::BTreeMap, sync::Arc};

use sauropod_config::ModelConfig;

/// The context in which a task is executed.
///
/// The context provides tools to the task.
pub struct TaskContext {
    /// The LLM engine to use for the task.
    pub llm_engine: sauropod_llm_inference::EnginePointer,
    /// The system prompt to use for the task.
    pub system_prompt: String,
    /// The tools available to the task.
    pub tools: BTreeMap<String, Arc<dyn sauropod_tool_spec::Tool>>,
    /// A mapping from model strength enums to model names.
    pub model_names: BTreeMap<sauropod_schemas::task::ModelStrength, ModelConfig>,
}

const DEFAULT_SYSTEM_PROMPT: &str = r#"
You are an automation that executes user tasks.
Your response may be used as part of a larger system or as input to other automation tools.
Do not ask for clarification or additional information.
If you are unable to complete the task, respond with an error message in JSON format, for example `{ "error": "Could not read /example.txt" }`.
"#;

impl TaskContext {
    /// Create a new task context.
    pub fn new(llm_engine: sauropod_llm_inference::EnginePointer) -> Self {
        Self {
            llm_engine,
            system_prompt: DEFAULT_SYSTEM_PROMPT.trim().to_string(),
            tools: BTreeMap::new(),
            model_names: BTreeMap::new(),
        }
    }

    /// Register a tool with the context.
    pub fn register_tool(&mut self, tool: Arc<dyn sauropod_tool_spec::Tool>) {
        self.tools.insert(tool.get_definition().name, tool);
    }

    /// Get a model name.
    pub fn get_model(
        &self,
        model_strength: sauropod_schemas::task::ModelStrength,
    ) -> anyhow::Result<ModelConfig> {
        if let Some(model) = self.model_names.get(&model_strength) {
            return Ok(model.clone());
        }

        let alternate_size = match model_strength {
            sauropod_schemas::task::ModelStrength::Weak => {
                sauropod_schemas::task::ModelStrength::Strong
            }
            sauropod_schemas::task::ModelStrength::Strong => {
                sauropod_schemas::task::ModelStrength::Weak
            }
        };
        if let Some(model) = self.model_names.get(&alternate_size) {
            tracing::warn!(
                "No model found for {model_strength:?}, using {alternate_size:?} instead"
            );
            return Ok(model.clone());
        }
        anyhow::bail!("No models available in the task context");
    }
}

/// Get the default list of tools.
pub fn get_default_tools() -> Vec<Arc<dyn sauropod_tool_spec::Tool>> {
    vec![Arc::new(sauropod_core_tools::fetch::FetchTool)]
}

/// Construct a task context with the default tools.
pub fn make_default_task_context(
    llm_engine: sauropod_llm_inference::EnginePointer,
    model_names: BTreeMap<sauropod_schemas::task::ModelStrength, ModelConfig>,
) -> Arc<TaskContext> {
    let mut context = TaskContext::new(llm_engine);
    for tool in get_default_tools() {
        context.register_tool(tool);
    }
    context.model_names = model_names;
    Arc::new(context)
}
