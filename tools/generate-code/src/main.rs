use sauropod_schemas::{
    InputAndOutputSchema, ModelDefinition, ToolDefinition,
    task::{Task, TaskInfo},
};

use generate_code::{Object, openapi};

fn main() -> anyhow::Result<()> {
    openapi! {
        route "/health" auth(false) (
            GET (()) -> sauropod_schemas::HealthCheckResponse : "Get the health of the system"
        )
        route "/observability/logs" auth(true) (
            GET (()) -> sauropod_schemas::observability::LogResponse : "Get the logs from the system"
        )
        route "/task/{id:i64}" auth(true) (
            GET (()) -> Task : "Get a task by ID"
            DELETE (()) -> () : "Delete a task"
            POST (Task) -> () : "Update a task"
        )
        route "/task/{id:i64}/run" auth(true) (
            POST (Object) -> Object : "Run a task by ID"
        )
        route "/task/{id:i64}/schema" auth(true) (
            GET (()) -> InputAndOutputSchema : "Get the input and output JSON Schemas for a task"
        )
        route "/task" auth(true) (
            POST (Task) -> i64 : "Create a task"
            GET (()) -> Vec<TaskInfo> : "Get the list of tasks"
        )
        route "/tools" auth(true) (
            GET (()) -> Vec<ToolDefinition> : "Get the list of available tools"
        )
        route "/models" auth(true) (
            GET (()) -> Vec<ModelDefinition> : "Get the list of available models"
        )
        route "/version" auth(false) (
            GET (()) -> String : "Get the version of the server"
        )
    };

    generate_code::generate_code_for_config()?;
    generate_code::generate_code_for_docker()?;

    Ok(())
}
