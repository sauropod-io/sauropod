//! Database.

use anyhow::Context;

mod schema;
pub use schema::*;
use sqlx::sqlite::SqliteConnectOptions;

pub type Database = sqlx::SqlitePool;

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

    let connection_options = SqliteConnectOptions::new()
        .create_if_missing(true)
        .filename(path);
    let pool = sqlx::sqlite::SqlitePool::connect_with(connection_options)
        .await
        .with_context(|| format!("opening database {}", path.display()))?;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .with_context(|| "running migrations")?;

    Ok(pool)
}
