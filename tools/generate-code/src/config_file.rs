//! Code generation to support the config file and CLI options.

use std::{fmt::Write as _, io::Write as _};

/// Get the environment variable for a given config key.
fn environment_variable(key: &str) -> String {
    format!("{}_{}", sauropod_config::ENV_VAR_PREFIX, key.to_uppercase())
}

pub fn generate_code_for_config() -> anyhow::Result<()> {
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

    let schema = schemars::schema_for!(sauropod_config::Config).to_value();

    for property in sauropod_json_schema::iterate_properties(&schema)? {
        let value_map = property.schema.as_object().unwrap();
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

        if let Some(description) = property.schema["description"].as_str() {
            write!(&mut add_config_flags, ".help(r#\"",)?;

            for (i, line) in description.lines().enumerate() {
                if i != 0 {
                    write!(
                        &mut add_config_flags,
                        "\n{}",
                        if let Some(trimmed_line) = line.strip_prefix(" ") {
                            trimmed_line
                        } else {
                            line
                        }
                    )?;
                } else {
                    write!(&mut add_config_flags, "{}", line.trim_matches('.'))?;
                }
            }

            write!(&mut add_config_flags, "\"#)")?;
        };

        write!(&mut clap_to_config_source, "if let Some(value) = ")?;
        if sauropod_json_schema::is_string(property.schema) {
            write!(
                &mut clap_to_config_source,
                "matches.get_one::<String>(\"{cli_key}\").cloned().map(|x| config::Value::new(None, x))"
            )?;
        } else if sauropod_json_schema::is_integer(property.schema) {
            write!(
                &mut add_config_flags,
                ".value_parser(clap::value_parser!(i64))"
            )?;
            write!(
                &mut clap_to_config_source,
                "matches.get_one::<i64>(\"{cli_key}\").cloned().map(|x| config::Value::new(None, x))"
            )?;
        } else if sauropod_json_schema::is_boolean(property.schema) {
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
