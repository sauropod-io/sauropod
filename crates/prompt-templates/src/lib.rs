//! Parsing library for prompt templates.

use std::collections::HashSet;

use nom::{
    IResult, Parser as _,
    branch::alt,
    bytes::complete::{tag, take_until, take_while1},
    sequence::delimited,
};

/// A section of a template.
#[derive(Debug)]
pub enum Section<'a> {
    /// Text to include verbatim.
    Text(&'a str),
    /// A hole to fill in with a variable.
    Variable(&'a str),
}

/// A section of the output.
pub enum OutputSection {
    /// Text.
    Text(String),
    /// A data blob, like an image or audio.
    Data(String),
}

/// A template.
#[derive(Debug)]
pub struct Template<'a> {
    pub sections: Vec<Section<'a>>,
}

impl<'a> Template<'a> {
    /// Create a template from a string.
    pub fn new(template: &'a str) -> Self {
        match parse_template(template) {
            Ok((_, sections)) => Self { sections },
            Err(_) => Self {
                sections: vec![Section::Text(template)],
            },
        }
    }

    /// Expand the template with the given variables.
    pub fn expand(
        &self,
        values: &serde_json::Map<String, serde_json::Value>,
        data_values: HashSet<String>,
    ) -> anyhow::Result<Vec<OutputSection>> {
        let mut result = Vec::with_capacity(self.sections.len());
        let mut current_text = String::with_capacity(
            self.sections
                .iter()
                .map(|x| match x {
                    Section::Text(text) => text.len(),
                    Section::Variable(_) => 16,
                })
                .sum(),
        );

        for section in &self.sections {
            match section {
                Section::Text(text) => current_text.push_str(text),
                Section::Variable(var_name) => {
                    if let Some(value) = values.get(*var_name) {
                        if data_values.contains(*var_name) {
                            // First add any accumulated text
                            if !current_text.is_empty() {
                                result.push(OutputSection::Text(current_text));
                                current_text = String::new();
                            }

                            // Add the data variable
                            let data_content = match value {
                                serde_json::Value::String(string_value) => string_value.clone(),
                                _ => value.to_string(),
                            };
                            result.push(OutputSection::Data(data_content));
                        } else {
                            // Regular variable, append to current text
                            match value {
                                serde_json::Value::String(string_value) => {
                                    current_text.push_str(string_value)
                                }
                                serde_json::Value::Number(number_value) => {
                                    if let Some(integer_value) = number_value.as_i64() {
                                        current_text.push_str(&integer_value.to_string());
                                    } else {
                                        current_text.push_str(&number_value.to_string());
                                    }
                                }
                                _ => {
                                    current_text.push_str(&value.to_string());
                                }
                            }
                        }
                    } else {
                        anyhow::bail!("Missing variable `{var_name}` in template expansion input");
                    }
                }
            }
        }

        // Add any remaining text
        if !current_text.is_empty() {
            result.push(OutputSection::Text(current_text));
        }

        Ok(result)
    }

    /// Get the list of variable names in the template.
    pub fn variables(&self) -> impl Iterator<Item = &'a str> {
        self.sections.iter().filter_map(|section| {
            if let Section::Variable(var_name) = section {
                Some(*var_name)
            } else {
                None
            }
        })
    }
}

/// Parse a variable of the form ${variableName}
fn parse_variable(input: &str) -> IResult<&str, Section> {
    let (remaining, var_name) = delimited(
        tag("${"),
        take_while1(|c: char| c.is_alphanumeric() || c == '_'),
        tag("}"),
    )
    .parse(input)?;

    Ok((remaining, Section::Variable(var_name)))
}

/// Parse text until we encounter a variable or end of input
fn parse_text(input: &str) -> IResult<&str, Section> {
    if input.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeUntil,
        )));
    }

    // If the string starts with "${", we should use the variable parser instead
    if input.starts_with("${") {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::TakeUntil,
        )));
    }

    // If the string doesn't contain "${", consume the whole string
    if !input.contains("${") {
        return Ok(("", Section::Text(input)));
    }

    // Otherwise, take all text until the next variable
    let (remaining, text) = take_until("${")(input)?;
    Ok((remaining, Section::Text(text)))
}

