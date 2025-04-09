/// Write a JSON schema for a given struct to a file.
pub fn write_schema<T: schemars::JsonSchema>() -> anyhow::Result<()> {
    let name = T::schema_name().to_lowercase();
    let api = crate::paths::get_api_path(&name).with_extension("json");
    let schema = schemars::schema_for!(T);
    std::fs::write(api, serde_json::to_string_pretty(&schema)?)?;
    Ok(())
}

/// Iterate over the properties of a JSON schema object.
pub(crate) fn iterate_properties(
    schema: &serde_json::Value,
) -> anyhow::Result<impl Iterator<Item = (String, serde_json::Value)>> {
    if let Some(properties) = schema.get("properties") {
        if let Some(properties) = properties.as_object() {
            let iter = properties.iter().map(|(k, v)| (k.clone(), v.clone()));
            return Ok(iter);
        }
    }
    Err(anyhow::anyhow!("No properties found in the schema"))
}
