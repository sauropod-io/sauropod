//! Sauropod workflow execution.

use std::collections::{HashMap, HashSet};

use tracing::Instrument as _;

use sauropod_task::{Task, TaskArc, task_from_schema};

enum Dependency {
    /// A task that depends on another task.
    Task {
        /// The ID of the task that is the dependencytask.
        input_id: String,
        /// The output field
        input_field: String,
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

/// Parse a path like "task_id.output" into a task ID "task_id" and field name "output".
fn parse_task_and_field(path: &str) -> (&str, &str) {
    match path.split_once('.') {
        Some((task_id, field)) => (task_id, field),
        None => (path, ""),
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
    /// The input schema for the workflow.
    input_schema: serde_json::Value,
    /// The output schema for the workflow.
    output_schema: serde_json::Value,
}

impl Workflow {
    /// Create a workflow from the schema representation.
    pub async fn from_schema(
        schema_workflow: sauropod_schemas::workflow::Workflow,
    ) -> anyhow::Result<Self> {
        // Initialize all the tasks
        let task_futures = schema_workflow
            .tasks
            .into_iter()
            .map(|(id, task_schema)| async {
                match task_from_schema(task_schema).await {
                    Ok(task) => Ok((id, task)),
                    Err(e) => Err(e),
                }
            })
            .collect::<Vec<_>>();
        let tasks: Vec<(String, TaskArc)> = futures::future::try_join_all(task_futures).await?;

        // Create the task map
        let mut task_data_dependencies =
            HashMap::<String, Vec<Dependency>>::with_capacity(tasks.len());
        let mut task_map: HashMap<String, TaskArc> = HashMap::new();
        for (id, task) in tasks {
            task_map.insert(id, task);
        }

        // Initialize dependency map with empty sets for all tasks
        let mut task_dependency_map = HashMap::new();
        for id in task_map.keys() {
            task_dependency_map.insert(id.clone(), HashSet::with_capacity(1));
            task_data_dependencies.insert(id.clone(), Vec::with_capacity(1));
        }

        // Process connections to build the dependency graph
        for connection in &schema_workflow.connections {
            match connection {
                sauropod_schemas::workflow::Connection::Parameter { parameter, to } => {
                    // Parameters create a data dependency, but not a task dependency
                    for task_path in to {
                        let (task, field) = parse_task_and_field(task_path);
                        task_data_dependencies
                            .entry(task.to_string())
                            .or_default()
                            .push(Dependency::Parameter {
                                parameter_name: parameter.clone(),
                                field: field.to_string(),
                            });
                    }
                }
                sauropod_schemas::workflow::Connection::Task { from, to } => {
                    // Parse the task ID from "task_id.output" format
                    let (from_task_id, from_field_path) = parse_task_and_field(from);

                    // Add dependencies: each "to" task depends on the "from" task
                    for to_path in to {
                        let (to_task_id, to_field_path) = parse_task_and_field(to_path);

                        task_dependency_map
                            .entry(to_task_id.to_string())
                            .or_default()
                            .insert(from_task_id.to_string());
                        task_data_dependencies
                            .entry(to_task_id.to_string())
                            .or_default()
                            .push(Dependency::Task {
                                input_id: from_task_id.to_string(),
                                input_field: from_field_path.to_string(),
                                field: to_field_path.to_string(),
                            });
                    }
                }
            }
        }

        // Process inputs for workflow parameters
        let inputs = schema_workflow
            .connections
            .into_iter()
            .flat_map(|x| {
                if let sauropod_schemas::workflow::Connection::Parameter { parameter, .. } = x {
                    Some(parameter)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut properties = HashMap::with_capacity(inputs.len());
        for input in inputs.iter() {
            properties.insert(input.clone(), serde_json::json!({ "type": "string" }));
        }

        let input_schema = serde_json::json!({
            "type": "object",
            "properties": properties,
            "required": inputs,
        });

        Ok(Self {
            task_map,
            task_data_dependencies,
            task_dependency_map,
            input_schema,
            // For now, just return a string.
            output_schema: serde_json::json!({
                "type": "string"
            }),
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
                            Dependency::Task {
                                input_id,
                                input_field,
                                field,
                            } => {
                                // Get the output from the previously executed task
                                if let Some(source_output) = task_outputs.get(input_id) {
                                    let source_value = if input_field.is_empty() {
                                        // If no field specified, use the entire output
                                        source_output.clone()
                                    } else {
                                        // Otherwise get the specific field
                                        source_output
                                            .get(input_field)
                                            .cloned()
                                            .unwrap_or(serde_json::Value::Null)
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

        // For now, return the output of the last executed task
        // In the future, we'll determine which task is the final output based on the workflow definition
        if let Some((_id, output)) = task_outputs.iter().next() {
            Ok(output.clone())
        } else {
            Ok(serde_json::json!({}))
        }
    }

    fn input_schema(&self) -> &serde_json::Value {
        &self.input_schema
    }

    fn output_schema(&self) -> &serde_json::Value {
        &self.output_schema
    }
}
