//! HTTP server code.

use std::env;
use std::sync::Arc;

use sauropod_database::{DatabaseId, DatabaseTypeWithID, DatabaseTypeWithName};
use sauropod_schemas::InputAndOutputSchema;
use sauropod_schemas::task::{ObjectInfo, Task};
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
    db: sauropod_database::Database,
    /// MCP interface.
    _mcp: Arc<sauropod_mcp::ModelContextProtocol>,
    /// All tools available to the server.
    tools: Vec<Arc<dyn sauropod_tool_spec::Tool>>,
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
        let db = sauropod_database::Database::new(db_location.into())?;
        db.init()?;
        let mcp = sauropod_mcp::ModelContextProtocol::new(config)
            .instrument(tracing::info_span!("MCP initialization"))
            .await?;
        let mut tools = sauropod_task_context::get_default_tools();
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
    pub async fn make_task_context(&self) -> anyhow::Result<Arc<TaskContext>> {
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
            self.llm_engine.clone(),
            model_config.clone(),
            self.tools.clone(),
        ))
    }
}

/// HTTP endpoint to get an object from the database by ID.
fn db_get_object<T>(
    database: &sauropod_database::Database,
    id: DatabaseId,
) -> anyhow::Result<HttpResponse<T>>
where
    for<'de> T: DatabaseTypeWithID + serde::Deserialize<'de>,
{
    match database.get_by_id::<T>(id)? {
        Some(object) => Ok(object.into()),
        None => Ok(HttpResponse::NotFound(None)),
    }
}

/// HTTP endpoint to delete an object from the database by ID.
fn db_delete_object<T: DatabaseTypeWithID>(
    database: &sauropod_database::Database,
    id: DatabaseId,
) -> anyhow::Result<HttpResponse<()>> {
    match database.delete_by_id::<T>(id)? {
        true => Ok(HttpResponse::Ok(())),
        false => Ok(HttpResponse::NotFound(None)),
    }
}

/// HTTP endpoint to update an object in the database by ID.
fn db_update_object<T: DatabaseTypeWithID + serde::Serialize>(
    database: &sauropod_database::Database,
    id: DatabaseId,
    input: T,
) -> anyhow::Result<HttpResponse<()>> {
    match database.update_by_id(id, &input)? {
        true => Ok(HttpResponse::Ok(())),
        false => Ok(HttpResponse::NotFound(None)),
    }
}

/// HTTP endpoint to create an object in the database.
fn db_create_object<T: DatabaseTypeWithID + serde::Serialize>(
    database: &sauropod_database::Database,
    input: T,
) -> anyhow::Result<HttpResponse<DatabaseId>> {
    Ok(HttpResponse::Ok(database.insert(&input)?))
}

/// HTTP endpoint to list the objects in the database.
fn db_list_objects<T: DatabaseTypeWithID + DatabaseTypeWithName>(
    database: &sauropod_database::Database,
    limit: Option<i64>,
) -> anyhow::Result<HttpResponse<Vec<ObjectInfo>>> {
    let mut query = format!(
        "SELECT id, json_extract(content, '$.name') as name FROM {}",
        T::table_name()
    );
    if let Some(limit) = limit {
        query.push_str(&format!(" LIMIT {}", limit));
    }

    let object_infos = database.with_connection(move |connection| -> anyhow::Result<_> {
        let mut statement = connection.prepare(&query)?;
        let mut object_infos = Vec::with_capacity(16);
        let objects = statement.query_map([], |row| {
            Ok(ObjectInfo {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        for object in objects {
            object_infos.push(object?);
        }
        Ok(object_infos)
    })?;

    Ok(HttpResponse::Ok(object_infos))
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
    ) -> anyhow::Result<HttpResponse<sauropod_schemas::observability::LogResponse>> {
        Ok(HttpResponse::Ok(
            self.observability.get_observability_logs(),
        ))
    }

    async fn get_task_id(&self, id: DatabaseId) -> anyhow::Result<HttpResponse<Task>> {
        db_get_object::<Task>(&self.db, id)
    }

    async fn delete_task_id(&self, id: DatabaseId) -> anyhow::Result<HttpResponse<()>> {
        db_delete_object::<Task>(&self.db, id)
    }

    async fn post_task_id(&self, id: DatabaseId, input: Task) -> anyhow::Result<HttpResponse<()>> {
        if let Err(e) = sauropod_task::validate_task(&input) {
            tracing::error!("Invalid task definition: {:#?}", e);
            return Ok(HttpResponse::BadRequest(e.to_string()));
        }

        db_update_object::<Task>(&self.db, id, input)
    }

    async fn post_task_id_run(
        &self,
        id: DatabaseId,
        input: serde_json::Map<String, serde_json::Value>,
    ) -> anyhow::Result<HttpResponse<serde_json::Map<String, serde_json::Value>>> {
        let task = match self.db.get_by_id::<Task>(id)? {
            Some(task) => sauropod_task::task_from_schema(task)?,
            None => {
                tracing::error!("Task not found: {id}");
                return Ok(HttpResponse::NotFound(None));
            }
        };

        let context = self.make_task_context().await?;
        let result = task.execute(serde_json::to_value(input)?, context).await?;
        let Some(result_map) = result.as_object() else {
            anyhow::bail!("Task result wasn't an object, was {:#?}", result);
        };
        Ok(HttpResponse::Ok(result_map.clone()))
    }

    async fn get_task_id_schema(
        &self,
        id: i64,
    ) -> anyhow::Result<HttpResponse<InputAndOutputSchema>> {
        let Some(task) = self.db.get_by_id::<Task>(id)? else {
            return Ok(HttpResponse::NotFound(None));
        };

        let input_schema = match sauropod_task::input_schema_from_task_schema(&task)? {
            serde_json::Value::Object(obj) => obj,
            x => {
                tracing::error!("Generated schema wasn't an object: {:#?}", x);
                anyhow::bail!("Couldn't generated schema");
            }
        };
        Ok(HttpResponse::Ok(InputAndOutputSchema {
            input_schema,
            output_schema: sauropod_task::output_schema_from_task_schema(&task)?,
        }))
    }

    async fn post_task(&self, input: Task) -> anyhow::Result<HttpResponse<DatabaseId>> {
        if let Err(e) = sauropod_task::validate_task(&input) {
            tracing::error!("Invalid task definition: {:#?}", e);
            return Ok(HttpResponse::BadRequest(e.to_string()));
        }

        db_create_object::<Task>(&self.db, input)
    }

    async fn get_task(&self) -> anyhow::Result<HttpResponse<Vec<ObjectInfo>>> {
        db_list_objects::<Task>(&self.db, None)
    }

    async fn get_tools(
        &self,
    ) -> anyhow::Result<HttpResponse<std::vec::Vec<sauropod_schemas::ToolDefinition>>> {
        Ok(HttpResponse::Ok(
            self.tools
                .iter()
                .map(|tool| tool.get_definition())
                .collect(),
        ))
    }

    async fn get_models(
        &self,
    ) -> anyhow::Result<HttpResponse<std::vec::Vec<sauropod_schemas::ModelDefinition>>> {
        Ok(HttpResponse::Ok(self.llm_engine.list_models().await?))
    }

    async fn get_version(&self) -> anyhow::Result<HttpResponse<String>> {
        Ok(HttpResponse::Ok(env!("CARGO_PKG_VERSION").to_string()))
    }
}
