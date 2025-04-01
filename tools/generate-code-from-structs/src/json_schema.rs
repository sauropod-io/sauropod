pub fn write_schema<T: schemars::JsonSchema>() -> anyhow::Result<()> {
    let name = T::schema_name().to_lowercase();
    let api = crate::paths::get_api_path(&name).with_extension("json");
    let schema = schemars::schema_for!(T);
    std::fs::write(api, serde_json::to_string_pretty(&schema)?)?;
    Ok(())
}
