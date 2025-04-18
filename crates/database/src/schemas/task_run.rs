use std::str::FromStr as _;

use chrono::{DateTime, Utc};
use sauropod_schemas::observability::{RunStatus, Step, StepAction, TaskRun};
use sqlx::FromRow as _;
use sqlx::types::Json;

use crate::{Database, DatabaseId, DatabaseTypeWithId, UserId};

/// A run of a task or tool.
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct TaskStep {
    /// The ID of the step in the task execution.
    pub step_id: DatabaseId,
    /// The ID of the run.
    pub run_id: DatabaseId,
    /// The ID of user that owns the run.
    pub owner_id: DatabaseId,
    /// The ID of the parent step in the task execution.
    pub parent_step_id: Option<DatabaseId>,
    /// The inputs to the task or tool.
    pub inputs: Json<serde_json::Value>,
    /// The outputs from the task or tool.
    pub outputs: Json<serde_json::Value>,
    /// The ID of the task being run (if this is a task run).
    pub task_id: Option<DatabaseId>,
    /// The ID of the tool being run (if this is a tool run).
    pub tool_id: Option<String>,
    /// An error message if an error occurs.
    pub error: Option<String>,
    /// When the step started executing.
    pub start_time: Option<DateTime<Utc>>,
    /// When the step finished executing.
    pub end_time: Option<DateTime<Utc>>,
}

impl TaskStep {
    // Set the result of a successful task.
    pub async fn set_success(
        step_id: DatabaseId,
        output: serde_json::Value,
        connection: &Database,
    ) -> sqlx::Result<bool> {
        let current_time = chrono::Utc::now();
        let json_output = Json(output);

        sqlx::query!(
            r#"
            UPDATE task_run_steps
            SET outputs = ?, end_time = ?
            WHERE step_id = ?"#,
            json_output,
            current_time,
            step_id
        )
        .execute(connection)
        .await
        .map(|result| result.rows_affected() > 0)
    }

    // Set failure.
    pub async fn set_failure(
        step_id: DatabaseId,
        error: String,
        connection: &Database,
    ) -> sqlx::Result<bool> {
        let current_time = chrono::Utc::now();

        sqlx::query!(
            r#"
            UPDATE task_run_steps
            SET error = ?, end_time = ?
            WHERE step_id = ?"#,
            error,
            current_time,
            step_id
        )
        .execute(connection)
        .await
        .map(|result| result.rows_affected() > 0)
    }

    /// Insert the task run into the database.
    ///
    /// Note: Insertions ignore the step_id field, as it is auto-incremented.
    pub async fn insert(&self, connection: &Database) -> sqlx::Result<DatabaseId> {
        sqlx::query!(
            "INSERT INTO task_run_steps (run_id, owner_id, parent_step_id, inputs, outputs, task_id, tool_id, error, start_time, end_time)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            self.run_id,
            self.owner_id,
            self.parent_step_id,
            self.inputs,
            self.outputs,
            self.task_id,
            self.tool_id,
            self.error,
            self.start_time,
            self.end_time
        )
        .execute(connection)
        .await
        .map(|result| result.last_insert_rowid())
    }

    /// Get task runs for a specific task.
    ///
    /// Returns at most `limit` runs (default: 10).
    pub async fn get_for_task(
        task_id: DatabaseId,
        owner: UserId,
        connection: &Database,
        limit: Option<u32>,
    ) -> sqlx::Result<Vec<Self>> {
        // Use a query builder to work around issues with the query_as! macro
        let mut builder = sqlx::query_builder::QueryBuilder::new(
            "SELECT run_id, owner_id, step_id, parent_step_id, inputs, outputs, task_id, tool_id, error, start_time, end_time \
             FROM task_run_steps WHERE task_id = ",
        );
        builder.push_bind(task_id);
        builder.push(" AND owner_id = ");
        builder.push_bind(owner);

        // Add limit clause
        builder.push(" LIMIT ");
        builder.push_bind(limit.unwrap_or(10));

        builder
            .build()
            .fetch_all(connection)
            .await?
            .iter()
            .map(TaskStep::from_row)
            .collect::<sqlx::Result<Vec<TaskStep>>>()
    }

    /// Get all the steps in a run.
    pub async fn list_steps_by_run_id(
        run_id: DatabaseId,
        owner: UserId,
        connection: &Database,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            TaskStep,
            r#"
                SELECT run_id, owner_id, step_id as "step_id!", parent_step_id, inputs as "inputs: Json<serde_json::Value>",
                        outputs as "outputs: Json<serde_json::Value>", task_id, tool_id, error,
                        start_time as "start_time?: DateTime<Utc>", end_time as "end_time?: DateTime<Utc>"
                FROM task_run_steps
                WHERE run_id = ? AND owner_id = ?
            "#,
            run_id,
            owner
        )
        .fetch_all(connection)
        .await
    }
}

