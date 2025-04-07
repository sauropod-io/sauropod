//! Sauropod workflow execution.

use std::collections::{HashMap, HashSet};

use sauropod_schemas::task::TaskId;
use tracing::Instrument as _;

use sauropod_task::{Task, TaskArc, task_from_schema};

/// A field in a task output.
struct TaskField {
    /// The ID of the task.
    id: String,
    /// The field name.
    field: Option<String>,
}

/// An output mapping from a task to the workflow output.
struct OutputMapping {
    /// The task field that provides the output data.
    from: TaskField,
    /// The name of the output in the workflow result.
    output_name: String,
}

enum Dependency {
    /// A task that depends on another task.
    Task {
        /// The task field that provides the input to the current task.
        input_data: TaskField,
        /// The field to map the data to.
        field: String,
    },
    /// A parameter that is passed to a task.
    Parameter {
        /// The name of the parameter.
        parameter_name: String,
        /// The field to map the parameter to.
        field: String,
    },
}

/// Check whether a workflow is valid.
pub fn validate_workflow(workflow: &sauropod_schemas::workflow::Workflow) -> anyhow::Result<()> {
    // Validate workflow has a name
    if workflow.name.trim().is_empty() {
        return Err(anyhow::anyhow!("Workflow must have a name"));
    }

    let task_ids: HashSet<String> = workflow.actions.keys().cloned().collect();
    // Track output names to check for duplicates
    let mut output_names = HashSet::new();

    // Validate all connections
    for connection in &workflow.connections {
        match connection {
            sauropod_schemas::workflow::Connection::Task { from, to } => {
                // Validate "from" task exists
                let (from_task_id, _) = parse_task_and_field(from);
                if !task_ids.contains(from_task_id) {
                    return Err(anyhow::anyhow!(
                        "Task '{from_task_id}' referenced in connection does not exist"
                    ));
                }

                // Validate all "to" tasks exist
                let (to_task_id, to_field) = parse_task_and_field(to);
                if !task_ids.contains(to_task_id) {
                    return Err(anyhow::anyhow!(
                        "Task '{}' referenced in connection does not exist",
                        to_task_id
                    ));
                }

                // Check that to_field is specified
                if to_field.is_none() {
                    return Err(anyhow::anyhow!(
                        "Task path '{}' is missing a field, like '{}.field_name_here'",
                        to,
                        to_task_id
                    ));
                }
            }
            sauropod_schemas::workflow::Connection::Output { from, output } => {
                // Validate that referenced task exists
                let (from_task_id, _) = parse_task_and_field(from);
                if !task_ids.contains(from_task_id) {
                    return Err(anyhow::anyhow!(
                        "Task '{}' referenced in output connection does not exist",
                        from_task_id
                    ));
                }

                // Validate output name is not empty
                if output.trim().is_empty() {
                    return Err(anyhow::anyhow!("Output name cannot be empty"));
                }

                // Check for duplicate output names
                if !output_names.insert(output.clone()) {
                    return Err(anyhow::anyhow!("Duplicate output name: {}", output));
                }
            }
            sauropod_schemas::workflow::Connection::Parameter { parameter, to } => {
                // Validate parameter name is not empty
                if parameter.trim().is_empty() {
                    return Err(anyhow::anyhow!("Parameter name cannot be empty"));
                }

                // Validate all "to" tasks exist
                let (to_task_id, to_field) = parse_task_and_field(to);
                if !task_ids.contains(to_task_id) {
                    return Err(anyhow::anyhow!(
                        "Task '{}' referenced in parameter connection does not exist",
                        to_task_id
                    ));
                }

                // Check that to_field is specified
                if to_field.is_none() {
                    return Err(anyhow::anyhow!(
                        "Task path '{}' is missing a field, like '{}.field_name_here'",
                        to,
                        to_task_id
                    ));
                }
            }
        }
    }

    Ok(())
}

/// Parse a path like "task_id.output" into a task ID "task_id" and field name "output".
fn parse_task_and_field(path: &str) -> (&str, Option<&str>) {
    match path.split_once('.') {
        Some((task_id, field)) => (task_id, Some(field)),
        None => (path, None),
    }
}

