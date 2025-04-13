use sauropod_schemas::{
    InputAndOutputSchema, ModelDefinition, ToolDefinition,
    task::{ObjectInfo, Task},
};

use generate_code::{Object, database_types, openapi};

fn main() -> anyhow::Result<()> {
    openapi! {
        route "/health" (
            GET (()) -> sauropod_schemas::HealthCheckResponse : "Get the health of the system"
        )
        route "/observability/logs" (
            GET (()) -> sauropod_schemas::observability::LogResponse : "Get the logs from the system"
        )
        route "/task/{id:i64}" (
            GET (()) -> Task : "Get a task by ID"
            DELETE (()) -> () : "Delete a task"
            POST (Task) -> () : "Update a task"
        )
        route "/task/{id:i64}/run" (
            POST (Object) -> Object : "Run a task by ID"
        )
        route "/task/{id:i64}/schema" (
            GET (()) -> InputAndOutputSchema : "Get the input and output JSON Schemas for a task"
        )
        route "/task" (
            POST (Task) -> i64 : "Create a task"
            GET (()) -> Vec<ObjectInfo> : "Get the list of tasks"
        )
        route "/tools" (
            GET (()) -> Vec<ToolDefinition> : "Get the list of available tools"
        )
        route "/models" (
            GET (()) -> Vec<ModelDefinition> : "Get the list of available models"
        )
        route "/version" (
            GET (()) -> String : "Get the version of the server"
        )
    };

    database_types! {
        Task
    };

    generate_code::generate_code_for_config()?;
    generate_code::generate_code_for_docker()?;

    Ok(())
}
