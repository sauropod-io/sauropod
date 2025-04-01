//! Generated code.

use rusqlite::Connection;

use crate::{DatabaseType, DatabaseTypeWithID, DatabaseTypeWithName};

impl DatabaseType for sauropod_schemas::task::Task {
    fn table_name() -> &'static str {
        "Task"
    }
}

impl DatabaseTypeWithID for sauropod_schemas::task::Task {
    fn get_by_id_statement() -> &'static str {
        "SELECT content FROM Task WHERE id = ?"
    }

    fn insert_statement() -> &'static str {
        "INSERT INTO Task (content) VALUES (?)"
    }

    fn update_by_id_statement() -> &'static str {
        "UPDATE Task SET content = ? WHERE id = ?"
    }

    fn delete_by_id_statement() -> &'static str {
        "DELETE FROM Task WHERE id = ?"
    }
}

impl DatabaseTypeWithName for sauropod_schemas::task::Task {}

impl DatabaseType for sauropod_schemas::workflow::Workflow {
    fn table_name() -> &'static str {
        "Workflow"
    }
}

impl DatabaseTypeWithID for sauropod_schemas::workflow::Workflow {
    fn get_by_id_statement() -> &'static str {
        "SELECT content FROM Workflow WHERE id = ?"
    }

    fn insert_statement() -> &'static str {
        "INSERT INTO Workflow (content) VALUES (?)"
    }

    fn update_by_id_statement() -> &'static str {
        "UPDATE Workflow SET content = ? WHERE id = ?"
    }

    fn delete_by_id_statement() -> &'static str {
        "DELETE FROM Workflow WHERE id = ?"
    }
}

impl DatabaseTypeWithName for sauropod_schemas::workflow::Workflow {}

pub(crate) fn create_tables(connection: &Connection) -> anyhow::Result<()> {
    crate::create_table_for_type_with_id::<sauropod_schemas::task::Task>(connection)?;
    crate::create_table_for_type_with_id::<sauropod_schemas::workflow::Workflow>(connection)?;
    Ok(())
}