/// A loaded workflow.
pub struct Workflow {
    /// The tasks in the workflow mapped by task ID
    task_map: HashMap<String, TaskArc>,
    /// The dependency map (key depends on values)
    task_dependency_map: HashMap<String, HashSet<String>>,
    /// The data dependency mapping.
    task_data_dependencies: HashMap<String, Vec<Dependency>>,
    /// The output mappings from tasks to workflow outputs.
    output_mappings: Vec<OutputMapping>,
    /// The input schema for the workflow.
    input_schema: serde_json::Value,
    /// The output schema for the workflow.
    output_schema: serde_json::Value,
}

impl Workflow {
    /// Create a workflow from the schema representation.
    pub async fn from_schema(
        schema_workflow: sauropod_schemas::workflow::Workflow,
        task_schema_map: &HashMap<TaskId, sauropod_schemas::task::Task>,
    ) -> anyhow::Result<Self> {
        // Initialize all the tasks
        let mut task_map: HashMap<String, TaskArc> =
            HashMap::with_capacity(schema_workflow.actions.len());
        for (id, action) in &schema_workflow.actions {
            match action {
                sauropod_schemas::workflow::WorkflowAction::RunTask(task_id) => {
                    // Get the task schema
                    let task_schema = task_schema_map
                        .get(task_id)
                        .ok_or_else(|| anyhow::anyhow!("Task not found: {}", task_id.task_id))?;

                    // Create the task from the schema representation
                    let task = task_from_schema(task_schema.clone())?;
                    task_map.insert(id.clone(), task);
                }
                unsupported => {
                    tracing::warn!("Unsupported action in workflow: {:?}", unsupported);
                    anyhow::bail!("Unsupported action in workflow: {:?}", unsupported);
                }
            }
        }

        // Create the task map
        let mut task_data_dependencies =
            HashMap::<String, Vec<Dependency>>::with_capacity(task_map.len());

        // Initialize dependency map with empty sets for all tasks
        let mut task_dependency_map = HashMap::new();
        for id in task_map.keys() {
            task_dependency_map.insert(id.clone(), HashSet::with_capacity(1));
            task_data_dependencies.insert(id.clone(), Vec::with_capacity(1));
        }

        // Track workflow outputs
        let mut output_mappings = Vec::new();

        // Process connections to build the dependency graph
        for connection in &schema_workflow.connections {
            match connection {
                sauropod_schemas::workflow::Connection::Parameter { parameter, to } => {
                    // Parameters create a data dependency, but not a task dependency
                    let (task, Some(field)) = parse_task_and_field(to) else {
                        anyhow::bail!("Invalid task path: {}", to);
                    };

                    task_data_dependencies
                        .entry(task.to_string())
                        .or_default()
                        .push(Dependency::Parameter {
                            parameter_name: parameter.clone(),
                            field: field.to_string(),
                        });
                }
                sauropod_schemas::workflow::Connection::Output { from, output } => {
                    // Parse the task ID from "task_id.output" format
                    let (from_task_id, from_field_path) = parse_task_and_field(from);

                    // Add to output mappings
                    output_mappings.push(OutputMapping {
                        from: TaskField {
                            id: from_task_id.to_string(),
                            field: from_field_path.map(|x| x.to_string()),
                        },
                        output_name: output.clone(),
                    });
                }
                sauropod_schemas::workflow::Connection::Task { from, to } => {
                    // Parse the task ID from "task_id.output" format
                    let (from_task_id, from_field_path) = parse_task_and_field(from);
                    let (to_task_id, Some(to_field_path)) = parse_task_and_field(to) else {
                        anyhow::bail!(
                            "Task path '{0}' is missing a field, like '{0}.field_name_here'",
                            to
                        );
                    };

                    task_dependency_map
                        .entry(to_task_id.to_string())
                        .or_default()
                        .insert(from_task_id.to_string());
                    task_data_dependencies
                        .entry(to_task_id.to_string())
                        .or_default()
                        .push(Dependency::Task {
                            input_data: TaskField {
                                id: from_task_id.to_string(),
                                field: from_field_path.map(|x| x.to_string()),
                            },
                            field: to_field_path.to_string(),
                        });
                }
            }
        }

        // Generate output schema based on output mappings
        let mut properties = serde_json::Map::new();
        for mapping in &output_mappings {
            let Some(task) = task_map.get(&mapping.from.id) else {
                anyhow::bail!(
                    "Task {} (connected to {}) not found",
                    mapping.from.id,
                    mapping.output_name
                );
            };

            let task_schema = task.output_schema();
            let output_field_schema = if let Some(field) = &mapping.from.field {
                if task_schema["type"] != "object" {
                    anyhow::bail!(
                        "Task {} output (connected to {}) is a {} but needs to be an object instead",
                        mapping.from.id,
                        mapping.output_name,
                        task_schema["type"]
                    );
                }

                if let Some(field) = task_schema["properties"]
                    .as_object()
                    .and_then(|props| props.get(field))
                {
                    field.clone()
                } else {
                    anyhow::bail!(
                        "Task {} output (connected to {}) does not have field {}",
                        mapping.from.id,
                        mapping.output_name,
                        field
                    );
                }
            } else {
                task_schema.clone()
            };

            properties.insert(
                mapping.output_name.clone(),
                serde_json::json!({
                    "type": output_field_schema
                }),
            );
        }

        let output_schema = serde_json::json!({
            "type": "object",
            "properties": properties,
            "required": output_mappings.iter().map(|m| m.output_name.clone()).collect::<Vec<_>>()
        });

        Ok(Self {
            task_map,
            task_data_dependencies,
            task_dependency_map,
            output_mappings,
            input_schema: serde_json::json!(input_schema_from_workflow_schema(&schema_workflow)),
            output_schema,
        })
    }
}

