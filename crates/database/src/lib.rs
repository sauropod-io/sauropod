//! Database.

mod generated;

use std::cell::RefCell;

use anyhow::Context;
use rusqlite::Connection;

/// The ID type used in the database.
pub type DatabaseId = i64;

macro_rules! execute {
    ($connection:expr, $sql:expr, $params:expr) => {{
        let _span = tracing::debug_span!("sql query").entered();
        let sql = $sql;
        tracing::debug!("Executing SQL query: {sql}", sql = sql);
        $connection.execute(sql, $params)
    }};
    ($connection:expr, $sql:expr) => {
        execute!($connection, $sql, [])
    };
}

/// A trait implemented by all types useable in the database.
pub trait DatabaseType {
    /// The name of the table in the database containing the type.
    fn table_name() -> &'static str;
}

/// Marker trait for objects that have a name for use in the database.
pub trait DatabaseTypeWithName: DatabaseType {}

/// Trait for objects that have an ID for use in the database.
pub trait DatabaseTypeWithID: DatabaseType {
    /// Get a SQL query to get an object by ID.
    fn get_by_id_statement() -> &'static str;

    /// Get a SQL query to insert an object by ID.
    fn insert_statement() -> &'static str;

    /// Get a SQL query to update an object by ID.
    fn update_by_id_statement() -> &'static str;

    /// Get a SQL query to delete an object by ID.
    fn delete_by_id_statement() -> &'static str;
}

/// Database interface.
pub struct Database {
    /// The connection to the database.
    path: std::path::PathBuf,
}

/// Create table for a type that has an ID.
pub(crate) fn create_table_for_type_with_id<T: DatabaseTypeWithID>(
    connection: &Connection,
) -> anyhow::Result<()> {
    execute!(
        connection,
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content JSON NOT NULL
        )",
            T::table_name()
        )
    )?;
    Ok(())
}

impl Database {
    /// Create a database instance.
    pub fn new(path: std::path::PathBuf) -> anyhow::Result<Self> {
        Ok(Self { path })
    }

    /// Initialize the database.
    pub fn init(&self) -> anyhow::Result<()> {
        self.with_connection(|connection| {
            crate::generated::create_tables(connection)?;
            Ok(())
        })
    }

    /// Use a thread-local connection.
    pub fn with_connection<F, R>(&self, f: F) -> anyhow::Result<R>
    where
        F: FnOnce(&rusqlite::Connection) -> anyhow::Result<R>,
    {
        thread_local! {
            static CONNECTION: RefCell<Option<rusqlite::Connection>> = const { RefCell::new(None) };
        }
        CONNECTION.with(|connection_cell| {
            let mut connection_optional = connection_cell.borrow_mut();
            if connection_optional.is_none() {
                *connection_optional = Some(rusqlite::Connection::open(&self.path)?);
            }
            let connection = connection_optional.as_ref().unwrap();
            f(connection)
        })
    }

    /// Look up an object by ID.
    pub fn get_by_id<T>(&self, id: DatabaseId) -> anyhow::Result<Option<T>>
    where
        for<'de> T: DatabaseTypeWithID + serde::Deserialize<'de>,
    {
        self.with_connection(|connection: &Connection| {
            let _span_guard =
                tracing::debug_span!("Query", table = T::table_name(), id = id).entered();
            tracing::debug!("Querying {table}: {id}", table = T::table_name(), id = id);

            match connection.query_row(T::get_by_id_statement(), [&id], |row| {
                row.get::<_, serde_json::Value>(0)
            }) {
                Ok(content) => {
                    let object = serde_json::from_value(content).with_context(|| {
                        format!(
                            "Error deserializing value from the database's {} table",
                            T::table_name()
                        )
                    })?;
                    Ok(Some(object))
                }
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e.into()),
            }
        })
    }

    /// Create an object and return its unique ID.
    pub fn insert<T: DatabaseTypeWithID + serde::Serialize>(
        &self,
        object: &T,
    ) -> anyhow::Result<DatabaseId> {
        self.with_connection(|connection: &Connection| {
            let _span_guard = tracing::debug_span!("Insert", table = T::table_name()).entered();
            tracing::debug!("Inserting into {table}", table = T::table_name());

            let content = serde_json::to_string(&object)?;
            match connection.execute(T::insert_statement(), [&content]) {
                Ok(0) => {
                    anyhow::bail!("No rows were inserted during an unconditional insert")
                }
                Ok(_) => Ok(connection.last_insert_rowid()),
                Err(e) => Err(e.into()),
            }
        })
    }

    /// Update an object by ID.
    ///
    /// Returns `true` if the object was updated, `false` if it was not found.
    pub fn update_by_id<T: DatabaseTypeWithID + serde::Serialize>(
        &self,
        id: DatabaseId,
        object: &T,
    ) -> anyhow::Result<bool> {
        self.with_connection(|connection: &Connection| {
            let _span_guard =
                tracing::debug_span!("Update", table = T::table_name(), id = id).entered();
            tracing::debug!("Updating {table}: {id}", table = T::table_name(), id = id);

            let content = serde_json::to_string(&object)?;
            match connection.execute(T::update_by_id_statement(), rusqlite::params![content, id]) {
                Ok(0) => {
                    Ok(false) // No rows were updated
                }
                Ok(_) => Ok(true),
                Err(e) => Err(e.into()),
            }
        })
    }

    /// Delete an object by ID.
    ///
    /// Returns `true` if the object was deleted, `false` if it was not found.
    pub fn delete_by_id<T: DatabaseTypeWithID>(&self, id: DatabaseId) -> anyhow::Result<bool> {
        self.with_connection(|connection: &Connection| {
            let _span_guard =
                tracing::debug_span!("Delete", table = T::table_name(), id = id).entered();
            tracing::debug!("Deleting {table}: {id}", table = T::table_name(), id = id);

            match connection.execute(T::delete_by_id_statement(), [id]) {
                Ok(0) => Ok(false), // No rows were deleted
                Ok(_) => Ok(true),  // Successfully deleted
                Err(e) => Err(e.into()),
            }
        })
    }
}
