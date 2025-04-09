//! Generated code.
use tracing::Instrument as _;
pub static API_PREFIX: &str = "/api";
#[async_trait::async_trait]
pub trait ServerInterface {
    /// Get the health of the system
    async fn get_health(
        &self,
    ) -> anyhow::Result<crate::HttpResponse<sauropod_schemas::HealthCheckResponse>>;
    /// Get the logs from the system
    async fn get_observability_logs(
        &self,
    ) -> anyhow::Result<crate::HttpResponse<sauropod_schemas::observability::LogResponse>>;
    /// Get a task by ID
    async fn get_task_id(
        &self,
        id: i64,
    ) -> anyhow::Result<crate::HttpResponse<sauropod_schemas::task::Task>>;
    /// Update a task
    async fn post_task_id(
        &self,
        id: i64,
        input: sauropod_schemas::task::Task,
    ) -> anyhow::Result<crate::HttpResponse<()>>;
    /// Delete a task
    async fn delete_task_id(&self, id: i64) -> anyhow::Result<crate::HttpResponse<()>>;
    /// Run a task by ID
    async fn post_task_id_run(
        &self,
        id: i64,
        input: serde_json::map::Map<std::string::String, serde_json::value::Value>,
    ) -> anyhow::Result<
        crate::HttpResponse<serde_json::map::Map<std::string::String, serde_json::value::Value>>,
    >;
    /// Get the input and output JSON Schemas for a task
    async fn get_task_id_schema(
        &self,
        id: i64,
    ) -> anyhow::Result<crate::HttpResponse<sauropod_schemas::InputAndOutputSchema>>;
    /// Get the list of tasks
    async fn get_task(
        &self,
    ) -> anyhow::Result<crate::HttpResponse<std::vec::Vec<sauropod_schemas::workflow::ObjectInfo>>>;
    /// Create a task
    async fn post_task(
        &self,
        input: sauropod_schemas::task::Task,
    ) -> anyhow::Result<crate::HttpResponse<i64>>;
    /// Get a workflow by ID
    async fn get_workflow_id(
        &self,
        id: i64,
    ) -> anyhow::Result<crate::HttpResponse<sauropod_schemas::workflow::Workflow>>;
    /// Update a workflow
    async fn post_workflow_id(
        &self,
        id: i64,
        input: sauropod_schemas::workflow::Workflow,
    ) -> anyhow::Result<crate::HttpResponse<()>>;
    /// Delete a workflow
    async fn delete_workflow_id(&self, id: i64) -> anyhow::Result<crate::HttpResponse<()>>;
    /// Get the list of workflows
    async fn get_workflow(
        &self,
    ) -> anyhow::Result<crate::HttpResponse<std::vec::Vec<sauropod_schemas::workflow::ObjectInfo>>>;
    /// Create a workflow
    async fn post_workflow(
        &self,
        input: sauropod_schemas::workflow::Workflow,
    ) -> anyhow::Result<crate::HttpResponse<i64>>;
    /// Run a workflow by ID
    async fn post_workflow_id_run(
        &self,
        id: i64,
        input: serde_json::map::Map<std::string::String, serde_json::value::Value>,
    ) -> anyhow::Result<
        crate::HttpResponse<serde_json::map::Map<std::string::String, serde_json::value::Value>>,
    >;
    /// Get the input and output JSON Schemas for a workflow
    async fn get_workflow_id_schema(
        &self,
        id: i64,
    ) -> anyhow::Result<crate::HttpResponse<sauropod_schemas::InputAndOutputSchema>>;
    /// Get the list of available tools
    async fn get_tools(
        &self,
    ) -> anyhow::Result<crate::HttpResponse<std::vec::Vec<sauropod_schemas::ToolDefinition>>>;
    /// Get the list of available models
    async fn get_models(
        &self,
    ) -> anyhow::Result<crate::HttpResponse<std::vec::Vec<sauropod_schemas::ModelDefinition>>>;
    /// Get the version of the server
    async fn get_version(&self) -> anyhow::Result<crate::HttpResponse<std::string::String>>;
}

