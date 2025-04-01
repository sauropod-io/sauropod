use {argh::FromArgs, std::fmt::Debug};

/// Schema validator tool.
#[derive(FromArgs, PartialEq, Debug)]
struct Command {
    /// the schema file to use
    #[argh(option, short = 's')]
    schema: Option<String>,
    #[argh(positional)]
    file: String,
}

fn main() -> anyhow::Result<()> {
    let args: Command = argh::from_env();

    let input_file_path = std::path::Path::new(&args.file);
    let mut json_value: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(input_file_path)?)?;

    let Some(json_map) = json_value.as_object_mut() else {
        anyhow::bail!("Expected a JSON object")
    };
    let schema_path = json_map
        .remove("$schema")
        .map(|v| input_file_path.parent().unwrap().join(v.to_string()))
        .or_else(|| args.schema.map(std::path::PathBuf::from))
        .ok_or_else(|| anyhow::anyhow!("$schema key is missing"))?;

    // Load the schema path relative to the input file
    let schema: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&schema_path)?)?;

    if let Err(validation_error) = jsonschema::validate(&schema, &json_value) {
        anyhow::bail!("Validation error: {}", validation_error)
    }

    Ok(())
}
