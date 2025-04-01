use std::io::Write;

pub struct TypeInfoForDatabase {
    pub schema_type_path: &'static str,
    pub json_schema_name: String,
    pub schema: serde_json::Value,
}

#[macro_export]
macro_rules! database_types {
    ($($schema:ty),* $(,)?) => {
        let mut schemas = Vec::with_capacity(4);
        let mut schema_generator = schemars::SchemaGenerator::default();

        $(
            schemas.push($crate::database::TypeInfoForDatabase{
                schema_type_path: std::any::type_name::<$schema>(),
                json_schema_name: <$schema as schemars::JsonSchema>::schema_name().to_string(),
                schema: <$schema as schemars::JsonSchema>::json_schema(&mut schema_generator).into()
            });
        )*
        $crate::database::generate_database_traits(schemas)?;
    };
}

pub fn generate_database_traits(mut schemas: Vec<TypeInfoForDatabase>) -> std::io::Result<()> {
    let mut output_path = crate::paths::get_crate_path("database");
    output_path.push("src");
    output_path.push("generated.rs");

    let mut output = std::fs::File::create(output_path)?;

    write!(
        &mut output,
        r#"//! Generated code.

        use rusqlite::Connection;

        use crate::{{DatabaseType, DatabaseTypeWithID, DatabaseTypeWithName}};
        "#
    )?;

    schemas.sort_by_key(|type_info| type_info.schema_type_path);

    let mut table_creation_statements = Vec::with_capacity(schemas.len());

    for schema in schemas {
        let schema_type_path = schema.schema_type_path;
        let table_name = &schema.json_schema_name;

        write!(
            &mut output,
            r#"
            impl DatabaseType for {schema_type_path} {{
                fn table_name() -> &'static str {{
                    "{}"
                }}
            }}
            "#,
            &table_name,
        )?;

        write!(
            &mut output,
            r#"
            impl DatabaseTypeWithID for {schema_type_path} {{
                fn get_by_id_statement() -> &'static str {{
                    "SELECT content FROM {table_name} WHERE id = ?"
                }}

                fn insert_statement() -> &'static str {{
                    "INSERT INTO {table_name} (content) VALUES (?)"
                }}

                fn update_by_id_statement() -> &'static str {{
                    "UPDATE {table_name} SET content = ? WHERE id = ?"
                }}

                fn delete_by_id_statement() -> &'static str {{
                    "DELETE FROM {table_name} WHERE id = ?"
                }}
            }}
            "#
        )?;

        if let Some(properties) = schema.schema.get("properties").and_then(|x| x.as_object()) {
            if properties.contains_key("name") {
                write!(
                    &mut output,
                    r#"
                    impl DatabaseTypeWithName for {schema_type_path} {{}}
                    "#
                )?;
            }
        };

        table_creation_statements.push(format!(
            "crate::create_table_for_type_with_id::<{}>(connection)?;",
            schema_type_path
        ));
    }

    let table_creation_statements_string = table_creation_statements.join("\n");
    write!(
        &mut output,
        r#"
        pub(crate) fn create_tables(connection: &Connection) -> anyhow::Result<()> {{
            {}
            Ok(())
        }}
        "#,
        table_creation_statements_string
    )?;

    Ok(())
}