#[async_trait::async_trait]
impl Task for Workflow {
    async fn execute(
        &self,
        workflow_parameters: serde_json::Value,
        context: std::sync::Arc<sauropod_task_context::TaskContext>,
    ) -> anyhow::Result<serde_json::Value> {
        tracing::info!(
            "Executing workflow task with input: {:?}",
            workflow_parameters
        );

        if let Err(validation_error) =
            jsonschema::validate(&self.input_schema, &workflow_parameters)
        {
            tracing::error!("Error running workflow: {:?}", &validation_error);
            anyhow::bail!("Error running workflow: {}", validation_error)
        }

        // Track which tasks have been executed
        let mut executed = HashSet::new();

        // Keep track of outputs for each task
        let mut task_outputs: HashMap<String, serde_json::Value> = HashMap::new();

        // Create a mutable copy of dependencies that we'll modify during execution
        let mut remaining_deps: HashMap<String, HashSet<String>> = self.task_dependency_map.clone();

        // Keep executing until all tasks are complete
        while executed.len() < self.task_map.len() {
            // Find tasks with no remaining dependencies
            let ready_tasks: Vec<String> = remaining_deps
                .iter()
                .filter(|(id, deps)| !executed.contains(*id) && deps.is_empty())
                .map(|(id, _)| id.clone())
                .collect();

            if ready_tasks.is_empty() && executed.len() < self.task_map.len() {
                return Err(anyhow::anyhow!("Circular dependency detected in workflow"));
            }

            // Execute all ready tasks (TODO in the future run each generation in parallel)
            for task_id in ready_tasks {
                let task = self
                    .task_map
                    .get(&task_id)
                    .ok_or_else(|| anyhow::anyhow!("Task not found: {}", task_id))?;

                // Prepare the input for the task
                let mut task_input = serde_json::json!({});

                // Populate task input based on dependencies
                if let Some(dependencies) = self.task_data_dependencies.get(&task_id) {
                    for dependency in dependencies {
                        match dependency {
                            Dependency::Parameter {
                                parameter_name,
                                field,
                            } => {
                                if let Some(param_value) = workflow_parameters.get(parameter_name) {
                                    // Set the parameter value in the appropriate field
                                    if field.is_empty() {
                                        // If no field specified, use the parameter directly
                                        task_input = param_value.clone();
                                    } else {
                                        // Otherwise set it as a field in the object
                                        task_input[field] = param_value.clone();
                                    }
                                }
                            }
                            Dependency::Task { input_data, field } => {
                                // Get the output from the previously executed task
                                if let Some(source_output) = task_outputs.get(&input_data.id) {
                                    let source_value = if let Some(field) = &input_data.field {
                                        // Otherwise get the specific field
                                        source_output
                                            .get(field)
                                            .cloned()
                                            .unwrap_or(serde_json::Value::Null)
                                    } else {
                                        // If no field specified, use the entire output
                                        source_output.clone()
                                    };

                                    // Set the value in the appropriate field
                                    if field.is_empty() {
                                        // If no target field, use the value directly (if it's an object)
                                        task_input = source_value;
                                    } else {
                                        // Otherwise set it as a field
                                        task_input[field] = source_value;
                                    }
                                }
                            }
                        }
                    }
                }

                tracing::info!("Executing task '{}' with input: {:?}", task_id, task_input);
                let task_result = task
                    .execute(task_input, context.clone())
                    .instrument(tracing::info_span!("Running task", task_id = &task_id))
                    .await?;

                // Store the task output
                task_outputs.insert(task_id.clone(), task_result);

                // Mark this task as executed
                executed.insert(task_id.clone());

                // Remove this task from all dependency lists
                for deps in remaining_deps.values_mut() {
                    deps.remove(&task_id);
                }
            }
        }

        // Construct output based on output mappings
        let mut result = serde_json::Map::new();

        for mapping in &self.output_mappings {
            if let Some(source_output) = task_outputs.get(&mapping.from.id) {
                let source_value = if let Some(field) = &mapping.from.field {
                    // Otherwise get the specific field
                    source_output
                        .get(field)
                        .cloned()
                        .unwrap_or(serde_json::Value::Null)
                } else {
                    // If no field specified, use the entire output
                    source_output.clone()
                };

                result.insert(mapping.output_name.clone(), source_value);
            } else {
                // Task not found or didn't produce output
                result.insert(mapping.output_name.clone(), serde_json::Value::Null);
            }
        }

        Ok(serde_json::Value::Object(result))
    }

