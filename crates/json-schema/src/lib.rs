//! Helpers for working with JSON Schema.
//!
//! See <https://json-schema.org/> for more information on JSON Schema.

use std::collections::HashSet;

/// The base types supported by JSON Schema.
///
/// Represented as a bitmask for convenience.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i16)]
pub enum JsonSchemaType {
    Null = 0,
    Object = 1,
    Array = 1 << 1,
    String = 1 << 2,
    Number = 1 << 3,
    Integer = 1 << 4,
    Boolean = 1 << 5,
    AnyOf = 1 << 6,
    AllOf = 1 << 7,
    OneOf = 1 << 8,
    Not = 1 << 9,
}

impl JsonSchemaType {
    /// Convert a string to a `JsonSchemaType`.
    pub fn new(type_str: &str) -> Option<Self> {
        match type_str {
            "null" => Some(JsonSchemaType::Null),
            "object" => Some(JsonSchemaType::Object),
            "array" => Some(JsonSchemaType::Array),
            "string" => Some(JsonSchemaType::String),
            "number" => Some(JsonSchemaType::Number),
            "integer" => Some(JsonSchemaType::Integer),
            "boolean" => Some(JsonSchemaType::Boolean),
            _ => None,
        }
    }

    /// Convert a single type bitmask to a `JsonSchemaType`.
    pub fn from_bitmask(bitmask: i16) -> Option<Self> {
        match bitmask {
            0b00000000 => Some(JsonSchemaType::Null),
            0b00000001 => Some(JsonSchemaType::Object),
            0b00000010 => Some(JsonSchemaType::Array),
            0b00000100 => Some(JsonSchemaType::String),
            0b00001000 => Some(JsonSchemaType::Number),
            0b00010000 => Some(JsonSchemaType::Integer),
            0b00100000 => Some(JsonSchemaType::Boolean),
            _ => None,
        }
    }
}

/// Information about an enum-like type.
pub struct EnumValues {
    /// The type of the enum values.
    pub value_type: JsonSchemaType,
    /// The enum values.
    pub values: Vec<serde_json::Value>,
}

/// Information about a property in a JSON schema object.
pub struct JsonSchemaProperty<'a> {
    pub name: &'a str,
    pub schema: JsonSchemaInterface<'a>,
    pub is_required: bool,
}

/// A JSON schema interface that provides methods to interact with JSON schema objects.
#[derive(Debug, Clone)]
pub struct JsonSchemaInterface<'a> {
    /// A bit mask representing the schema type.
    type_mask: i16,
    /// The schema object.
    pub schema: &'a serde_json::Value,
}

impl<'a> JsonSchemaInterface<'a> {
    pub fn new(schema: &'a serde_json::Value) -> anyhow::Result<JsonSchemaInterface<'a>> {
        let schema_object = schema.as_object();

        let type_mask = if schema_object.is_some_and(|obj| obj.get("anyOf").is_some()) {
            JsonSchemaType::AnyOf as i16
        } else if schema_object.is_some_and(|obj| obj.get("allOf").is_some()) {
            JsonSchemaType::AllOf as i16
        } else if schema_object.is_some_and(|obj| obj.get("oneOf").is_some()) {
            JsonSchemaType::OneOf as i16
        } else if schema_object.is_some_and(|obj| obj.get("not").is_some()) {
            JsonSchemaType::Not as i16
        } else {
            let type_value = schema.get("type");

            if let Some(type_str) = type_value.and_then(|x| x.as_str()) {
                if let Some(parsed_type) = JsonSchemaType::new(type_str) {
                    parsed_type as i16
                } else {
                    anyhow::bail!("Unknown schema type: {}", type_str);
                }
            } else if let Some(type_array) = type_value.and_then(|x| x.as_array()) {
                let type_strings = type_array.iter().filter_map(|item| item.as_str());
                let mut mask = if let Some(first) = type_strings.clone().next() {
                    if let Some(parsed_type) = JsonSchemaType::new(first) {
                        parsed_type as i16
                    } else {
                        anyhow::bail!("Unknown schema type: {}", first);
                    }
                } else {
                    anyhow::bail!("Schema type is an empty array");
                };
                for type_str in type_strings.skip(1) {
                    if let Some(parsed_type) = JsonSchemaType::new(type_str) {
                        mask |= parsed_type as i16;
                    } else {
                        anyhow::bail!("Unknown schema type: {}", type_str);
                    }
                }
                mask as i16
            } else {
                anyhow::bail!("Schema type is not a string or array: {:#?}", type_value);
            }
        };

        Ok(JsonSchemaInterface { schema, type_mask })
    }

    /// Get the description.
    pub fn get_description(&self) -> Option<&str> {
        self.schema.get("description").and_then(|x| x.as_str())
    }

    /// Get member types from an `anyOf`, `allOf`, or `noneOf` schema.
    pub fn get_member_types(&self) -> anyhow::Result<Vec<JsonSchemaInterface<'_>>> {
        let members = if self.is_any_of() {
            self.schema.get("anyOf").and_then(|x| x.as_array())
        } else if self.is_all_of() {
            self.schema.get("allOf").and_then(|x| x.as_array())
        } else if self.is_one_of() {
            self.schema.get("oneOf").and_then(|x| x.as_array())
        } else {
            None
        };

        if let Some(members) = members {
            let mut result = Vec::with_capacity(members.len());
            for member in members {
                let member_interface = JsonSchemaInterface::new(member)?;
                result.push(member_interface);
            }
            Ok(result)
        } else {
            anyhow::bail!("Schema is not an anyOf, allOf, or oneOf");
        }
    }

