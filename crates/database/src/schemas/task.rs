use sqlx::{FromRow, types::Json};

use crate::{Database, DatabaseId, DatabaseTypeWithId, UserId};

/// A task is the smallest unit of work in a workflow.
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct Task {
    /// The ID of the task.
    pub task_id: DatabaseId,
    /// The ID of user that owns the task.
    pub owner_id: DatabaseId,
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
            "INSERT INTO task (owner_id, name, description, template, output_schema, input_schema, available_tool_ids)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            self.owner_id,
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
        owner: UserId,
        content: sauropod_schemas::Task,
        connection: &Database,
    ) -> sqlx::Result<bool> {
        let output_schema = Json(content.output_schema);
        let input_schema = Json(content.input_schema);
        let available_tool_ids = Json(content.available_tool_ids);
        sqlx::query!(
            "UPDATE task SET name = ?, template = ?, output_schema = ?, input_schema = ?, available_tool_ids = ? WHERE task_id = ? AND owner_id = ?",
            content.name,
            content.template.0,
            output_schema,
            input_schema,
            available_tool_ids,
            id,
            owner
        )
        .execute(connection)
        .await
        .map(|result| result.rows_affected() > 0)
    }
}

impl From<Task> for sauropod_schemas::Task {
    fn from(val: Task) -> Self {
        sauropod_schemas::Task {
            name: val.name,
            template: sauropod_schemas::Template(val.template),
            output_schema: val.output_schema.0,
            input_schema: val.input_schema.0,
            available_tool_ids: val.available_tool_ids.to_vec(),
        }
    }
}
impl From<sauropod_schemas::Task> for Task {
    fn from(val: sauropod_schemas::Task) -> Task {
        Task {
            task_id: 0,
            owner_id: 0,
            name: val.name,
            description: "".to_string(),
            template: val.template.0,
            output_schema: Json(val.output_schema),
            input_schema: Json(val.input_schema),
            available_tool_ids: Json(val.available_tool_ids),
        }
    }
}

impl DatabaseTypeWithId for sauropod_schemas::Task {
    async fn get_by_id(
        id: DatabaseId,
        owner: UserId,
        connection: &Database,
    ) -> sqlx::Result<Option<Self>> {
        Ok(Task::get_by_id(id, owner, connection)
            .await?
            .map(|x| x.into()))
    }

    async fn delete_by_id(
        id: DatabaseId,
        owner: UserId,
        connection: &Database,
    ) -> sqlx::Result<bool> {
        Task::delete_by_id(id, owner, connection).await
    }

    async fn list(owner: UserId, connection: &Database) -> sqlx::Result<Vec<Self>> {
        Ok(Task::list(owner, connection)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect())
    }
}

impl DatabaseTypeWithId for Task {
    async fn get_by_id(
        id: DatabaseId,
        owner: UserId,
        connection: &Database,
    ) -> sqlx::Result<Option<Self>> {
        let result = sqlx::query_as!(
            Task,
            r#"
                SELECT
                    task_id,
                    owner_id,
                    name,
                    description,
                    template,
                    output_schema as "output_schema: Json<Option<serde_json::Map<String, serde_json::Value>>>",
                    input_schema as "input_schema: Json<serde_json::Map<String, serde_json::Value>>",
                    available_tool_ids as "available_tool_ids: Json<Vec<String>>"
                FROM task
                WHERE task_id = ? AND owner_id = ?
            "#,
            id,
            owner
        )
        .fetch_one(connection)
        .await;

        match result {
            Ok(task) => Ok(Some(task)),
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
            "DELETE FROM task WHERE task_id = ? AND owner_id = ?",
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
        // We use a query builder instead of the query macro because there's a glitch in the macro where it thinks owner_id is nullable
        let mut builder = sqlx::query_builder::QueryBuilder::new(
            "SELECT task_id, owner_id, name, description, template, output_schema, input_schema, available_tool_ids \
            FROM task WHERE owner_id = ",
        );
        builder.push_bind(owner);
        builder
            .build()
            .fetch_all(connection)
            .await?
            .iter()
            .map(Task::from_row)
            .collect::<sqlx::Result<Vec<Task>>>()
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    use sqlx::types::Json;

    #[sqlx::test]
    async fn test_task(connection: Database) -> anyhow::Result<()> {
        let user = User {
            user_id: 2,
            name: "Test User".to_string(),
        };
        user.insert(&connection).await?;

        let user_id = UserId(user.user_id);

        let task = Task {
            task_id: 1234,
            owner_id: user.user_id,
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
        let Some(fetched_task) = Task::get_by_id(1, user_id, &connection).await? else {
            anyhow::bail!("Task not found");
        };
        assert_eq!(fetched_task.task_id, 1);

        // List tasks
        let task_list = Task::list(user_id, &connection).await?;
        assert_eq!(task_list.len(), 1);

        // Try to get the task again as a different user
        assert!(Task::get_by_id(1, UserId(3), &connection).await?.is_none());

        assert_eq!(fetched_task.name, task.name);

        // Delete the task
        assert!(Task::delete_by_id(1, user_id, &connection).await?);
        // Try to get the task again
        assert!(Task::get_by_id(1, user_id, &connection).await?.is_none());
        // Try to delete the task again
        assert!(!(Task::delete_by_id(1, user_id, &connection).await?));

        Ok(())
    }
}
