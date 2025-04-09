//! Helpers for working with JSON Schema.
//!
//! See <https://json-schema.org/> for more information on JSON Schema.

use std::collections::HashSet;

const TYPE: &str = "type";

pub fn is_type(schema: &serde_json::Value, type_name: &str) -> bool {
    if let Some(schema) = schema.get(TYPE) {
        if let Some(schema) = schema.as_str() {
            return schema == type_name;
        }
        if let Some(schema) = schema.as_array() {
            for item in schema {
                if let Some(item) = item.as_str() {
                    if item == type_name {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Check if the given schema type contains object.
pub fn is_object(schema: &serde_json::Value) -> bool {
    is_type(schema, "object")
}

/// Check if the given schema type contains array.
pub fn is_array(schema: &serde_json::Value) -> bool {
    is_type(schema, "array")
}

/// Check if the given schema contains string.
pub fn is_string(schema: &serde_json::Value) -> bool {
    is_type(schema, "string")
}

/// Check if the given schema type contains number.
pub fn is_number(schema: &serde_json::Value) -> bool {
    is_type(schema, "number")
}

/// Check if the given schema type contains number.
pub fn is_integer(schema: &serde_json::Value) -> bool {
    is_type(schema, "integer")
}

/// Check if the given schema contains boolean.
pub fn is_boolean(schema: &serde_json::Value) -> bool {
    is_type(schema, "boolean")
}

/// Check if the given schema contains null.
pub fn is_null(schema: &serde_json::Value) -> bool {
    is_type(schema, "null")
}

/// Information about a property in a JSON schema object.
pub struct JsonSchemaProperty<'a> {
    pub name: &'a str,
    pub schema: &'a serde_json::Value,
    pub is_required: bool,
}

/// Iterate over the properties of a JSON schema object.
pub fn iterate_properties(
    schema: &serde_json::Value,
) -> anyhow::Result<impl Iterator<Item = JsonSchemaProperty<'_>>> {
    if !is_object(schema) {
        return Err(anyhow::anyhow!("Schema is not an object"));
    }

    let required: HashSet<&str> = if let Some(required) = schema.get("required") {
        if let Some(required) = required.as_array() {
            required.iter().filter_map(|item| item.as_str()).collect()
        } else {
            return Err(anyhow::anyhow!("Required field is not an array"));
        }
    } else {
        HashSet::new()
    };

    if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
        Ok(properties
            .iter()
            .map(move |(name, schema)| JsonSchemaProperty {
                name,
                schema,
                is_required: required.contains(name.as_str()),
            }))
    } else {
        Err(anyhow::anyhow!("Properties field is missing"))
    }
}
