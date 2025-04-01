static REQUIRED: &str = "required";
static PROPERTIES: &str = "properties";

/// Parsed template data.
#[derive(Debug, serde::Serialize)]
pub struct ParsedTemplateData {
    /// The JSON schema for the input variables.
    #[serde(rename = "inputJsonSchema")]
    pub input_json_schema: serde_json::Value,
    /// The unknown variables in the template.
    #[serde(rename = "unknownVariables")]
    pub unknown_variables: Vec<String>,
}

/// Load a minijinja template, extract the input variables, and return a JSON schema where the leaves of each input variable are strings.
///
/// # Example
/// For the template string `Classify "{{ input.sentence }}" into the categories "{{ input.categories }}"`, the function will return the following JSON schema:
/// ```json
/// {
///   "type": "object",
///   "properties": {
///     "sentence": {
///       "type": "string"
///     },
///     "categories": {
///       "type": "string"
///     }
///   },
///   "required": ["sentence", "categories"]
/// }
/// ```
pub fn template_to_inputs(
    template: minijinja::Template<'_, '_>,
) -> anyhow::Result<serde_json::Value> {
    let variables = template.undeclared_variables(true);
    let mut input_variables = variables.into_iter().collect::<Vec<_>>();

    // Sort the input variables such that the longest comes first
    input_variables.sort_by_key(|b| std::cmp::Reverse(b.len()));

    let mut input_schema = serde_json::json!({
        "type": "object",
        PROPERTIES: {},
        REQUIRED: Vec::<String>::with_capacity(1),
    });
    for variable in &input_variables {
        let path: Vec<&str> = variable.split('.').collect();
        let last_path_part = path.last().unwrap();

        let mut current = &mut input_schema;
        for part in path.iter().take(path.len() - 1) {
            let requirment_array = current
                .as_object_mut()
                .unwrap()
                .entry(REQUIRED)
                .or_insert(serde_json::json!({}))
                .as_array_mut()
                .unwrap();

            if !requirment_array.contains(&serde_json::json!(part)) {
                requirment_array.push(serde_json::json!(part));
            }

            current = current
                .as_object_mut()
                .unwrap()
                .entry(PROPERTIES)
                .or_insert(serde_json::json!({}))
                .as_object_mut()
                .unwrap()
                .entry(part.to_string())
                .or_insert(serde_json::json!({
                    "type": "object",
                    PROPERTIES: {},
                    REQUIRED: [],
                }));
        }

        // Now add the leaf to the properties and the required list
        let as_object = current.as_object_mut().unwrap();
        as_object
            .entry(PROPERTIES)
            .or_insert(serde_json::json!({}))
            .as_object_mut()
            .unwrap()
            .entry(last_path_part.to_string())
            .or_insert(serde_json::json!({
                "type": "string",
            }));
        as_object
            .entry(REQUIRED)
            .or_insert(serde_json::json!([]))
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!(last_path_part));
    }

    Ok(input_schema)
}

/// Load a minijinja template from a string, extract the input variables, and return a JSON schema where the leaves of each input variable are strings.
pub fn template_string_to_inputs(template: &str) -> anyhow::Result<serde_json::Value> {
    let mut env = minijinja::Environment::new();
    env.add_template("template", template)?;
    template_to_inputs(env.get_template("template")?)
}

/// Load a minijinja template from a string and extract data about it.
pub fn template_string_to_parsed_data(template: &str) -> anyhow::Result<ParsedTemplateData> {
    let mut env = minijinja::Environment::new();
    env.add_template("template", template)?;
    Ok(ParsedTemplateData {
        input_json_schema: template_to_inputs(env.get_template("template")?)?,
        unknown_variables: env
            .get_template("template")?
            .undeclared_variables(true)
            .iter()
            .map(|s| s.to_string())
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use minijinja::Environment;
    use serde_json::json;

    #[test]
    fn test_two_variables() -> anyhow::Result<()> {
        let tpl_str =
            r#"Classify "{{ input.sentence }}" into the categories "{{ input.categories }}""#;
        let mut env: Environment<'_> = Environment::new();
        env.add_template("template", tpl_str)?;
        let result = template_to_inputs(env.get_template("template")?)?;
        assert_eq!(
            result,
            json!({
                "type": "object",
                "properties": {
                    "categories": {
                        "type": "string"
                    },
                    "sentence": {
                        "type": "string"
                    }
                },
                "required": ["categories", "sentence"]
            })
        );
        Ok(())
    }

    #[test]
    fn test_no_variables() -> anyhow::Result<()> {
        let tpl_str = "Hello world";
        let mut env = Environment::new();
        env.add_template("template", tpl_str)?;
        let result = template_to_inputs(env.get_template("template")?)?;
        assert_eq!(
            result,
            json!({
                "type": "object",
                "properties": {},
                "required": []
            })
        );
        Ok(())
    }

    #[test]
    fn test_nested_input() -> anyhow::Result<()> {
        let tpl_str = r#"Hi {{ input.user.name }} welcome!"#;
        let mut env = Environment::new();
        env.add_template("template", tpl_str)?;
        let result = template_to_inputs(env.get_template("template")?)?;
        assert_eq!(
            result,
            json!({
                "type": "object",
                "properties": {
                    "user": {
                        "type": "object",
                        "properties": {
                            "name": {
                                "type": "string"
                            }
                        },
                        "required": ["name"]
                    }
                },
                "required": ["user"]
            })
        );
        Ok(())
    }
}