pub fn register_routes<T: ServerInterface + Sync + Send + 'static>(
    server: std::sync::Arc<T>,
) -> axum::Router<()> {
    axum::Router::new()
        .without_v07_checks()
        .route(
            "/health",
            axum::routing::get({
                let server_clone = server.clone();
                async move || {
                    tracing::debug!("GET /health");
                    let response = server_clone
                        .get_health()
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/health"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/observability/logs",
            axum::routing::get({
                let server_clone = server.clone();
                async move || {
                    tracing::debug!("GET /observability/logs");
                    let response = server_clone
                        .get_observability_logs()
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/observability/logs"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/task/{id}",
            axum::routing::get({
                let server_clone = server.clone();
                async move |axum::extract::Path(id): axum::extract::Path<i64>| {
                    tracing::debug!("GET /task/{{id}}");
                    let response = server_clone
                        .get_task_id(id)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/task/{{id}}"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            })
            .post({
                let server_clone = server.clone();
                async move |axum::extract::Path(id): axum::extract::Path<i64>,
                            axum::extract::Json(input): axum::extract::Json<
                    sauropod_schemas::task::Task,
                >| {
                    tracing::debug!("POST /task/{{id}}");
                    let response = server_clone
                        .post_task_id(id, input)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "POST",
                            path = "/task/{{id}}"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            })
            .delete({
                let server_clone = server.clone();
                async move |axum::extract::Path(id): axum::extract::Path<i64>| {
                    tracing::debug!("DELETE /task/{{id}}");
                    let response = server_clone
                        .delete_task_id(id)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "DELETE",
                            path = "/task/{{id}}"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/task/{id}/run",
            axum::routing::post({
                let server_clone = server.clone();
                async move |axum::extract::Path(id): axum::extract::Path<i64>,
                            axum::extract::Json(input): axum::extract::Json<
                    serde_json::map::Map<std::string::String, serde_json::value::Value>,
                >| {
                    tracing::debug!("POST /task/{{id}}/run");
                    let response = server_clone
                        .post_task_id_run(id, input)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "POST",
                            path = "/task/{{id}}/run"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/task/{id}/schema",
            axum::routing::get({
                let server_clone = server.clone();
                async move |axum::extract::Path(id): axum::extract::Path<i64>| {
                    tracing::debug!("GET /task/{{id}}/schema");
                    let response = server_clone
                        .get_task_id_schema(id)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/task/{{id}}/schema"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/task",
            axum::routing::get({
                let server_clone = server.clone();
                async move || {
                    tracing::debug!("GET /task");
                    let response = server_clone
                        .get_task()
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/task"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            })
            .post({
                let server_clone = server.clone();
                async move |axum::extract::Json(input): axum::extract::Json<
                    sauropod_schemas::task::Task,
                >| {
                    tracing::debug!("POST /task");
                    let response = server_clone
                        .post_task(input)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "POST",
                            path = "/task"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/workflow/{id}",
            axum::routing::get({
                let server_clone = server.clone();
                async move |axum::extract::Path(id): axum::extract::Path<i64>| {
                    tracing::debug!("GET /workflow/{{id}}");
                    let response = server_clone
                        .get_workflow_id(id)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/workflow/{{id}}"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            })
            .post({
                let server_clone = server.clone();
                async move |axum::extract::Path(id): axum::extract::Path<i64>,
                            axum::extract::Json(input): axum::extract::Json<
                    sauropod_schemas::workflow::Workflow,
                >| {
                    tracing::debug!("POST /workflow/{{id}}");
                    let response = server_clone
                        .post_workflow_id(id, input)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "POST",
                            path = "/workflow/{{id}}"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            })
            .delete({
                let server_clone = server.clone();
                async move |axum::extract::Path(id): axum::extract::Path<i64>| {
                    tracing::debug!("DELETE /workflow/{{id}}");
                    let response = server_clone
                        .delete_workflow_id(id)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "DELETE",
                            path = "/workflow/{{id}}"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/workflow",
            axum::routing::get({
                let server_clone = server.clone();
                async move || {
                    tracing::debug!("GET /workflow");
                    let response = server_clone
                        .get_workflow()
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/workflow"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            })
            .post({
                let server_clone = server.clone();
                async move |axum::extract::Json(input): axum::extract::Json<
                    sauropod_schemas::workflow::Workflow,
                >| {
                    tracing::debug!("POST /workflow");
                    let response = server_clone
                        .post_workflow(input)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "POST",
                            path = "/workflow"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/workflow/{id}/run",
            axum::routing::post({
                let server_clone = server.clone();
                async move |axum::extract::Path(id): axum::extract::Path<i64>,
                            axum::extract::Json(input): axum::extract::Json<
                    serde_json::map::Map<std::string::String, serde_json::value::Value>,
                >| {
                    tracing::debug!("POST /workflow/{{id}}/run");
                    let response = server_clone
                        .post_workflow_id_run(id, input)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "POST",
                            path = "/workflow/{{id}}/run"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/workflow/{id}/schema",
            axum::routing::get({
                let server_clone = server.clone();
                async move |axum::extract::Path(id): axum::extract::Path<i64>| {
                    tracing::debug!("GET /workflow/{{id}}/schema");
                    let response = server_clone
                        .get_workflow_id_schema(id)
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/workflow/{{id}}/schema"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/tools",
            axum::routing::get({
                let server_clone = server.clone();
                async move || {
                    tracing::debug!("GET /tools");
                    let response = server_clone
                        .get_tools()
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/tools"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/models",
            axum::routing::get({
                let server_clone = server.clone();
                async move || {
                    tracing::debug!("GET /models");
                    let response = server_clone
                        .get_models()
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/models"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
        .route(
            "/version",
            axum::routing::get({
                let server_clone = server.clone();
                async move || {
                    tracing::debug!("GET /version");
                    let response = server_clone
                        .get_version()
                        .instrument(tracing::info_span!(
                            "Request",
                            method = "GET",
                            path = "/version"
                        ))
                        .await;
                    if let Err(error) = &response {
                        tracing::error!("Error responding to request: {:?}", error);
                    }
                    crate::create_response_from_result(response)
                }
            }),
        )
}
