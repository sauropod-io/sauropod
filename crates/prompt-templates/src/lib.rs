//! Parsing library for prompt templates.

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
    ) -> anyhow::Result<String> {
        let mut result = String::with_capacity(
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
                Section::Text(text) => result.push_str(text),
                Section::Variable(var_name) => {
                    if let Some(value) = values.get(*var_name) {
                        match value {
                            serde_json::Value::String(string_value) => {
                                result.push_str(string_value)
                            }
                            serde_json::Value::Number(number_value) => {
                                if let Some(integer_value) = number_value.as_i64() {
                                    result.push_str(&integer_value.to_string());
                                } else {
                                    result.push_str(&number_value.to_string());
                                }
                            }
                            _ => {
                                result.push_str(&value.to_string());
                            }
                        }
                    } else {
                        anyhow::bail!("Missing variable `{var_name}` in template expansion input");
                    }
                }
            }
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

        assert_eq!(template.expand(&serde_json::Map::new()).unwrap(), text);
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

        assert_eq!(
            template
                .expand(serde_json::json!({"name": "abc"}).as_object().unwrap())
                .unwrap(),
            "Hello, abc!"
        );
        match template.expand(serde_json::json!({"nam": "abc"}).as_object().unwrap()) {
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
}
