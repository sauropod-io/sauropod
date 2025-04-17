use crate::{Database, DatabaseId, DatabaseTypeWithId};

/// ID of a user in the database.
#[derive(Clone, Copy, sqlx::Type)]
#[sqlx(transparent)]
pub struct UserId(pub DatabaseId);
/// Schema for users in the database.
#[derive(Debug, Clone, PartialEq, sqlx::FromRow)]
pub struct User {
    /// The ID of the user.
    pub user_id: DatabaseId,
    /// The name of the user.
    pub name: String,
}

impl User {
    /// Add a user to the database.
    pub async fn insert(&self, connection: &Database) -> sqlx::Result<DatabaseId> {
        sqlx::query!(
            "INSERT INTO user (user_id, name) VALUES (?, ?)",
            self.user_id,
            self.name
        )
        .execute(connection)
        .await
        .map(|result| result.last_insert_rowid())
    }
}

impl DatabaseTypeWithId for User {
    async fn get_by_id(
        id: DatabaseId,
        _owner: UserId,
        connection: &Database,
    ) -> sqlx::Result<Option<Self>> {
        let result = sqlx::query_as!(
            User,
            r#"
                SELECT user_id, name
                FROM user
                WHERE user_id = ?
            "#,
            id
        )
        .fetch_one(connection)
        .await;

        match result {
            Ok(user) => Ok(Some(user)),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(e),
        }
    }

    async fn delete_by_id(
        id: DatabaseId,
        _owner: UserId,
        connection: &Database,
    ) -> sqlx::Result<bool> {
        let result = sqlx::query!("DELETE FROM user WHERE user_id = ?", id)
            .execute(connection)
            .await;

        match result {
            Ok(result) => Ok(result.rows_affected() > 0),
            Err(sqlx::Error::RowNotFound) => Ok(false),
            Err(e) => Err(e),
        }
    }

    async fn list(_owner: UserId, connection: &Database) -> sqlx::Result<Vec<Self>> {
        let result = sqlx::query_as!(
            User,
            r#"
                SELECT user_id, name
                FROM user
            "#
        )
        .fetch_all(connection)
        .await;

        match result {
            Ok(users) => Ok(users),
            Err(e) => Err(e),
        }
    }
}