    /// Check if the schema type contains the specified type.
    pub fn is_type(&self, schema_type: JsonSchemaType) -> bool {
        self.type_mask & schema_type as i16 != 0
    }

    /// Check if the schema type contains object.
    pub fn is_object(&self) -> bool {
        self.is_type(JsonSchemaType::Object)
    }

    /// Check if the schema type contains array.
    pub fn is_array(&self) -> bool {
        self.is_type(JsonSchemaType::Array)
    }

    /// Check if the schema type contains string.
    pub fn is_string(&self) -> bool {
        self.is_type(JsonSchemaType::String)
    }

    /// Check if the schema type contains number.
    pub fn is_number(&self) -> bool {
        self.is_type(JsonSchemaType::Number)
    }

    /// Check if the schema type contains integer.
    pub fn is_integer(&self) -> bool {
        self.is_type(JsonSchemaType::Integer)
    }

    /// Check if the schema type contains boolean.
    pub fn is_boolean(&self) -> bool {
        self.is_type(JsonSchemaType::Boolean)
    }

    /// Check if the schema type contains null.
    pub fn is_null(&self) -> bool {
        self.is_type(JsonSchemaType::Null)
    }

    /// Check if the schema type contains anyOf.
    pub fn is_any_of(&self) -> bool {
        self.is_type(JsonSchemaType::AnyOf)
    }

    /// Check if the schema type contains allOf.
    pub fn is_all_of(&self) -> bool {
        self.is_type(JsonSchemaType::AllOf)
    }

    /// Check if the schema type contains oneOf.
    pub fn is_one_of(&self) -> bool {
        self.is_type(JsonSchemaType::OneOf)
    }

    /// Check if the schema type contains not.
    pub fn is_not(&self) -> bool {
        self.is_type(JsonSchemaType::Not)
    }

    /// Get the value from a `const` type.
    pub fn const_value(&self) -> Option<&serde_json::Value> {
        self.schema.get("const")
    }

    /// Iterate over the properties of a JSON schema object.
    pub fn properties(&self) -> anyhow::Result<Vec<JsonSchemaProperty<'a>>> {
        if !self.is_object() {
            return Err(anyhow::anyhow!("Schema is not an object"));
        }

        let required: HashSet<&str> = if let Some(required) = self.schema.get("required") {
            if let Some(required) = required.as_array() {
                required.iter().filter_map(|item| item.as_str()).collect()
            } else {
                return Err(anyhow::anyhow!("Required field is not an array"));
            }
        } else {
            HashSet::new()
        };

        if let Some(properties) = self.schema.get("properties").and_then(|v| v.as_object()) {
            let mut result = Vec::with_capacity(properties.len());
            for (name, schema) in properties {
                let schema_interface = JsonSchemaInterface::new(schema)?;
                result.push(JsonSchemaProperty {
                    name,
                    schema: schema_interface,
                    is_required: required.contains(name.as_str()),
                });
            }
            Ok(result)
        } else {
            Err(anyhow::anyhow!("Properties field is missing"))
        }
    }

    /// Get the properties field from the underlying JSON object.
    pub fn properties_map(&self) -> anyhow::Result<&serde_json::Map<String, serde_json::Value>> {
        if !self.is_object() {
            return Err(anyhow::anyhow!("Schema is not an object"));
        }

        if let Some(properties) = self.schema.get("properties").and_then(|v| v.as_object()) {
            Ok(properties)
        } else {
            Err(anyhow::anyhow!("Properties field is missing or malformed"))
        }
    }

    /// Get the item type in a JSON schema array.
    pub fn items(&self) -> anyhow::Result<Self> {
        if !self.is_array() {
            return Err(anyhow::anyhow!("Schema is not an array"));
        }

        if let Some(items) = self.schema.get("items") {
            Ok(Self::new(items)?)
        } else {
            Err(anyhow::anyhow!("items field is missing is array type"))
        }
    }

    pub fn enum_values(&self) -> anyhow::Result<Option<EnumValues>> {
        if self.is_one_of() {
            let members = self.get_member_types()?;
            let first_member_type = if let Some(first) = members
                .first()
                .and_then(|x| JsonSchemaType::from_bitmask(x.type_mask))
            {
                first
            } else {
                // If the oneOf is empty, return None
                return Ok(None);
            };

            if members.iter().all(|member| {
                member.type_mask == (first_member_type as i16) && member.const_value().is_some()
            }) {
                let values: Vec<serde_json::Value> = members
                    .iter()
                    .filter_map(|member| member.const_value())
                    .cloned()
                    .collect();
                Ok(Some(EnumValues {
                    value_type: first_member_type,
                    values,
                }))
            } else {
                Ok(None)
            }
        } else {
            anyhow::bail!("Schema is not a oneOf");
        }
    }

    /// Get the default value from the schema.
    pub fn get_default(&self) -> Option<&serde_json::Value> {
        self.schema.get("default")
    }
}
