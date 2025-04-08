use sauropod_schemas::{
    InputAndOutputSchema, ModelDefinition, ToolDefinition,
    task::Task,
    workflow::{ObjectInfo, Workflow},
};

use generate_code_from_structs::{Object, database_types, json_schema::write_schema, openapi};

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
        route "/workflow/{id:i64}" (
            GET (()) -> Workflow : "Get a workflow by ID"
            DELETE (()) -> () : "Delete a workflow"
            POST (Workflow) -> () : "Update a workflow"
        )
        route "/workflow" (
            POST (Workflow) -> i64 : "Create a workflow"
            GET (()) -> Vec<ObjectInfo> : "Get the list of workflows"
        )
        route "/workflow/{id:i64}/run" (
            POST (Object) -> Object : "Run a workflow by ID"
        )
        route "/workflow/{id:i64}/schema" (
            GET (()) -> InputAndOutputSchema : "Get the input and output JSON Schemas for a workflow"
        )
        route "/tools" (
            GET (()) -> Vec<ToolDefinition> : "Get the list of available tools"
        )
        route "/models" (
            GET (()) -> Vec<ModelDefinition> : "Get the list of available models"
        )
    };

    database_types! {
        Workflow,
        Task
    };

    write_schema::<Workflow>()?;
    write_schema::<Task>()?;

    generate_code_from_structs::generate_code_for_config()?;

    Ok(())
}