    fn input_schema(&self) -> &serde_json::Value {
        &self.input_schema
    }

    fn output_schema(&self) -> &serde_json::Value {
        &self.output_schema
    }
}

/// Get the input schema for a workflow.
///
/// This function uses workflow's connections to determine the required inputs to invoke it.
pub fn input_schema_from_workflow_schema(
    schema_workflow: &sauropod_schemas::workflow::Workflow,
) -> serde_json::Map<String, serde_json::Value> {
    // Process inputs for workflow parameters
    let input_names = schema_workflow
        .connections
        .iter()
        .flat_map(|x| {
            if let sauropod_schemas::workflow::Connection::Parameter { parameter, .. } = x {
                Some(parameter)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut properties = HashMap::with_capacity(input_names.len());
    for input in input_names.iter() {
        properties.insert(input, serde_json::json!({ "type": "string" }));
    }

    let mut inputs = serde_json::Map::with_capacity(3);
    inputs.insert("type".to_string(), serde_json::json!("object"));
    inputs.insert("properties".to_string(), serde_json::json!(properties));
    inputs.insert("required".to_string(), serde_json::json!(input_names));
    inputs
}

/// Get the output schema for a workflow.
///
/// This function uses workflow's connections to determine the outputs it produces.
pub fn output_schema_from_workflow_schema(
    schema_workflow: &sauropod_schemas::workflow::Workflow,
) -> serde_json::Map<String, serde_json::Value> {
    // Process outputs for workflow parameters
    let output_names = schema_workflow
        .connections
        .iter()
        .flat_map(|x| {
            if let sauropod_schemas::workflow::Connection::Output { output, .. } = x {
                Some(output)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut properties = HashMap::with_capacity(output_names.len());
    for output in output_names.iter() {
        properties.insert(output, serde_json::json!({ "type": "string" }));
    }

    let mut outputs = serde_json::Map::with_capacity(3);
    outputs.insert("type".to_string(), serde_json::json!("object"));
    outputs.insert("properties".to_string(), serde_json::json!(properties));
    outputs.insert("required".to_string(), serde_json::json!(output_names));
    outputs
}
