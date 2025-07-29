//! LLM output parsing tool.

mod qwen;
use qwen::Qwen3Parser;

mod markdown_tool_call;
use markdown_tool_call::MarkdownToolCallParser;

/// The type of a model.
#[derive(Debug, Clone, Copy)]
pub enum ModelType {
    /// Qwen 3 model
    Qwen3,
    /// Models like Gemma 3 that have no reasoning and use a Markdown block for tool calls
    MarkdownToolCall,
    /// Fallback parser for unknown models
    Unknown,
}

/// Events that can be emitted by the parser.
#[derive(Debug, Clone, PartialEq)]
pub enum Event<'a> {
    /// A reasoning step by the model
    Reasoning(&'a str),
    /// End of a reasoning section
    ReasoningEnd,
    /// A tool call with JSON data
    ToolCall(&'a str),
    /// End of a tool call
    ToolCallEnd,
    /// Regular text output
    Text(&'a str),
}

enum ParseState {
    /// Parsing a tool call
    InToolCall,
    /// Parsing a reasoning section
    InReasoning,
    /// At the top level parsing regular text
    RegularText,
}

pub trait ModelOutputParser: Send {
    /// Parse the input text and return any complete events.
    fn parse<'a>(&mut self, input: &'a str) -> Vec<Event<'a>>;
}

/// A parser that emits all input as text events without any special handling.
pub struct NoOpParser;

impl ModelOutputParser for NoOpParser {
    fn parse<'a>(&mut self, input: &'a str) -> Vec<Event<'a>> {
        vec![Event::Text(input)]
    }
}

/// Get a model parser for the specified model type.
pub fn get_model_parser(model_type: ModelType) -> Box<dyn ModelOutputParser> {
    match model_type {
        ModelType::Qwen3 => Box::new(Qwen3Parser {
            buffer: String::with_capacity(16),
            parse_state: ParseState::RegularText,
        }),
        ModelType::MarkdownToolCall => Box::new(MarkdownToolCallParser {
            buffer: String::with_capacity(16),
            parse_state: ParseState::RegularText,
        }),
        ModelType::Unknown => Box::new(NoOpParser),
    }
}

#[cfg(test)]
mod test {
    use crate::Event;

    #[test]
    fn noop_parser_emits_text() {
        let mut parser = super::get_model_parser(super::ModelType::Unknown);
        assert_eq!(parser.parse("hello"), vec![Event::Text("hello")]);
        assert_eq!(parser.parse(""), vec![Event::Text("")]);
    }
}
