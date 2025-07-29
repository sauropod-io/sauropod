//! Database.

use anyhow::Context;
use sqlx::sqlite::SqliteConnectOptions;

pub type Database = sqlx::SqlitePool;

async fn create_database_with_options(
    connection_options: SqliteConnectOptions,
) -> anyhow::Result<Database> {
    let pool = sqlx::sqlite::SqlitePool::connect_with(connection_options)
        .await
        .context("opening database")?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("running migrations")?;

    Ok(pool)
}

/// Create a new database instance.
pub async fn create_database(path: &std::path::Path) -> anyhow::Result<Database> {
    // Create the directory if it doesn't exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent)
                .with_context(|| format!("creating database parent directory {}", parent.display()))
            {
                tracing::error!("Error {:?}", e);
                return Err(e);
            }
        }
    }

    create_database_with_options(
        SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(path),
    )
    .await
}

/// Create a new in-memory database instance.
///
/// # Note
///
/// This is only for unit testing.
pub async fn create_in_memory() -> anyhow::Result<Database> {
    create_database_with_options(SqliteConnectOptions::new().in_memory(true)).await
}
