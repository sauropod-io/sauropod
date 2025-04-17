use sqlx::FromRow as _;
use sqlx::types::Json;

use crate::{Database, DatabaseId, DatabaseTypeWithId, UserId};

/// A run of a task or tool.
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct TaskRun {
    /// The ID of the run.
    pub run_id: DatabaseId,
    /// The ID of user that owns the run.
    pub owner_id: DatabaseId,
    /// The ID of the step in the task execution.
    pub step_id: Option<DatabaseId>,
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
}

impl TaskRun {
    /// Insert the task run into the database.
    ///
    /// Note: Insertions ignore the run_id field, as it is auto-incremented.
    pub async fn insert(&self, connection: &Database) -> sqlx::Result<DatabaseId> {
        sqlx::query!(
            "INSERT INTO task_run (owner_id, step_id, parent_step_id, inputs, outputs, task_id, tool_id, error)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            self.owner_id,
            self.step_id,
            self.parent_step_id,
            self.inputs,
            self.outputs,
            self.task_id,
            self.tool_id,
            self.error
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
            "SELECT run_id, owner_id, step_id, parent_step_id, inputs, outputs, task_id, tool_id, error \
             FROM task_run WHERE task_id = ",
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
            .map(TaskRun::from_row)
            .collect::<sqlx::Result<Vec<TaskRun>>>()
    }
}

impl DatabaseTypeWithId for TaskRun {
    async fn get_by_id(
        id: DatabaseId,
        owner: UserId,
        connection: &Database,
    ) -> sqlx::Result<Option<Self>> {
        let result = sqlx::query_as!(
            TaskRun,
            r#"
                SELECT run_id, owner_id, step_id, parent_step_id, inputs as "inputs: Json<serde_json::Value>",
                       outputs as "outputs: Json<serde_json::Value>", task_id, tool_id, error
                FROM task_run
                WHERE run_id = ? AND owner_id = ?
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
            "DELETE FROM task_run WHERE run_id = ? AND owner_id = ?",
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
            "SELECT run_id, owner_id, step_id, parent_step_id, inputs, outputs, task_id, tool_id, error \
            FROM task_run WHERE owner_id = ",
        );
        builder.push_bind(owner);
        builder
            .build()
            .fetch_all(connection)
            .await?
            .iter()
            .map(TaskRun::from_row)
            .collect::<sqlx::Result<Vec<TaskRun>>>()
    }
}

#[cfg(test)]
mod test {
    use sqlx::types::Json;

    use crate::*;

    #[sqlx::test]
    async fn test_task_run(connection: Database) -> anyhow::Result<()> {
        let user = crate::User {
            user_id: 3,
            name: "Task Run Test User".to_string(),
        };
        user.insert(&connection).await?;

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

        // Create a task run
        let task_run = TaskRun {
            run_id: 0, // will be ignored during insert
            owner_id: user.user_id,
            step_id: Some(1),
            parent_step_id: None,
            inputs: Json(serde_json::json!({"param": "value"})),
            outputs: Json(serde_json::json!({})),
            task_id: Some(task_id),
            tool_id: None,
            error: None,
        };

        // Insert the task run
        let run_id = task_run.insert(&connection).await?;

        // Get the task run by ID
        let Some(fetched_run) = TaskRun::get_by_id(run_id, user_id, &connection).await? else {
            anyhow::bail!("Task run not found");
        };

        assert_eq!(fetched_run.run_id, run_id);
        assert_eq!(fetched_run.owner_id, user.user_id);
        assert_eq!(fetched_run.task_id, Some(task_id));
        assert_eq!(
            fetched_run.inputs,
            Json(serde_json::json!({"param": "value"}))
        );
        assert_eq!(fetched_run.outputs, Json(serde_json::json!({})));
        assert_eq!(fetched_run.error, None);

        // Get runs for a specific task
        let task_runs = TaskRun::get_for_task(task_id, user_id, &connection, Some(10)).await?;
        assert_eq!(task_runs.len(), 1);

        // List all runs
        let all_runs = TaskRun::list(user_id, &connection).await?;
        assert_eq!(all_runs.len(), 1);

        // Delete the run
        assert!(TaskRun::delete_by_id(run_id, user_id, &connection).await?);

        // Make sure it's gone
        assert!(
            TaskRun::get_by_id(run_id, user_id, &connection)
                .await?
                .is_none()
        );

        Ok(())
    }
}
