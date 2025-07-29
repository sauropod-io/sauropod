// Qwen3 output parser implementation

use super::{Event, ModelOutputParser, ParseState};

// Tag constants
const TOOL_CALL_OPEN_TAG: &str = "<tool_call>";
const TOOL_CALL_CLOSE_TAG: &str = "</tool_call>";
const THINK_OPEN_TAG: &str = "<think>";
const THINK_CLOSE_TAG: &str = "</think>";

// Tag length constants
const TOOL_CALL_OPEN_TAG_LEN: usize = TOOL_CALL_OPEN_TAG.len(); // 11
const TOOL_CALL_CLOSE_TAG_LEN: usize = TOOL_CALL_CLOSE_TAG.len(); // 12
const THINK_OPEN_TAG_LEN: usize = THINK_OPEN_TAG.len(); // 7
const THINK_CLOSE_TAG_LEN: usize = THINK_CLOSE_TAG.len(); // 8

pub struct Qwen3Parser {
    pub(crate) buffer: String,
    pub(crate) parse_state: ParseState,
}

impl ModelOutputParser for Qwen3Parser {
    fn parse<'a>(&mut self, input: &'a str) -> Vec<Event<'a>> {
        match self.parse_state {
            ParseState::RegularText => self.parse_regular_text(input),
            ParseState::InToolCall => self.parse_tool_call(input),
            ParseState::InReasoning => self.parse_reasoning(input),
        }
    }
}

impl Qwen3Parser {
    /// Parse regular text, looking for special tags
    pub(crate) fn parse_regular_text<'a>(&mut self, input: &'a str) -> Vec<Event<'a>> {
        let mut events = Vec::with_capacity(4); // Pre-allocate for common case
        let mut current_pos = 0;

        if current_pos < input.len() {
            let tool_pos = input[current_pos..].find(TOOL_CALL_OPEN_TAG);
            let think_pos = input[current_pos..].find(THINK_OPEN_TAG);

            let next_tag = match (tool_pos, think_pos) {
                (Some(tool), Some(think)) => {
                    if tool < think {
                        Some((
                            tool,
                            TOOL_CALL_OPEN_TAG,
                            TOOL_CALL_OPEN_TAG_LEN,
                            ParseState::InToolCall,
                        ))
                    } else {
                        Some((
                            think,
                            THINK_OPEN_TAG,
                            THINK_OPEN_TAG_LEN,
                            ParseState::InReasoning,
                        ))
                    }
                }
                (Some(tool), None) => Some((
                    tool,
                    TOOL_CALL_OPEN_TAG,
                    TOOL_CALL_OPEN_TAG_LEN,
                    ParseState::InToolCall,
                )),
                (None, Some(think)) => Some((
                    think,
                    THINK_OPEN_TAG,
                    THINK_OPEN_TAG_LEN,
                    ParseState::InReasoning,
                )),
                (None, None) => None,
            };

            if let Some((tag_pos, _tag_str, tag_len, new_state)) = next_tag {
                let absolute_tag_pos = current_pos + tag_pos;
                if absolute_tag_pos > current_pos {
                    events.push(Event::Text(&input[current_pos..absolute_tag_pos]));
                }
                self.parse_state = new_state;
                self.buffer.clear();
                current_pos = absolute_tag_pos + tag_len;
                let remaining = &input[current_pos..];
                match self.parse_state {
                    ParseState::InToolCall => events.extend(self.parse_tool_call(remaining)),
                    ParseState::InReasoning => events.extend(self.parse_reasoning(remaining)),
                    ParseState::RegularText => unreachable!(),
                }
                return events;
            } else if current_pos < input.len() {
                events.push(Event::Text(&input[current_pos..]));
            }
        }
        events
    }

    /// Parse tool call content, looking for the closing tag
    pub(crate) fn parse_tool_call<'a>(&mut self, input: &'a str) -> Vec<Event<'a>> {
        let mut events = Vec::with_capacity(3);
        if let Some(end_pos) = input.find(TOOL_CALL_CLOSE_TAG) {
            let content = &input[..end_pos];
            if !content.is_empty() {
                events.push(Event::ToolCall(content));
            }
            events.push(Event::ToolCallEnd);
            self.parse_state = ParseState::RegularText;
            self.buffer.clear();
            let remaining = &input[end_pos + TOOL_CALL_CLOSE_TAG_LEN..];
            if !remaining.is_empty() {
                events.extend(self.parse_regular_text(remaining));
            }
        } else if !input.is_empty() {
            events.push(Event::ToolCall(input));
        }
        events
    }

    /// Parse reasoning content, looking for the closing tag
    pub(crate) fn parse_reasoning<'a>(&mut self, input: &'a str) -> Vec<Event<'a>> {
        let mut events = Vec::with_capacity(3);
        if let Some(end_pos) = input.find(THINK_CLOSE_TAG) {
            let content = &input[..end_pos];
            if !content.is_empty() {
                events.push(Event::Reasoning(content));
            }
            events.push(Event::ReasoningEnd);
            self.parse_state = ParseState::RegularText;
            self.buffer.clear();
            let remaining = &input[end_pos + THINK_CLOSE_TAG_LEN..];
            if !remaining.is_empty() {
                events.extend(self.parse_regular_text(remaining));
            }
        } else {
            events.push(Event::Reasoning(input));
        }
        events
    }
}

#[cfg(test)]
mod test {
    use crate::{Event, ModelType, get_model_parser};

    const QWEN3_TOOL_CALL: &str =
        r#"<tool_call>\n{"name": "test_tool", "arguments": {}}\n</tool_call>"#;

