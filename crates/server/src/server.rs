//! HTTP server code.

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use sauropod_config::ModelConfig;
use sauropod_database::{DatabaseId, DatabaseTypeWithID, DatabaseTypeWithName};
use sauropod_schemas::task::{ModelStrength, Task, TaskId};
use sauropod_schemas::workflow::{ObjectInfo, Workflow};
use tracing::Instrument;

use sauropod_http::HttpResponse;
use sauropod_task::Task as _;

use crate::observability::Observability;

/// The server contains the main state of the application.
pub struct Server {
    /// The server configuration.
    _config: sauropod_config::Config,
    /// Observability state.
    observability: Observability,
    /// The database.
    db: sauropod_database::Database,
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
        Ok(std::sync::Arc::new(Self {
            _config: config.clone(),
            observability: Observability { log_buffer },
            db,
            tools: sauropod_task_context::get_default_tools(),
            llm_engine: sauropod_llm_inference::create_engine(config).await?,
        }))
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
        Some(workflow) => Ok(workflow.into()),
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

/// Build a model selection map.
async fn make_model_selection(
    llm_engine: &sauropod_llm_inference::EnginePointer,
    config: &sauropod_config::Config,
) -> anyhow::Result<BTreeMap<ModelStrength, ModelConfig>> {
    let mut model_names = config.models.to_map();

    if model_names.is_empty() {
        let llm_engine_models = llm_engine.list_models().await?;
        if llm_engine_models.len() == 1 {
            model_names.insert(
                ModelStrength::Strong,
                ModelConfig {
                    model: llm_engine_models[0].name.clone(),
                    ..ModelConfig::default()
                },
            );
        }
    }

    Ok(model_names)
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

    async fn get_workflow_id(&self, id: DatabaseId) -> anyhow::Result<HttpResponse<Workflow>> {
        db_get_object::<Workflow>(&self.db, id)
    }

    async fn delete_workflow_id(&self, id: DatabaseId) -> anyhow::Result<HttpResponse<()>> {
        db_delete_object::<Workflow>(&self.db, id)
    }

    async fn post_workflow_id(
        &self,
        id: DatabaseId,
        input: Workflow,
    ) -> anyhow::Result<HttpResponse<()>> {
        if let Err(e) = sauropod_workflows::validate_workflow(&input) {
            tracing::error!("Invalid workflow definition: {:#?}", e);
            return Ok(HttpResponse::BadRequest(e.to_string()));
        }

        db_update_object::<Workflow>(&self.db, id, input)
    }

    async fn post_workflow(&self, input: Workflow) -> anyhow::Result<HttpResponse<DatabaseId>> {
        if let Err(e) = sauropod_workflows::validate_workflow(&input) {
            tracing::error!("Invalid workflow definition: {:#?}", e);
            return Ok(HttpResponse::BadRequest(e.to_string()));
        }

        db_create_object::<Workflow>(&self.db, input)
    }

    async fn get_workflow(&self) -> anyhow::Result<HttpResponse<Vec<ObjectInfo>>> {
        db_list_objects::<Workflow>(&self.db, None)
    }

    async fn post_workflow_id_invoke(
        &self,
        id: DatabaseId,
        input: serde_json::Map<String, serde_json::Value>,
    ) -> anyhow::Result<HttpResponse<serde_json::Map<String, serde_json::Value>>> {
        let workflow = match self.db.get_by_id::<Workflow>(id)? {
            Some(workflow) => workflow,
            None => {
                tracing::error!("Workflow not found: {id}");
                return Ok(HttpResponse::NotFound(None));
            }
        };

        let mut task_schema_map: HashMap<TaskId, sauropod_schemas::task::Task> =
            HashMap::with_capacity(8);
        for task in workflow.actions.values() {
            match task {
                sauropod_schemas::workflow::WorkflowAction::RunTask(task_id) => {
                    // Tasks may appear in the action list multiple times
                    if task_schema_map.contains_key(task_id) {
                        continue;
                    }

                    // Get the task from the database and populate the map
                    if let Some(task) = self.db.get_by_id(task_id.task_id)? {
                        task_schema_map.insert(*task_id, task);
                    }
                }
                x => {
                    tracing::error!("Unsupported action: {:#?}", x);
                    anyhow::bail!("Unsupported action: {:#?}", x);
                }
            }
        }

        let workflow = sauropod_workflows::Workflow::from_schema(workflow, &task_schema_map)
            .instrument(tracing::info_span!("loading workflow"))
            .await?;

        let model_names = make_model_selection(&self.llm_engine, &self._config).await?;

        // Ensure that the models are available
        let available_models = self.llm_engine.list_models().await?;
        for model in model_names.values() {
            let has_model = available_models.iter().any(|m| m.name == model.model);

            if !has_model {
                if self.llm_engine.can_pull_model() {
                    self.llm_engine.pull_model(&model.model).await?;
                } else {
                    tracing::error!("Model {} not available", model.model);
                }
            }
        }

        let context =
            sauropod_task_context::make_default_task_context(self.llm_engine.clone(), model_names);
        let mut result = serde_json::Map::<String, serde_json::Value>::with_capacity(1);
        result.insert(
            "result".to_string(),
            workflow
                .execute(serde_json::to_value(input)?, context)
                .await?,
        );
        Ok(HttpResponse::Ok(result))
    }

    async fn get_workflow_id_input_schema(
        &self,
        id: i64,
    ) -> anyhow::Result<
        HttpResponse<serde_json::map::Map<std::string::String, serde_json::value::Value>>,
    > {
        let workflow = match self.db.get_by_id::<Workflow>(id)? {
            Some(workflow) => workflow,
            None => {
                return Ok(HttpResponse::NotFound(None));
            }
        };

        Ok(HttpResponse::Ok(
            sauropod_workflows::input_schema_from_workflow_schema(&workflow),
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

    async fn get_task_id_input_schema(
        &self,
        id: i64,
    ) -> anyhow::Result<
        HttpResponse<serde_json::map::Map<std::string::String, serde_json::value::Value>>,
    > {
        let Some(task) = self.db.get_by_id::<Task>(id)? else {
            return Ok(HttpResponse::NotFound(None));
        };

        match sauropod_task::input_schema_from_task_schema(&task)? {
            serde_json::Value::Object(obj) => Ok(HttpResponse::Ok(obj)),
            x => {
                tracing::error!("Generated schema wasn't an object: {:#?}", x);
                anyhow::bail!("Couldn't generated schema");
            }
        }
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
}