impl DatabaseTypeWithId for TaskStep {
    async fn get_by_id(
        id: DatabaseId,
        owner: UserId,
        connection: &Database,
    ) -> sqlx::Result<Option<Self>> {
        let result = sqlx::query_as!(
            TaskStep,
            r#"
                SELECT run_id, owner_id, step_id as "step_id!", parent_step_id, inputs as "inputs: Json<serde_json::Value>",
                       outputs as "outputs: Json<serde_json::Value>", task_id, tool_id, error,
                       start_time as "start_time?: DateTime<Utc>", end_time as "end_time?: DateTime<Utc>"
                FROM task_run_steps
                WHERE step_id = ? AND owner_id = ?
            "#,
            id,
            owner
        )
        .fetch_one(connection)
        .await;

        match result {
            Ok(run) => Ok(Some(run)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn delete_by_id(
        id: DatabaseId,
        owner: UserId,
        connection: &Database,
    ) -> sqlx::Result<bool> {
        let result = sqlx::query!(
            "DELETE FROM task_run_steps WHERE step_id = ? AND owner_id = ?",
            id,
            owner
        )
        .execute(connection)
        .await;

        match result {
            Ok(result) => Ok(result.rows_affected() > 0),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn list(owner: UserId, connection: &Database) -> sqlx::Result<Vec<Self>> {
        let mut builder = sqlx::query_builder::QueryBuilder::new(
            "SELECT run_id, owner_id, step_id, parent_step_id, inputs, outputs, task_id, tool_id, error, start_time, end_time \
            FROM task_run_steps WHERE owner_id = ",
        );
        builder.push_bind(owner);
        builder
            .build()
            .fetch_all(connection)
            .await?
            .iter()
            .map(TaskStep::from_row)
            .collect::<sqlx::Result<Vec<TaskStep>>>()
    }
}

/// Information about a task run.
#[derive(Debug, Clone, PartialEq)]
pub struct TaskRunRecord {
    /// The ID of the run.
    pub run_id: DatabaseId,
    /// The ID of the user that owns this run.
    pub owner_id: DatabaseId,
    /// The status of the run.
    pub status: String,
    /// The start of the run in milliseconds since UTC epoch.
    pub start_time: Option<DateTime<Utc>>,
    /// The end time of the run in milliseconds since UTC epoch.
    pub end_time: Option<DateTime<Utc>>,
}

impl TaskRunRecord {
    /// Create a new task run.
    pub async fn create(owner_id: UserId, connection: &Database) -> sqlx::Result<DatabaseId> {
        Self {
            run_id: 0, // This will be set by the database
            owner_id: owner_id.0,
            status: RunStatus::Running.to_str().to_string(),
            start_time: Some(chrono::Utc::now()),
            end_time: None,
        }
        .insert(connection)
        .await
    }

    /// Update the status and end time of a task run.
    pub async fn end_with_status(
        run_id: DatabaseId,
        owner_id: UserId,
        status: RunStatus,
        connection: &Database,
    ) -> sqlx::Result<bool> {
        let current_time = chrono::Utc::now();
        let status = status.to_str();
        sqlx::query!(
            "UPDATE task_run SET status = ?, end_time = ? WHERE run_id = ? AND owner_id = ?",
            status,
            current_time,
            run_id,
            owner_id.0
        )
        .execute(connection)
        .await
        .map(|result| result.rows_affected() > 0)
    }

    /// Insert the task run into the database.
    ///
    /// Note: Insertions ignore the `run_id` field, as it is auto-incremented.
    pub async fn insert(&self, connection: &Database) -> sqlx::Result<DatabaseId> {
        sqlx::query!(
            "INSERT INTO task_run (status, owner_id, start_time, end_time) VALUES (?, ?, ?, ?)",
            self.status,
            self.owner_id,
            self.start_time,
            self.end_time
        )
        .execute(connection)
        .await
        .map(|result| result.last_insert_rowid())
    }

    /// Get a list of task run IDs.
    pub async fn get(id: DatabaseId, owner: UserId, connection: &Database) -> sqlx::Result<Self> {
        sqlx::query_as!(
            Self,
            r#"SELECT run_id, owner_id, status, start_time as "start_time?: DateTime<Utc>", end_time as "end_time?: DateTime<Utc>"
            FROM task_run
            WHERE run_id = ? AND owner_id = ?
            "#,
            id,
            owner
        )
        .fetch_one(connection)
        .await
    }

    /// Get a list of task run IDs.
    pub async fn list(owner: UserId, limit: i64, connection: &Database) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            r#"SELECT run_id, owner_id, status, start_time as "start_time?: DateTime<Utc>", end_time as "end_time?: DateTime<Utc>"
            FROM task_run
            WHERE owner_id = ?
            ORDER BY start_time DESC
            LIMIT ?
            "#,
            owner,
            limit
        )
        .fetch_all(connection)
        .await
    }
}

/// Get a task run by ID.
///
/// This uses multiple queries to reconstruct all the information needed to build the task graph.
pub async fn get_task_run_by_id(
    run_id: DatabaseId,
    owner: UserId,
    connection: &Database,
) -> anyhow::Result<Option<TaskRun>> {
    let task_run_record = TaskRunRecord::get(run_id, owner, connection).await?;
    let steps = TaskStep::list_steps_by_run_id(run_id, owner, connection).await?;
    if steps.is_empty() {
        return Ok(None);
    }

    let mut task_run = TaskRun {
        id: run_id,
        end_time_ms: task_run_record.end_time.map(|t| t.timestamp_millis()),
        start_time_ms: task_run_record.start_time.map(|t| t.timestamp_millis()),
        status: RunStatus::from_str(&task_run_record.status)?,
        steps: Vec::with_capacity(steps.len()),
    };

    for x in steps.into_iter() {
        task_run.steps.push(Step {
            step_id: x.step_id,
            parent_step_id: x.parent_step_id,
            inputs: x.inputs.0,
            outputs: x.outputs.0,
            step_action: if let Some(task_id) = x.task_id {
                StepAction::TaskId(task_id)
            } else if let Some(tool_id) = x.tool_id {
                StepAction::ToolId(tool_id)
            } else {
                anyhow::bail!("Task run step has neither task_id nor tool_id");
            },
            start_time_ms: x.start_time.map(|t| t.timestamp_millis()),
            end_time_ms: x.end_time.map(|t| t.timestamp_millis()),
        });
    }
    Ok(Some(task_run))
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use sqlx::types::Json;

    use crate::*;

    #[sqlx::test]
    async fn test_task_step(connection: Database) -> anyhow::Result<()> {
        let user = crate::User {
            user_id: 3,
            name: "Task Run Test User".to_string(),
        };
        user.insert(&connection).await?;

        let task_run = TaskRunRecord {
            run_id: 0,
            owner_id: user.user_id,
            status: "running".to_string(),
            start_time: Some(Utc::now()),
            end_time: None,
        };
        let run_id = task_run.insert(&connection).await?;

        let user_id = UserId(user.user_id);

        // Create a task first
        let task = Task {
            id: 0,
            owner_id: user.user_id,
            name: "Test Task for Run".to_string(),
            description: "This is a test task for task_run.".to_string(),
            template: "Test Template".to_string(),
            output_schema: Json(None),
            input_schema: Json(serde_json::Map::new()),
            available_tool_ids: Json(vec!["tool1".to_string()]),
        };

        let task_id = task.insert(&connection).await?;

        let now = Utc::now();

        // Create a task step
        let task_step = TaskStep {
            step_id: 1, // will be ignored during insert
            run_id,
            owner_id: user.user_id,
            parent_step_id: None,
            inputs: Json(serde_json::json!({"param": "value"})),
            outputs: Json(serde_json::json!({})),
            task_id: Some(task_id),
            tool_id: None,
            error: None,
            start_time: Some(now),
            end_time: None,
        };

        // Insert the task run
        let step_id = task_step.insert(&connection).await?;

        // Get the task run by ID
        let Some(fetched_run) = TaskStep::get_by_id(step_id, user_id, &connection).await? else {
            anyhow::bail!("Task run not found");
        };

        assert_eq!(fetched_run.step_id, step_id);
        assert_eq!(fetched_run.run_id, task_step.run_id);
        assert_eq!(fetched_run.owner_id, user.user_id);
        assert_eq!(fetched_run.task_id, Some(task_id));
        assert_eq!(
            fetched_run.inputs,
            Json(serde_json::json!({"param": "value"}))
        );
        assert_eq!(fetched_run.outputs, Json(serde_json::json!({})));
        assert_eq!(fetched_run.error, None);
        assert!(fetched_run.start_time.is_some());
        assert_eq!(fetched_run.end_time, None);

        // Get runs for a specific task
        let task_steps = TaskStep::get_for_task(task_id, user_id, &connection, Some(10)).await?;
        assert_eq!(task_steps.len(), 1);

        // List all runs
        let all_runs = TaskStep::list(user_id, &connection).await?;
        assert_eq!(all_runs.len(), 1);

        // Delete the run
        assert!(TaskStep::delete_by_id(step_id, user_id, &connection).await?);

        // Make sure it's gone
        assert!(
            TaskStep::get_by_id(step_id, user_id, &connection)
                .await?
                .is_none()
        );

        Ok(())
    }
}
