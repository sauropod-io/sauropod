//! Code generation to support the config file and CLI options.

use std::{fmt::Write as _, io::Write as _};

/// Get the environment variable for a given config key.
fn environment_variable(key: &str) -> String {
    format!(
        "{}_{}",
        sauropod_config::ENV_VAR_PREFIX,
        key.to_uppercase().replace('.', "_")
    )
}

pub fn generate_code_for_config() -> anyhow::Result<()> {
    let schema_generator = schemars::SchemaGenerator::new({
        let mut schema_settings = schemars::generate::SchemaSettings::draft2020_12();
        schema_settings.inline_subschemas = true;
        schema_settings
    });
    let schema = schema_generator
        .into_root_schema_for::<sauropod_config::Config>()
        .to_value();

    generate_config_documentation(&schema)?;

    let mut rust_output_path = crate::paths::get_crate_path("server");
    rust_output_path.push("src");
    rust_output_path.push("cli_generated.rs");

    let mut output = std::fs::File::create(rust_output_path)?;
    let mut add_config_flags = String::with_capacity(1024);
    let mut clap_to_config_source = String::with_capacity(1024);

    writeln!(
        &mut clap_to_config_source,
        r#"
        #[derive(Debug, Clone)]
        pub struct ClapConfigSource {{
            values: config::Map<String, config::Value>,
        }}

        impl config::Source for ClapConfigSource {{
            fn clone_into_box(&self) -> Box<dyn config::Source + Send + Sync> {{
                Box::new(self.clone())
            }}

            fn collect(&self) -> Result<config::Map<String, config::Value>, config::ConfigError> {{
                Ok(self.values.clone())
            }}
        }}

        pub fn clap_to_config_source(matches: clap::ArgMatches) -> Box<ClapConfigSource> {{
            let mut values = config::Map::new();
        "#
    )?;

    writeln!(
        &mut add_config_flags,
        "pub fn add_config_flags(parser: clap::Command) -> clap::Command {{"
    )?;
    writeln!(&mut add_config_flags, "    parser.args([")?;

    for property in sauropod_json_schema::JsonSchemaInterface::new(&schema)?.properties()? {
        let interface = &property.schema;
        let value_map = property.schema.schema.as_object().unwrap();
        let Some(value_type) = value_map.get("type") else {
            continue;
        };
        if value_type == "object" || value_type == "array" {
            continue;
        }

        let cli_key = property.name.replace('_', "-");
        let env_var = environment_variable(property.name);

        write!(
            &mut add_config_flags,
            r#"clap::Arg::new("{cli_key}").long("{cli_key}").env("{env_var}")"#
        )?;

        if let Some(description) = property
            .schema
            .schema
            .get("description")
            .and_then(|d| d.as_str())
        {
            write!(&mut add_config_flags, ".help(r#\"",)?;

            if let Some(line) = description.lines().next() {
                write!(&mut add_config_flags, "{}", line.trim_matches([' ', '.']))?;
            }

            write!(&mut add_config_flags, "\"#)")?;
        };

        write!(&mut clap_to_config_source, "if let Some(value) = ")?;
        if interface.is_string() {
            write!(
                &mut clap_to_config_source,
                "matches.get_one::<String>(\"{cli_key}\").cloned().map(|x| config::Value::new(None, x))"
            )?;
        } else if interface.is_integer() {
            write!(
                &mut add_config_flags,
                ".value_parser(clap::value_parser!(i64))"
            )?;
            write!(
                &mut clap_to_config_source,
                "matches.get_one::<i64>(\"{cli_key}\").cloned().map(|x| config::Value::new(None, x))"
            )?;
        } else if interface.is_boolean() {
            writeln!(&mut add_config_flags, ".action(clap::ArgAction::SetTrue)")?;
            write!(
                &mut clap_to_config_source,
                "matches.get_one::<bool>(\"{cli_key}\").cloned().map(|x| config::Value::new(None, x))"
            )?;
        } else {
            todo!(
                "Unsupported type: {} for field {}",
                value_type,
                property.name
            );
        }
        writeln!(
            &mut clap_to_config_source,
            "{{ values.insert(\"{}\".to_string(), value); }}",
            property.name
        )?;

        writeln!(&mut add_config_flags, ",")?
    }
    writeln!(&mut add_config_flags, "    ])")?;
    writeln!(&mut add_config_flags, "}}")?;

    writeln!(
        &mut clap_to_config_source,
        "Box::new(ClapConfigSource {{ values }})"
    )?;
    writeln!(&mut clap_to_config_source, "}}")?;

    // Write out to the file
    writeln!(&mut output, "{add_config_flags}\n\n{clap_to_config_source}")?;
    Ok(())
}

/// Generate documentation to describe the config file.
fn generate_config_documentation(schema: &serde_json::Value) -> anyhow::Result<()> {
    let config_output_path = crate::paths::get_docs_path("config.md");
    let mut config_md = String::with_capacity(1024);

    writeln!(&mut config_md, "# Configuration file")?;
    writeln!(
        &mut config_md,
        "Configuration can be provided via a config file, environment variables, or command line arguments.\n"
    )?;

    // Document top-level properties
    for property in sauropod_json_schema::JsonSchemaInterface::new(schema)?.properties()? {
        document_property(
            &mut config_md,
            &Property {
                name: property.name,
                path: property.name.to_string(),
                schema: &property.schema,
                supports_env_var: true,
                array_member: false,
                union_member: false,
            },
            2,
        )?;
    }

    std::fs::write(config_output_path, &config_md)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Property<'a> {
    name: &'a str,
    path: String,
    schema: &'a sauropod_json_schema::JsonSchemaInterface<'a>,
    supports_env_var: bool,
    array_member: bool,
    union_member: bool,
}

/// Recursively document a property and its children if it's an object
fn document_property(
    output: &mut String,
    property: &Property,
    heading_level: usize,
) -> anyhow::Result<()> {
    let heading = "#".repeat(heading_level);
    if property.schema.is_any_of() {
        let member_types = property.schema.get_member_types()?;
        writeln!(
            output,
            "\n`{}` supports {} different options:",
            property.path,
            member_types.len()
        )?;
        for (i, member) in member_types.iter().enumerate() {
            writeln!(output, "{} Option {}", heading, i + 1)?;
            document_property(
                output,
                &Property {
                    schema: member,
                    union_member: true,
                    ..property.clone()
                },
                heading_level,
            )?;
        }
        return Ok(());
    }

    // Create heading with property name
    if !property.union_member {
        if property.schema.is_array() {
            writeln!(output, "{} `[[{}]]`", heading, property.name)?;
        } else if property.schema.is_object() {
            writeln!(output, "{} `[{}]`", heading, property.name)?;
        } else {
            writeln!(output, "{} `{}`", heading, property.name)?;
        }
    }
    let property_path = property.path.clone();

    if !(property.schema.is_object()
        || property.schema.is_array()
        || property.array_member
        || property.union_member)
        && property.supports_env_var
    {
        let env_var = environment_variable(&property.path);
        writeln!(output, "- **Environment variable**: `{}`", env_var)?;
    }

    // Add type information
    if let Some(value_map) = property.schema.schema.as_object() {
        let value_type = value_map.get("type").and_then(|x| x.as_str());
        if let Some(value_type) = value_type {
            writeln!(output, "- **Type**: `{}`", value_type)?;
        } else if property.schema.is_any_of() {
            writeln!(output, "- **Type**: multiple options - see sections below")?;
        } else if property.schema.is_one_of() {
            if let Some(enum_values) = property.schema.enum_values()? {
                write!(output, "- **Type**: ")?;
                let mut values = enum_values.values.iter();
                if let Some(first_value) = values.next() {
                    write!(output, "`{}`", first_value)?;
                }
                for value in values {
                    write!(output, " | `{}`", value)?;
                }
                writeln!(output)?;
            } else {
                writeln!(output, "- **Type**: multiple options - see sections below")?;
            }
        }
    }

    if property.schema.is_array() {
        return document_property(
            output,
            &Property {
                schema: &property.schema.items()?,
                supports_env_var: false,
                array_member: true,
                ..property.clone()
            },
            heading_level + 1,
        );
    }

    if let Some(default) = property.schema.schema.get("default") {
        writeln!(output, "- **Default**: `{}`", default)?;
    }
    if let Some(examples) = property
        .schema
        .schema
        .get("examples")
        .and_then(|x| x.as_array())
    {
        if !examples.is_empty() {
            writeln!(output, "- **Example Value**: `{}`", examples[0])?;
        }
    }

    // Add description if available
    if let Some(description) = property.schema.get_description() {
        writeln!(output, "\n{}\n", description)?;
    }

    // Handle nested objects
    if property.schema.is_object() {
        writeln!(output)?;
        for nested_property in property.schema.properties()? {
            let nested_name = format!("{}.{}", property_path, nested_property.name);
            let nested_property = Property {
                name: nested_property.name,
                path: nested_name,
                schema: &nested_property.schema,
                supports_env_var: property.supports_env_var,
                array_member: false,
                union_member: false,
            };
            document_property(output, &nested_property, heading_level + 1)?;
        }
    }

    writeln!(output)?;
    Ok(())
}