/// Parse a single section (either text or variable)
fn parse_section(input: &str) -> IResult<&str, Section> {
    alt((parse_variable, parse_text)).parse(input)
}

/// Parse the entire template into sections
fn parse_template(input: &str) -> IResult<&str, Vec<Section>> {
    let mut sections = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        match parse_section(remaining) {
            Ok((new_remaining, section)) => {
                sections.push(section);
                remaining = new_remaining;
            }
            Err(_) => break,
        }
    }

    Ok((remaining, sections))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plain_text() {
        let text = "This is plain text with no variables.";
        let template = Template::new(text);

        assert_eq!(template.sections.len(), 1);
        match &template.sections[0] {
            Section::Text(t) => assert_eq!(t, &text),
            Section::Variable(_) => panic!("Expected Text section"),
        }

        let result = template
            .expand(&serde_json::Map::new(), HashSet::new())
            .unwrap();
        assert_eq!(result.len(), 1);
        match &result[0] {
            OutputSection::Text(t) => assert_eq!(t, text),
            OutputSection::Data(_) => panic!("Expected Text section"),
        }
    }

    #[test]
    fn test_parse_single_variable() {
        let text = "Hello, ${name}!";
        let template = Template::new(text);

        assert_eq!(template.sections.len(), 3);

        // First section should be "Hello, "
        match &template.sections[0] {
            Section::Text(t) => assert_eq!(t, &"Hello, "),
            Section::Variable(_) => panic!("Expected Text section"),
        }

        // Second section should be the variable "name"
        match &template.sections[1] {
            Section::Variable(v) => assert_eq!(v, &"name"),
            Section::Text(_) => panic!("Expected Variable section"),
        }

        // Third section should be "!"
        match &template.sections[2] {
            Section::Text(t) => assert_eq!(t, &"!"),
            Section::Variable(_) => panic!("Expected Text section"),
        }

        let result = template
            .expand(
                serde_json::json!({"name": "abc"}).as_object().unwrap(),
                HashSet::new(),
            )
            .unwrap();
        assert_eq!(result.len(), 1);
        match &result[0] {
            OutputSection::Text(t) => assert_eq!(t, "Hello, abc!"),
            OutputSection::Data(_) => panic!("Expected Text section"),
        }

        match template.expand(
            serde_json::json!({"nam": "abc"}).as_object().unwrap(),
            HashSet::new(),
        ) {
            Ok(_) => panic!("Expected error due to missing variable"),
            Err(e) => assert_eq!(
                e.to_string(),
                "Missing variable `name` in template expansion input"
            ),
        };
    }

    #[test]
    fn test_parse_multiple_variables() {
        let text = "Hello, ${firstName} ${lastName}! How are you today?";
        let template = Template::new(text);

        assert_eq!(template.sections.len(), 5);

        match &template.sections[0] {
            Section::Text(t) => assert_eq!(t, &"Hello, "),
            _ => panic!("Expected Text section"),
        }

        match &template.sections[1] {
            Section::Variable(v) => assert_eq!(v, &"firstName"),
            _ => panic!("Expected Variable section"),
        }

        match &template.sections[2] {
            Section::Text(t) => assert_eq!(t, &" "),
            _ => panic!("Expected Text section"),
        }

        match &template.sections[3] {
            Section::Variable(v) => assert_eq!(v, &"lastName"),
            _ => panic!("Expected Variable section"),
        }

        match &template.sections[4] {
            Section::Text(t) => assert_eq!(t, &"! How are you today?"),
            _ => panic!("Expected Text section"),
        }
    }

    #[test]
    fn test_parse_consecutive_variables() {
        let text = "${greeting}${name}";
        let template = Template::new(text);

        assert_eq!(template.sections.len(), 2);

        match &template.sections[0] {
            Section::Variable(v) => assert_eq!(v, &"greeting"),
            _ => panic!("Expected Variable section"),
        }

        match &template.sections[1] {
            Section::Variable(v) => assert_eq!(v, &"name"),
            _ => panic!("Expected Variable section"),
        }
    }

    #[test]
    fn test_parse_empty_string() {
        let text = "";
        let template = Template::new(text);
        assert_eq!(template.sections.len(), 0);
    }

    #[test]
    fn test_data_variable() {
        let text = "Here is an image: ${image}";
        let template = Template::new(text);

        let mut data_values = HashSet::new();
        data_values.insert("image".to_string());

        let result = template
            .expand(
                serde_json::json!({"image": "data:image/png;base64,abc123"})
                    .as_object()
                    .unwrap(),
                data_values,
            )
            .unwrap();

        assert_eq!(result.len(), 2);
        match &result[0] {
            OutputSection::Text(t) => assert_eq!(t, "Here is an image: "),
            OutputSection::Data(_) => panic!("Expected Text section"),
        }
        match &result[1] {
            OutputSection::Data(d) => assert_eq!(d, "data:image/png;base64,abc123"),
            OutputSection::Text(_) => panic!("Expected Data section"),
        }
    }

    #[test]
    fn test_mixed_variables() {
        let text = "Person: ${name}, Avatar: ${avatar}, Age: ${age}";
        let template = Template::new(text);

        let mut data_values = HashSet::new();
        data_values.insert("avatar".to_string());

        let result = template
            .expand(
                serde_json::json!({
                    "name": "John",
                    "avatar": "data:image/jpeg;base64,xyz789",
                    "age": 30
                })
                .as_object()
                .unwrap(),
                data_values,
            )
            .unwrap();

        assert_eq!(result.len(), 3);
        match &result[0] {
            OutputSection::Text(t) => assert_eq!(t, "Person: John, Avatar: "),
            OutputSection::Data(_) => panic!("Expected Text section"),
        }
        match &result[1] {
            OutputSection::Data(d) => assert_eq!(d, "data:image/jpeg;base64,xyz789"),
            OutputSection::Text(_) => panic!("Expected Data section"),
        }
        match &result[2] {
            OutputSection::Text(t) => assert_eq!(t, ", Age: 30"),
            OutputSection::Data(_) => panic!("Expected Text section"),
        }
    }

    #[test]
    fn test_multiple_data_variables() {
        let text = "${image1} some text ${image2}";
        let template = Template::new(text);

        let mut data_values = HashSet::new();
        data_values.insert("image1".to_string());
        data_values.insert("image2".to_string());

        let result = template
            .expand(
                serde_json::json!({
                    "image1": "data:image/png;base64,abc123",
                    "image2": "data:image/png;base64,def456"
                })
                .as_object()
                .unwrap(),
                data_values,
            )
            .unwrap();

        assert_eq!(result.len(), 3);
        match &result[0] {
            OutputSection::Data(d) => assert_eq!(d, "data:image/png;base64,abc123"),
            OutputSection::Text(_) => panic!("Expected Data section"),
        }
        match &result[1] {
            OutputSection::Text(t) => assert_eq!(t, " some text "),
            OutputSection::Data(_) => panic!("Expected Text section"),
        }
        match &result[2] {
            OutputSection::Data(d) => assert_eq!(d, "data:image/png;base64,def456"),
            OutputSection::Text(_) => panic!("Expected Data section"),
        }
    }

    #[test]
    fn test_consecutive_data_variables() {
        let text = "${image1}${image2}";
        let template = Template::new(text);

        let mut data_values = HashSet::new();
        data_values.insert("image1".to_string());
        data_values.insert("image2".to_string());

        let result = template
            .expand(
                serde_json::json!({
                    "image1": "data:image/png;base64,abc123",
                    "image2": "data:image/png;base64,def456"
                })
                .as_object()
                .unwrap(),
                data_values,
            )
            .unwrap();

        assert_eq!(result.len(), 2);
        match &result[0] {
            OutputSection::Data(d) => assert_eq!(d, "data:image/png;base64,abc123"),
            OutputSection::Text(_) => panic!("Expected Data section"),
        }
        match &result[1] {
            OutputSection::Data(d) => assert_eq!(d, "data:image/png;base64,def456"),
            OutputSection::Text(_) => panic!("Expected Data section"),
        }
    }
}
