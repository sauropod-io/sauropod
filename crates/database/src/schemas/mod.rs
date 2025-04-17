mod task;
pub use task::*;
mod task_run;
pub use task_run::*;
mod user;
pub use user::*;

/// ID of types stored in the database.
pub type DatabaseId = i64;

/// Trait for objects that have an ID for use in the database.
pub trait DatabaseTypeWithId: Sized {
    /// Get a SQL query to get an object by ID.
    ///
    /// Returns `None` if the object was not found.
    fn get_by_id(
        id: DatabaseId,
        owner: UserId,
        connection: &crate::Database,
    ) -> impl Future<Output = sqlx::Result<Option<Self>>>;

    /// Delete an object by ID.
    ///
    /// Returns `true` if the object was deleted, `false` if it was not found.
    fn delete_by_id(
        id: DatabaseId,
        owner: UserId,
        connection: &crate::Database,
    ) -> impl Future<Output = sqlx::Result<bool>>;

    /// Get a list of objects.
    fn list(
        owner: UserId,
        connection: &crate::Database,
    ) -> impl Future<Output = sqlx::Result<Vec<Self>>>;
}
