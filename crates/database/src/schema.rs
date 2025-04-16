use sqlx::types::Json;

use crate::Database;

/// ID of types stored in the database.
pub type DatabaseId = i64;

/// Trait for objects that have an ID for use in the database.
pub trait DatabaseTypeWithId: Sized {
    /// Get a SQL query to get an object by ID.
    ///
    /// Returns `None` if the object was not found.
    fn get_by_id(
        id: DatabaseId,
        connection: &Database,
    ) -> impl Future<Output = sqlx::Result<Option<Self>>>;

    /// Delete an object by ID.
    ///
    /// Returns `true` if the object was deleted, `false` if it was not found.
    fn delete_by_id(
        id: DatabaseId,
        connection: &Database,
    ) -> impl Future<Output = sqlx::Result<bool>>;

    /// Get a list of objects.
    fn list(connection: &Database) -> impl Future<Output = sqlx::Result<Vec<Self>>>;
}

/// A task is the smallest unit of work in a workflow.
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Task {
    /// The ID of the task.
    pub id: DatabaseId,
    /// The ID of user that owns the task.
    pub owner: DatabaseId,
    /// The name of the task.
    pub name: String,
    /// Description of the task.
    pub description: String,
    /// The template to use.
    pub template: String,
    /// The output schema.
    pub output_schema: Json<Option<serde_json::Map<String, serde_json::Value>>>,
    /// The input schema of a task.
    pub input_schema: Json<serde_json::Map<String, serde_json::Value>>,
    /// The IDs of tools to make available to the LLM.
    pub available_tool_ids: Json<Vec<String>>,
}

impl Task {
    /// Insert the task into the database.
    ///
    /// Note: Insertions ignore the ID field, as it is auto-incremented.
    pub async fn insert(&self, connection: &Database) -> sqlx::Result<DatabaseId> {
        sqlx::query!(
            "INSERT INTO task (owner, name, description, template, output_schema, input_schema, available_tool_ids)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            self.owner,
            self.name,
            self.description,
            self.template,
            self.output_schema,
            self.input_schema,
            self.available_tool_ids
        )
        .execute(connection)
        .await
        .map(|result| result.last_insert_rowid())
    }

    /// Update the task in the database.
    pub async fn update(
        id: DatabaseId,
        content: sauropod_schemas::task::Task,
        connection: &Database,
    ) -> sqlx::Result<bool> {
        let output_schema = Json(content.output_schema);
        let input_schema = Json(content.input_schema);
        let available_tool_ids = Json(content.available_tool_ids);
        sqlx::query!(
            "UPDATE task SET name = ?, template = ?, output_schema = ?, input_schema = ?, available_tool_ids = ? WHERE id = ?",
            content.name,
            content.template.0,
            output_schema,
            input_schema,
            available_tool_ids,
            id
        )
        .execute(connection)
        .await
        .map(|result| result.rows_affected() > 0)
    }
}

impl From<Task> for sauropod_schemas::task::Task {
    fn from(val: Task) -> Self {
        sauropod_schemas::task::Task {
            name: val.name,
            template: sauropod_schemas::task::Template(val.template),
            output_schema: val.output_schema.0,
            input_schema: val.input_schema.0,
            available_tool_ids: val.available_tool_ids.to_vec(),
        }
    }
}
impl From<sauropod_schemas::task::Task> for Task {
    fn from(val: sauropod_schemas::task::Task) -> Task {
        Task {
            id: 0,
            owner: 0,
            name: val.name,
            description: "".to_string(),
            template: val.template.0,
            output_schema: Json(val.output_schema),
            input_schema: Json(val.input_schema),
            available_tool_ids: Json(val.available_tool_ids),
        }
    }
}

impl DatabaseTypeWithId for sauropod_schemas::task::Task {
    async fn get_by_id(id: DatabaseId, connection: &Database) -> sqlx::Result<Option<Self>> {
        Ok(Task::get_by_id(id, connection).await?.map(|x| x.into()))
    }

    async fn delete_by_id(id: DatabaseId, connection: &Database) -> sqlx::Result<bool> {
        Task::delete_by_id(id, connection).await
    }

    async fn list(connection: &Database) -> sqlx::Result<Vec<Self>> {
        Ok(Task::list(connection)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect())
    }
}

impl DatabaseTypeWithId for Task {
    async fn get_by_id(id: DatabaseId, connection: &Database) -> sqlx::Result<Option<Self>> {
        let result = sqlx::query_as!(
            Task,
            r#"
                SELECT
                    id,
                    owner,
                    name,
                    description,
                    template,
                    output_schema as "output_schema: Json<Option<serde_json::Map<String, serde_json::Value>>>",
                    input_schema as "input_schema: Json<serde_json::Map<String, serde_json::Value>>",
                    available_tool_ids as "available_tool_ids: Json<Vec<String>>"
                FROM task
                WHERE id = ?
            "#,
            id
        )
        .fetch_one(connection)
        .await;

        match result {
            Ok(task) => Ok(Some(task)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn delete_by_id(id: DatabaseId, connection: &Database) -> sqlx::Result<bool> {
        let reuslt = sqlx::query!("DELETE FROM task WHERE id = ?", id)
            .execute(connection)
            .await;
        match reuslt {
            Ok(result) => Ok(result.rows_affected() > 0),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn list(connection: &Database) -> sqlx::Result<Vec<Self>> {
        let result = sqlx::query_as!(
            Task,
            r#"
                SELECT
                    id,
                    owner,
                    name,
                    description,
                    template,
                    output_schema as "output_schema: Json<Option<serde_json::Map<String, serde_json::Value>>>",
                    input_schema as "input_schema: Json<serde_json::Map<String, serde_json::Value>>",
                    available_tool_ids as "available_tool_ids: Json<Vec<String>>"
                FROM task
            "#
        )
        .fetch_all(connection)
        .await;

        match result {
            Ok(tasks) => Ok(tasks),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[sqlx::test]
    async fn test_task(connection: Database) -> anyhow::Result<()> {
        let task = Task {
            id: 1234,
            owner: 0,
            name: "Test Task".to_string(),
            description: "This is a test task.".to_string(),
            template: "Test Template".to_string(),
            output_schema: Json(None),
            input_schema: Json(serde_json::Map::new()),
            available_tool_ids: Json(vec!["tool1".to_string()]),
        };

        // Insert the task
        task.insert(&connection).await?;

        // Get the task by ID
        let Some(fetched_task) = Task::get_by_id(1, &connection).await? else {
            anyhow::bail!("Task not found");
        };
        assert_eq!(fetched_task.id, 1);

        assert_eq!(fetched_task.name, task.name);

        // Delete the task
        assert!(Task::delete_by_id(1, &connection).await?);
        // Try to get the task again
        let fetched_task = Task::get_by_id(1, &connection).await?;
        assert!(fetched_task.is_none());
        // Try to delete the task again
        assert!(!(Task::delete_by_id(1, &connection).await?));

        Ok(())
    }
}
