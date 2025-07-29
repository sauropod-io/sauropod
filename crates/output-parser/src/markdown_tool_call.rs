// Markdown tool call parser implementation

use super::{Event, ModelOutputParser, ParseState};

const TOOL_CALL_OPEN_TAG: &str = "```tool_call";
const TOOL_CALL_OPEN_TAG_LEN: usize = TOOL_CALL_OPEN_TAG.len();
const TOOL_CALL_CLOSE_TAG: &str = "```";
const TOOL_CALL_CLOSE_TAG_LEN: usize = TOOL_CALL_CLOSE_TAG.len();

pub struct MarkdownToolCallParser {
    pub(crate) buffer: String,
    pub(crate) parse_state: ParseState,
}

impl ModelOutputParser for MarkdownToolCallParser {
    fn parse<'a>(&mut self, input: &'a str) -> Vec<Event<'a>> {
        match self.parse_state {
            ParseState::RegularText => self.parse_regular_text(input),
            ParseState::InToolCall => self.parse_tool_call(input),
            ParseState::InReasoning => unreachable!(),
        }
    }
}

impl MarkdownToolCallParser {
    /// Parse regular text looking for the opening tool call fence.  The parser
    /// is optimized for streaming and therefore may receive the opening fence
    /// split across multiple calls.  The `buffer` field is used to store any
    /// partial fence between calls.
    pub(crate) fn parse_regular_text<'a>(&mut self, input: &'a str) -> Vec<Event<'a>> {
        let mut events = Vec::new();

        // If we already buffered part of the opening tag, append the new input
        // and check again.
        if !self.buffer.is_empty() {
            self.buffer.push_str(input);
            if TOOL_CALL_OPEN_TAG.starts_with(&self.buffer)
                && self.buffer.len() < TOOL_CALL_OPEN_TAG_LEN
            {
                // Still waiting for the rest of the tag
                return events;
            }

            if self.buffer.starts_with(TOOL_CALL_OPEN_TAG) {
                // Determine how many bytes of `input` belonged to the tag so we
                // can continue parsing the remainder as tool call content.
                let consumed = TOOL_CALL_OPEN_TAG_LEN - (self.buffer.len() - input.len());
                self.parse_state = ParseState::InToolCall;
                self.buffer.clear();
                if consumed < input.len() {
                    events.extend(self.parse_tool_call(&input[consumed..]));
                }
                return events;
            }

            // Not actually a tag; clear buffer and fall back to normal parsing
            self.buffer.clear();
        }

        let mut current_pos = 0;
        if current_pos < input.len() {
            if let Some(tag_pos) = input[current_pos..].find(TOOL_CALL_OPEN_TAG) {
                let absolute = current_pos + tag_pos;
                if absolute > current_pos {
                    events.push(Event::Text(&input[current_pos..absolute]));
                }
                self.parse_state = ParseState::InToolCall;
                current_pos = absolute + TOOL_CALL_OPEN_TAG_LEN;
                let remaining = &input[current_pos..];
                events.extend(self.parse_tool_call(remaining));
                return events;
            } else {
                let remaining = &input[current_pos..];
                if TOOL_CALL_OPEN_TAG.starts_with(remaining) {
                    // Potential start of the tag; store for the next call
                    self.buffer.push_str(remaining);
                } else if !remaining.is_empty() {
                    events.push(Event::Text(remaining));
                } else {
                    events.push(Event::Text(""));
                }
            }
        }

        if input.is_empty() && events.is_empty() {
            events.push(Event::Text(""));
        }

        events
    }

    /// Parse tool call content.  When the closing fence is encountered a
    /// `ToolCallEnd` event is emitted and the parser returns to regular text
    /// mode.
    pub(crate) fn parse_tool_call<'a>(&mut self, input: &'a str) -> Vec<Event<'a>> {
        let mut events = Vec::new();
        if let Some(end_pos) = input.find(TOOL_CALL_CLOSE_TAG) {
            let content = &input[..end_pos];
            if !content.is_empty() {
                events.push(Event::ToolCall(content));
            }
            events.push(Event::ToolCallEnd);
            self.parse_state = ParseState::RegularText;
            let remaining = &input[end_pos + TOOL_CALL_CLOSE_TAG_LEN..];
            if !remaining.is_empty() {
                events.extend(self.parse_regular_text(remaining));
            }
        } else if !input.is_empty() {
            events.push(Event::ToolCall(input));
        }
        events
    }
}

#[cfg(test)]
mod test {
    use crate::Event;

    #[test]
    fn parse_regular_text() {
        let mut parser = crate::get_model_parser(crate::ModelType::MarkdownToolCall);
        assert_eq!(parser.parse("hello"), vec![Event::Text("hello")]);
        assert_eq!(parser.parse(""), vec![Event::Text("")]);
    }

    #[test]
    fn parse_tool_call() {
        let mut parser = crate::get_model_parser(crate::ModelType::MarkdownToolCall);
        assert_eq!(
            parser.parse("```tool_call\nhello\n```"),
            vec![Event::ToolCall("\nhello\n"), Event::ToolCallEnd]
        );
        assert_eq!(
            parser.parse("```tool_call\n```"),
            vec![Event::ToolCall("\n"), Event::ToolCallEnd]
        );
    }

    #[test]
    fn parse_tool_call_streaming() {
        let mut parser = crate::get_model_parser(crate::ModelType::MarkdownToolCall);
        assert_eq!(parser.parse("```tool_call"), Vec::<Event>::new());
        assert_eq!(parser.parse("\nhe"), vec![Event::ToolCall("\nhe")]);
        assert_eq!(
            parser.parse("llo\n```abc"),
            vec![
                Event::ToolCall("llo\n"),
                Event::ToolCallEnd,
                Event::Text("abc")
            ]
        );
    }

    #[test]
    fn parse_tool_call_streaming_2() {
        let mut parser = crate::get_model_parser(crate::ModelType::MarkdownToolCall);
        let texts = &[
            "```",
            "tool",
            "_",
            "call",
            "\n",
            "{\"",
            "name",
            "\":",
            " \"",
            "test",
            "_",
            "tool",
            "\",",
            " \"",
            "parameters",
            "\":",
            " {",
            "}}",
            "\n",
            "```",
        ];
        let mut events = Vec::new();
        for text in texts {
            events.extend(parser.parse(text));
        }
        assert_eq!(
            events,
            vec![
                Event::ToolCall("\n"),
                Event::ToolCall("{\""),
                Event::ToolCall("name"),
                Event::ToolCall("\":"),
                Event::ToolCall(" \""),
                Event::ToolCall("test"),
                Event::ToolCall("_"),
                Event::ToolCall("tool"),
                Event::ToolCall("\","),
                Event::ToolCall(" \""),
                Event::ToolCall("parameters"),
                Event::ToolCall("\":"),
                Event::ToolCall(" {"),
                Event::ToolCall("}}"),
                Event::ToolCall("\n"),
                Event::ToolCallEnd,
            ]
        );
    }
}
