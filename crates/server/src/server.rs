//! HTTP server code.

use std::env;
use std::sync::Arc;

use sauropod_database::{DatabaseId, DatabaseTypeWithId, UserId};
use sauropod_schemas::InputAndOutputSchema;
use sauropod_schemas::{Task, TaskInfo};
use sauropod_task_context::TaskContext;
use tracing::Instrument;

use sauropod_http::HttpResponse;

use crate::observability::Observability;

/// The server contains the main state of the application.
pub struct Server {
    /// The server configuration.
    config: sauropod_config::Config,
    /// Observability state.
    observability: Observability,
    /// The database.
    db: Arc<sauropod_database::Database>,
    /// MCP interface.
    _mcp: Arc<sauropod_mcp::ModelContextProtocol>,
    /// All tools available to the server.
    tools: Vec<Arc<dyn sauropod_task_context::Tool>>,
    /// The LLM engine.
    llm_engine: sauropod_llm_inference::EnginePointer,
}

impl Server {
    pub async fn new(
        config: &sauropod_config::Config,
        log_buffer: std::sync::Arc<sauropod_logging::InMemoryLogBuffer>,
    ) -> anyhow::Result<std::sync::Arc<Self>> {
        // Use config to set up the database location
        let db_location = config
            .database_path
            .clone()
            .ok_or_else(|| anyhow::anyhow!("No database path configured"))?;
        let db = Arc::new(sauropod_database::create_database(db_location.as_ref()).await?);
        let mcp = sauropod_mcp::ModelContextProtocol::new(config)
            .instrument(tracing::info_span!("MCP initialization"))
            .await?;
        let mut tools = sauropod_core_tools::get_default_tools();
        tools.extend(mcp.clone().list_all_tools().await?);

        Ok(Arc::new(Self {
            config: config.clone(),
            observability: Observability { log_buffer },
            db,
            _mcp: mcp,
            tools,
            llm_engine: sauropod_llm_inference::create_engine(config).await?,
        }))
    }