    #[test]
    fn qwen3_tool_call() {
        let mut parser = get_model_parser(ModelType::Qwen3);
        let events = parser.parse(&QWEN3_TOOL_CALL[0..15]);
        assert_eq!(events.len(), 1);
        match &events[0] {
            Event::ToolCall(json) => {
                assert_eq!(*json, r#"\n{""#);
            }
            _ => panic!("Expected tool call event, got {:?}", events[0]),
        }

        let next_events = parser.parse(&QWEN3_TOOL_CALL[15..]);
        match &next_events[0] {
            Event::ToolCall(json) => {
                assert_eq!(*json, r#"name": "test_tool", "arguments": {}}\n"#);
            }
            _ => panic!("Expected tool call event, got {:?}", next_events[0]),
        };
        assert_eq!(next_events[1], Event::ToolCallEnd);
    }

    #[test]
    fn qwen3_reasoning() {
        let mut parser = get_model_parser(ModelType::Qwen3);
        let reasoning_text = "<think>Let me think about this problem step by step.</think>";

        assert_eq!(
            parser.parse(reasoning_text),
            vec![
                Event::Reasoning("Let me think about this problem step by step."),
                Event::ReasoningEnd
            ]
        );
    }

    #[test]
    fn qwen3_reasoning_streaming() {
        let mut parser = get_model_parser(ModelType::Qwen3);

        assert_eq!(
            parser.parse("<think>Let me think about"),
            vec![Event::Reasoning("Let me think about")]
        );
        assert_eq!(
            parser.parse(" this problem step by step.</think>"),
            vec![
                Event::Reasoning(" this problem step by step."),
                Event::ReasoningEnd
            ]
        );
        assert_eq!(
            parser.parse(" I have a solution."),
            vec![Event::Text(" I have a solution.")]
        );
    }

    #[test]
    fn qwen3_reasoning_truncated() {
        let mut parser = get_model_parser(ModelType::Qwen3);
        assert_eq!(parser.parse("<think>"), vec![Event::Reasoning("")]);
    }

    #[test]
    fn qwen3_reasoning_about_tool_call() {
        let mut parser = get_model_parser(ModelType::Qwen3);
        let reasoning_text = "<think>I should use a <tool_call>abc</tool_call>.</think>";

        assert_eq!(
            parser.parse(reasoning_text),
            vec![
                Event::Reasoning("I should use a <tool_call>abc</tool_call>."),
                Event::ReasoningEnd
            ]
        );
    }
    #[test]
    fn qwen3_tool_call_with_reasoning_tokens_inside() {
        let mut parser = get_model_parser(ModelType::Qwen3);

        assert_eq!(
            parser.parse(
                r#"<tool_call>{\"param\": \"<think>I should use a abc</think>\"}</tool_call>"#
            ),
            vec![
                Event::ToolCall(r#"{\"param\": \"<think>I should use a abc</think>\"}"#),
                Event::ToolCallEnd
            ]
        );
    }

    #[test]
    fn qwen3_tool_call_with_reasoning_tokens_inside_streaming() {
        let mut parser = get_model_parser(ModelType::Qwen3);

        assert_eq!(
            parser.parse(r#"<tool_call>{\"param\": \"<think>I should"#),
            vec![Event::ToolCall(r#"{\"param\": \"<think>I should"#)]
        );
        assert_eq!(
            parser.parse(r#" a abc</think>"}</tool_call>"#),
            vec![Event::ToolCall(r#" a abc</think>"}"#), Event::ToolCallEnd]
        );
    }

    #[test]
    fn mixed_content() {
        let mut parser = get_model_parser(ModelType::Qwen3);
        let mixed_content = r#"Here is some text <think>thinking</think> and then <tool_call>\n{"name": "calculator", "arguments": {"expression": "2+2"}}\n</tool_call>"#;

        let events = parser.parse(mixed_content);
        assert_eq!(events.len(), 6);

        match &events[0] {
            Event::Text(content) => assert_eq!(*content, "Here is some text "),
            _ => panic!("Expected text event, got {:?}", events[0]),
        }

        match &events[1] {
            Event::Reasoning(content) => assert_eq!(*content, "thinking"),
            _ => panic!("Expected reasoning event, got {:?}", events[1]),
        }

        assert_eq!(events[2], Event::ReasoningEnd);

        match &events[3] {
            Event::Text(content) => assert_eq!(*content, " and then "),
            _ => panic!("Expected text event, got {:?}", events[3]),
        }

        match &events[4] {
            Event::ToolCall(json) => {
                assert_eq!(
                    *json,
                    r#"\n{"name": "calculator", "arguments": {"expression": "2+2"}}\n"#
                );
            }
            _ => panic!("Expected tool call event, got {:?}", events[4]),
        }

        assert_eq!(events[5], Event::ToolCallEnd);
    }

    #[test]
    fn streaming_tool_call() {
        let mut parser = get_model_parser(ModelType::Qwen3);
        assert_eq!(parser.parse("<tool_call>"), vec![]);
        assert_eq!(
            parser.parse("\\n{\"name\": \"test\""),
            vec![Event::ToolCall("\\n{\"name\": \"test\"")]
        );
        assert_eq!(
            parser.parse(", \"arguments\": {}}"),
            vec![Event::ToolCall(", \"arguments\": {}}")]
        );
        let events = parser.parse("\\n</tool_call>");
        assert_eq!(events.len(), 2);
        match &events[0] {
            Event::ToolCall(json) => {
                assert_eq!(*json, "\\n");
            }
            _ => panic!("Expected tool call event"),
        }
        assert_eq!(events[1], Event::ToolCallEnd);
    }
}