    /// Create a task context.
    pub async fn make_task_context(&self, user_id: UserId) -> anyhow::Result<Arc<TaskContext>> {
        let model_config = &self.config.default_model;

        // Ensure that the models are available
        let available_models = self.llm_engine.list_models().await?;
        if !available_models
            .iter()
            .any(|m| m.name == model_config.model)
        {
            if self.llm_engine.can_pull_model() {
                self.llm_engine.pull_model(&model_config.model).await?;
            } else {
                anyhow::bail!(
                    "Model {} not available, the available models are:\n{}",
                    model_config.model,
                    available_models
                        .iter()
                        .map(|m| m.name.clone())
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
        }

        Ok(sauropod_task_context::TaskContext::new(
            user_id,
            self.llm_engine.clone(),
            model_config.clone(),
            self.tools.clone(),
            self.db.clone(),
        ))
    }
}

#[async_trait::async_trait]
impl sauropod_http::ServerInterface for Server {
    async fn get_health(
        &self,
    ) -> anyhow::Result<HttpResponse<sauropod_schemas::HealthCheckResponse>> {
        Ok(HttpResponse::Ok(sauropod_schemas::HealthCheckResponse {}))
    }

    async fn get_observability_logs(
        &self,
        _user_id: UserId,
    ) -> anyhow::Result<HttpResponse<sauropod_schemas::observability::LogResponse>> {
        Ok(HttpResponse::Ok(
            self.observability.get_observability_logs(),
        ))
    }

    async fn get_task_id(
        &self,
        user_id: UserId,
        id: DatabaseId,
    ) -> anyhow::Result<HttpResponse<Task>> {
        match Task::get_by_id(id, user_id, &self.db).await? {
            Some(object) => Ok(object.into()),
            None => Ok(HttpResponse::NotFound(None)),
        }
    }

    async fn delete_task_id(
        &self,
        user_id: UserId,
        id: DatabaseId,
    ) -> anyhow::Result<HttpResponse<()>> {
        match Task::delete_by_id(id, user_id, &self.db).await? {
            true => Ok(HttpResponse::Ok(())),
            false => Ok(HttpResponse::NotFound(None)),
        }
    }

    async fn post_task_id(
        &self,
        user_id: UserId,
        id: DatabaseId,
        input: Task,
    ) -> anyhow::Result<HttpResponse<()>> {
        if let Err(e) = sauropod_task::validate_task(input.clone()) {
            tracing::error!("Invalid task definition: {:#?}", e);
            return Ok(HttpResponse::BadRequest(e.to_string()));
        }

        match sauropod_database::Task::update(id, user_id, input, &self.db).await? {
            true => Ok(HttpResponse::Ok(())),
            false => Ok(HttpResponse::NotFound(None)),
        }
    }

    async fn post_task_id_run(
        &self,
        user_id: UserId,
        id: DatabaseId,
        input: serde_json::Map<String, serde_json::Value>,
    ) -> anyhow::Result<HttpResponse<serde_json::Map<String, serde_json::Value>>> {
        let task = match Task::get_by_id(id, user_id, &self.db).await? {
            Some(task) => sauropod_task::Task::new(task)?,
            None => {
                tracing::error!("Task not found: {id}");
                return Ok(HttpResponse::NotFound(None));
            }
        };

        let context = self.make_task_context(user_id).await?;
        let result = task.execute(serde_json::to_value(input)?, context).await?;
        let Some(result_map) = result.as_object() else {
            anyhow::bail!("Task result wasn't an object, was {:#?}", result);
        };
        Ok(HttpResponse::Ok(result_map.clone()))
    }

    async fn get_task_id_schema(
        &self,
        user_id: UserId,
        id: i64,
    ) -> anyhow::Result<HttpResponse<InputAndOutputSchema>> {
        let Some(task) = Task::get_by_id(id, user_id, &self.db).await? else {
            return Ok(HttpResponse::NotFound(None));
        };

        let internal_task = sauropod_task::Task::new(task)?;

        let input_schema = match internal_task.input_schema() {
            serde_json::Value::Object(obj) => obj,
            x => {
                tracing::error!("Generated schema wasn't an object: {:#?}", x);
                anyhow::bail!("Couldn't generated schema");
            }
        };
        let output_schema = match internal_task.output_schema() {
            serde_json::Value::Object(obj) => obj,
            x => {
                tracing::error!("Generated schema wasn't an object: {:#?}", x);
                anyhow::bail!("Couldn't generated schema");
            }
        };

        Ok(HttpResponse::Ok(InputAndOutputSchema {
            input_schema: input_schema.clone(),
            output_schema: output_schema.clone(),
        }))
    }

    async fn post_task(
        &self,
        user_id: UserId,
        input: Task,
    ) -> anyhow::Result<HttpResponse<DatabaseId>> {
        if let Err(e) = sauropod_task::validate_task(input.clone()) {
            tracing::error!("Invalid task definition: {:#?}", e);
            return Ok(HttpResponse::BadRequest(e.to_string()));
        }

        let id = sauropod_database::Task {
            owner_id: user_id.0,
            ..sauropod_database::Task::from(input)
        }
        .insert(&self.db)
        .await?;
        Ok(HttpResponse::Ok(id))
    }

    async fn get_task(&self, user_id: UserId) -> anyhow::Result<HttpResponse<Vec<TaskInfo>>> {
        Ok(HttpResponse::Ok(
            sauropod_database::Task::list(user_id, &self.db)
                .await
                .map(|tasks| {
                    tasks
                        .into_iter()
                        .map(|task| TaskInfo {
                            id: task.id,
                            name: task.name,
                        })
                        .collect()
                })?,
        ))
    }

    async fn get_tools(
        &self,
        user_id: UserId,
    ) -> anyhow::Result<HttpResponse<std::vec::Vec<sauropod_schemas::ToolDefinition>>> {
        let mut tools: Vec<_> = self
            .tools
            .iter()
            .map(|tool| tool.get_definition())
            .collect();

        let tasks_as_tools = sauropod_database::Task::list(user_id, &self.db)
            .await?
            .into_iter()
            .map(|task| sauropod_schemas::ToolDefinition {
                id: format!("{}{}", sauropod_task::TASK_TOOL_PREFIX, task.id),
                name: task.name,
                description: task.description,
                input_schema: serde_json::json!(task.input_schema.0),
                provider: "Task".to_string(),
            });
        tools.extend(tasks_as_tools);

        Ok(HttpResponse::Ok(tools))
    }

    async fn get_models(
        &self,
        _user_id: UserId,
    ) -> anyhow::Result<HttpResponse<std::vec::Vec<sauropod_schemas::ModelDefinition>>> {
        Ok(HttpResponse::Ok(self.llm_engine.list_models().await?))
    }

    async fn get_version(&self) -> anyhow::Result<HttpResponse<String>> {
        Ok(HttpResponse::Ok(env!("CARGO_PKG_VERSION").to_string()))
    }
}
