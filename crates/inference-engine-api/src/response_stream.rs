/// Helper function to get the next sequence number from a RefCell.
fn get_next_sequence_number(sequence_number: &std::cell::RefCell<i64>) -> i64 {
    let seq = *sequence_number.borrow();
    *sequence_number.borrow_mut() += 1;
    seq
}

/// A utility to track the state of a response stream and create new events.
pub struct ResponseStreamCreator {
    /// The response that's being constructed.
    pub response: sauropod_openai_api::Response,
    /// The current output index.
    output_index: i64,
    /// The current content index.
    content_index: i64,
    /// Whether response created has been sent.
    response_created: bool,
    /// Whether there's currently an open content part.
    content_part_open: bool,
    /// Whether there's currently an open output item.
    output_item_open: bool,
    /// The sequence number of the next event.
    sequence_number: std::cell::RefCell<i64>,
    parser: Box<dyn sauropod_output_parser::ModelOutputParser>,
    reasoning_state: Option<ReasoningState>,
    tool_call_state: Option<ToolCallState>,
}

struct ReasoningState {
    item_id: String,
}

struct ToolCallState {
    item_id: String,
    buffer: String,
}

impl ResponseStreamCreator {
    /// Creates a new instance of the response stream creator.
    pub fn new(
        output_parser: Box<dyn sauropod_output_parser::ModelOutputParser>,
        mut initial_response: sauropod_openai_api::Response,
    ) -> Self {
        initial_response.usage = Some(sauropod_openai_api::ResponseUsage {
            input_tokens: 0,
            input_tokens_details: sauropod_openai_api::ResponseUsageInputTokensDetails {
                cached_tokens: 0,
            },
            output_tokens: 0,
            output_tokens_details: sauropod_openai_api::ResponseUsageOutputTokensDetails {
                reasoning_tokens: 0,
            },
            total_tokens: 0,
        });

        Self {
            response: initial_response,
            output_index: 0,
            content_index: 0,
            response_created: false,
            content_part_open: false,
            output_item_open: false,
            sequence_number: std::cell::RefCell::new(0),
            parser: output_parser,
            reasoning_state: None,
            tool_call_state: None,
        }
    }

    /// Whether the response is empty.
    pub fn is_empty(&self) -> bool {
        self.response.output.is_empty()
    }

    fn ensure_response_created(
        &mut self,
        events: &mut Vec<sauropod_openai_api::ResponseStreamEvent>,
    ) {
        if !self.response_created {
            self.response.status = Some(sauropod_openai_api::ResponseStatus::InProgress);
            events.push(
                sauropod_openai_api::ResponseStreamEvent::ResponseCreatedEvent {
                    sequence_number: get_next_sequence_number(&self.sequence_number),
                    response: self.response.clone(),
                },
            );
            events.push(
                sauropod_openai_api::ResponseStreamEvent::ResponseInProgressEvent {
                    sequence_number: get_next_sequence_number(&self.sequence_number),
                    response: self.response.clone(),
                },
            );
            self.response_created = true;
        }
    }

    /// Pushes a text delta to the current message, creating output items and content parts as needed.
    /// Returns the events that should be emitted for this text push.
    fn push_text_internal(
        &mut self,
        text: String,
    ) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let mut events: Vec<sauropod_openai_api::ResponseStreamEvent> = Vec::with_capacity(2);

        self.ensure_response_created(&mut events);

        // Ensure we have an output message at the current index
        if self.response.output.len() <= self.output_index as usize {
            let message_id = uuid::Uuid::new_v4().to_string();
            let output_item = sauropod_openai_api::OutputItem::OutputMessage {
                id: message_id.clone(),
                content: Vec::new(),
                role: sauropod_openai_api::OutputMessageRole::Assistant,
                status: sauropod_openai_api::Status::InProgress,
            };

            self.response.output.push(output_item.clone());

            // Emit output item added event
            events.push(
                sauropod_openai_api::ResponseStreamEvent::ResponseOutputItemAddedEvent {
                    sequence_number: get_next_sequence_number(&self.sequence_number),
                    item: output_item,
                    output_index: self.output_index,
                },
            );

            // Mark output item as open and reset content index for new output item
            self.output_item_open = true;
            self.content_index = 0;
        }

        // Get the current message
        let message_id = match &mut self.response.output[self.output_index as usize] {
            sauropod_openai_api::OutputItem::OutputMessage { id, content, .. } => {
                // Ensure we have a text content part at the current index
                if content.len() <= self.content_index as usize {
                    let output_content = sauropod_openai_api::OutputContent::OutputTextContent {
                        logprobs: None,
                        text: String::new(),
                        annotations: Vec::new(),
                    };
                    content.push(output_content.clone());

                    // Emit content part added event
                    events.push(
                        sauropod_openai_api::ResponseStreamEvent::ResponseContentPartAddedEvent {
                            sequence_number: get_next_sequence_number(&self.sequence_number),
                            content_index: self.content_index,
                            item_id: id.clone(),
                            output_index: self.output_index,
                            part: output_content,
                        },
                    );

                    // Mark content part as open
                    self.content_part_open = true;
                }

                // Append the text to the existing content
                if let Some(sauropod_openai_api::OutputContent::OutputTextContent {
                    text: existing_text,
                    ..
                }) = content.get_mut(self.content_index as usize)
                {
                    existing_text.push_str(&text);
                }

                id.clone()
            }
            _ => {
                // This shouldn't happen if we created a message above, but handle it gracefully
                tracing::warn!(
                    "Expected OutputMessage at output index {}, found {:?}",
                    self.output_index,
                    self.response.output[self.output_index as usize]
                );
                return events;
            }
        };

        // Emit text delta event unless the text is empty. When pushing an empty
        // string we still want to open the content part so that it can later be
        // closed, but there is no delta to emit.
        if !text.is_empty() {
            events.push(
                sauropod_openai_api::ResponseStreamEvent::ResponseTextDeltaEvent {
                    sequence_number: get_next_sequence_number(&self.sequence_number),
                    content_index: self.content_index,
                    delta: text,
                    item_id: message_id,
                    output_index: self.output_index,
                    logprobs: vec![],
                },
            );
        }

        events
    }

    /// Parses the provided text for reasoning and tool calls and emits the appropriate events.
    pub fn push_text(&mut self, text: String) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        // If the text is empty the parser will return no events. We still want
        // to open the output item and content part so that they can be closed
        // later, but we shouldn't emit a text delta event. Call
        // `push_text_internal` directly in this case.
        let mut events = Vec::new();
        let parsed_events = self.parser.parse(&text);
        if parsed_events.is_empty() && text.is_empty() {
            events.extend(self.push_text_internal(text));
            return events;
        }

        for event in parsed_events {
            match event {
                sauropod_output_parser::Event::Text(t) => {
                    events.extend(self.push_text_internal(t.to_string()));
                }
                sauropod_output_parser::Event::Reasoning(t) => {
                    events.extend(self.push_reasoning_delta(t));
                }
                sauropod_output_parser::Event::ReasoningEnd => {
                    events.extend(self.finish_reasoning());
                }
                sauropod_output_parser::Event::ToolCall(t) => {
                    events.extend(self.push_tool_call_delta(t));
                }
                sauropod_output_parser::Event::ToolCallEnd => {
                    events.extend(self.finish_tool_call());
                }
            }
        }
        events
    }

    /// Call push text and update the token content in the internal response state.
    pub fn push_part(&mut self, part: String) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let Some(usage) = self.response.usage.as_mut() else {
            unreachable!()
        };
        usage.output_tokens += 1;
        usage.total_tokens += 1;
        self.push_text(part)
    }

    /// Closes the current content part if one is open and emits a ResponseContentPartDoneEvent.
    /// Returns the events that should be emitted.
    pub fn close_current_content_part(&mut self) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let mut events = Vec::new();

        if !self.content_part_open {
            return events;
        }

        // Get the current message and content part
        if let Some(sauropod_openai_api::OutputItem::OutputMessage { id, content, .. }) =
            self.response.output.get(self.output_index as usize)
            && let Some(output_content) = content.get(self.content_index as usize)
        {
            // If this is a text content part with non-empty text, emit ResponseTextDoneEvent first
            if let sauropod_openai_api::OutputContent::OutputTextContent { text, .. } =
                output_content
                && !text.is_empty()
            {
                events.push(
                    sauropod_openai_api::ResponseStreamEvent::ResponseTextDoneEvent {
                        sequence_number: get_next_sequence_number(&self.sequence_number),
                        content_index: self.content_index,
                        item_id: id.clone(),
                        output_index: self.output_index,
                        text: text.clone(),
                        logprobs: vec![],
                    },
                );
            }

            // Emit content part done event
            events.push(
                sauropod_openai_api::ResponseStreamEvent::ResponseContentPartDoneEvent {
                    sequence_number: get_next_sequence_number(&self.sequence_number),
                    content_index: self.content_index,
                    item_id: id.clone(),
                    output_index: self.output_index,
                    part: output_content.clone(),
                },
            );

            self.content_part_open = false;
        }

        events
    }

    /// Closes the current output item if one is open and emits a ResponseOutputItemDoneEvent.
    /// Returns the events that should be emitted.
    pub fn close_current_output_item(&mut self) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let mut events = Vec::new();

        if !self.output_item_open {
            return events;
        }

        // Get the current output item
        let maybe_output_item =
            if let Some(output_item) = self.response.output.get_mut(self.output_index as usize) {
                match output_item {
                    sauropod_openai_api::OutputItem::OutputMessage { status, .. } => {
                        *status = sauropod_openai_api::Status::Completed;
                    }
                    sauropod_openai_api::OutputItem::ReasoningItem { status, .. } => {
                        *status = Some(sauropod_openai_api::Status::Completed);
                    }
                    sauropod_openai_api::OutputItem::FunctionToolCall { status, .. } => {
                        *status = Some(sauropod_openai_api::Status::Completed);
                    }
                    _ => {}
                }

                self.output_item_open = false;
                Some(output_item.clone())
            } else {
                None
            };

        if let Some(item) = maybe_output_item {
            // Emit output item done event
            events.push(
                sauropod_openai_api::ResponseStreamEvent::ResponseOutputItemDoneEvent {
                    sequence_number: get_next_sequence_number(&self.sequence_number),
                    item,
                    output_index: self.output_index,
                },
            );
        }

        events
    }

    fn push_reasoning_delta(
        &mut self,
        delta: &str,
    ) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let mut events = Vec::new();
        if self.reasoning_state.is_none() {
            events.extend(self.open_reasoning_item());
        }

        let state = self.reasoning_state.as_ref().unwrap();
        if let Some(sauropod_openai_api::OutputItem::ReasoningItem { summary, .. }) =
            self.response.output.get_mut(self.output_index as usize)
            && let Some(part) = summary.get_mut(self.content_index as usize)
        {
            part.text.push_str(delta);
        }
        events.push(
            sauropod_openai_api::ResponseStreamEvent::ResponseReasoningSummaryTextDeltaEvent {
                sequence_number: get_next_sequence_number(&self.sequence_number),
                delta: delta.to_string(),
                item_id: state.item_id.clone(),
                output_index: self.output_index,
                summary_index: self.content_index,
            },
        );
        events
    }

    fn finish_reasoning(&mut self) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let mut events = Vec::new();
        let Some(state) = self.reasoning_state.take() else {
            return events;
        };

        if let Some(sauropod_openai_api::OutputItem::ReasoningItem {
            summary, status, ..
        }) = self.response.output.get_mut(self.output_index as usize)
        {
            if let Some(part) = summary.get(self.content_index as usize) {
                events.push(sauropod_openai_api::ResponseStreamEvent::ResponseReasoningSummaryTextDoneEvent {
                    sequence_number: get_next_sequence_number(&self.sequence_number),
                    item_id: state.item_id.clone(),
                    output_index: self.output_index,
                    summary_index: self.content_index,
                    text: part.text.clone(),
                });
                events.push(sauropod_openai_api::ResponseStreamEvent::ResponseReasoningSummaryPartDoneEvent {
                    sequence_number: get_next_sequence_number(&self.sequence_number),
                    item_id: state.item_id.clone(),
                    output_index: self.output_index,
                    summary_index: self.content_index,
                    part: sauropod_openai_api::ResponseReasoningSummaryPartDoneEventPart {
                        text: part.text.clone(),
                        r#type: sauropod_openai_api::ResponseReasoningSummaryPartDoneEventPartType::SummaryText,
                    },
                });
            }
            *status = Some(sauropod_openai_api::Status::Completed);
        }

        events.extend(self.close_current_output_item());
        self.output_index += 1;
        events
    }

    fn push_tool_call_delta(
        &mut self,
        delta: &str,
    ) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let mut events = Vec::new();
        if self.tool_call_state.is_none() {
            events.extend(self.open_tool_call_item());
        }

        if let Some(state) = self.tool_call_state.as_mut() {
            state.buffer.push_str(delta);
            if let Some(sauropod_openai_api::OutputItem::FunctionToolCall { arguments, .. }) =
                self.response.output.get_mut(self.output_index as usize)
            {
                arguments.push_str(delta);
            }
            events.push(
                sauropod_openai_api::ResponseStreamEvent::ResponseFunctionCallArgumentsDeltaEvent {
                    sequence_number: get_next_sequence_number(&self.sequence_number),
                    delta: delta.to_string(),
                    item_id: state.item_id.clone(),
                    output_index: self.output_index,
                },
            );
        }
        events
    }

    fn finish_tool_call(&mut self) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let mut events = Vec::new();
        let Some(state) = self.tool_call_state.take() else {
            return events;
        };

        let mut args_string = state.buffer.clone();
        let mut name = String::new();
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&state.buffer)
            && let Some(obj) = val.as_object()
        {
            if let Some(n) = obj.get("name").and_then(|v| v.as_str()) {
                name = n.to_string();
            }
            if let Some(arg) = obj.get("arguments") {
                args_string = arg.to_string();
            }
        }

        if let Some(sauropod_openai_api::OutputItem::FunctionToolCall {
            arguments,
            name: item_name,
            status,
            ..
        }) = self.response.output.get_mut(self.output_index as usize)
        {
            *arguments = args_string.clone();
            *item_name = name;
            *status = Some(sauropod_openai_api::Status::Completed);
        }

        events.push(
            sauropod_openai_api::ResponseStreamEvent::ResponseFunctionCallArgumentsDoneEvent {
                sequence_number: get_next_sequence_number(&self.sequence_number),
                arguments: args_string,
                item_id: state.item_id.clone(),
                output_index: self.output_index,
            },
        );

        events.extend(self.close_current_output_item());
        self.output_index += 1;
        events
    }

    fn open_reasoning_item(&mut self) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let mut events = Vec::new();
        self.ensure_response_created(&mut events);
        events.extend(self.close_current_content_part());
        events.extend(self.close_current_output_item());

        let item_id = uuid::Uuid::new_v4().to_string();
        let reasoning_item = sauropod_openai_api::OutputItem::ReasoningItem {
            encrypted_content: None,
            id: item_id.clone(),
            content: None, // TODO
            status: Some(sauropod_openai_api::Status::InProgress),
            summary: vec![sauropod_openai_api::ReasoningItemSummaryItem {
                text: String::new(),
                r#type: sauropod_openai_api::ReasoningItemSummaryItemType::SummaryText,
            }],
        };

        self.response.output.push(reasoning_item.clone());
        self.output_index = (self.response.output.len() - 1) as i64;
        self.output_item_open = true;
        self.content_index = 0;

        events.push(
            sauropod_openai_api::ResponseStreamEvent::ResponseOutputItemAddedEvent {
                sequence_number: get_next_sequence_number(&self.sequence_number),
                item: reasoning_item,
                output_index: self.output_index,
            },
        );

        events.push(sauropod_openai_api::ResponseStreamEvent::ResponseReasoningSummaryPartAddedEvent {
            sequence_number: get_next_sequence_number(&self.sequence_number),
            item_id: item_id.clone(),
            output_index: self.output_index,
            summary_index: self.content_index,
            part: sauropod_openai_api::ResponseReasoningSummaryPartAddedEventPart {
                text: String::new(),
                r#type: sauropod_openai_api::ResponseReasoningSummaryPartAddedEventPartType::SummaryText,
            },
        });

        self.reasoning_state = Some(ReasoningState { item_id });
        events
    }

    fn open_tool_call_item(&mut self) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let mut events = Vec::new();
        self.ensure_response_created(&mut events);
        events.extend(self.close_current_content_part());
        events.extend(self.close_current_output_item());

        let call_id = uuid::Uuid::new_v4().to_string();
        let function_item = sauropod_openai_api::OutputItem::FunctionToolCall {
            arguments: String::new(),
            call_id: call_id.clone(),
            id: call_id.clone(),
            name: String::new(),
            status: Some(sauropod_openai_api::Status::InProgress),
        };

        self.response.output.push(function_item.clone());
        self.output_index = (self.response.output.len() - 1) as i64;
        self.output_item_open = true;
        self.content_index = 0;

        events.push(
            sauropod_openai_api::ResponseStreamEvent::ResponseOutputItemAddedEvent {
                sequence_number: get_next_sequence_number(&self.sequence_number),
                item: function_item,
                output_index: self.output_index,
            },
        );

        self.tool_call_state = Some(ToolCallState {
            item_id: call_id,
            buffer: String::new(),
        });
        events
    }

    /// Finishes the current response and returns the final response with all output items.
    pub fn finish(&mut self) -> Vec<sauropod_openai_api::ResponseStreamEvent> {
        let mut events = Vec::new();

        // Close any open content part first
        events.extend(self.close_current_content_part());

        // Close any open output item
        events.extend(self.close_current_output_item());

        self.response.status = Some(sauropod_openai_api::ResponseStatus::Completed);

        events.push(
            sauropod_openai_api::ResponseStreamEvent::ResponseCompletedEvent {
                sequence_number: get_next_sequence_number(&self.sequence_number),
                response: self.response.clone(),
            },
        );
        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sauropod_openai_api::{
        ModelResponseProperties, OutputItem, OutputMessageRole, Response, ResponseObject,
        ResponseProperties, ResponseStatus, Status,
    };

    fn create_test_response() -> Response {
        Response {
            instructions: None,
            model_response_properties: ModelResponseProperties::default(),
            response_properties: ResponseProperties::default(),
            created_at: 1234567890,
            error: None,
            id: "test_response".to_string(),
            incomplete_details: None,
            object: ResponseObject::Response,
            output: Vec::new(),
            parallel_tool_calls: false,
            status: Some(ResponseStatus::InProgress),
            usage: None,
        }
    }

    #[test]
    fn test_push_text_creates_proper_events() {
        let initial_response = create_test_response();
        let mut creator = ResponseStreamCreator::new(
            sauropod_output_parser::get_model_parser(sauropod_output_parser::ModelType::Unknown),
            initial_response,
        );

        // First push should create response created, output item, content part, and text delta events
        let events = creator.push_text("Hello".to_string());
        assert_eq!(events.len(), 5);

        // Check response created event
        match &events[0] {
            sauropod_openai_api::ResponseStreamEvent::ResponseCreatedEvent { response, .. } => {
                assert_eq!(response.id, "test_response");
            }
            _ => panic!("Expected ResponseCreatedEvent"),
        }

        // Check response in progress
        match &events[1] {
            sauropod_openai_api::ResponseStreamEvent::ResponseInProgressEvent {
                response, ..
            } => {
                assert_eq!(response.id, "test_response");
            }
            _ => panic!("Expected ResponseInProgressEvent"),
        }

        // Check output item added event
        match &events[2] {
            sauropod_openai_api::ResponseStreamEvent::ResponseOutputItemAddedEvent {
                item,
                output_index,
                ..
            } => {
                assert_eq!(*output_index, 0);
                match item {
                    OutputItem::OutputMessage { role, status, .. } => {
                        assert!(matches!(role, OutputMessageRole::Assistant));
                        assert!(matches!(status, Status::InProgress));
                    }
                    _ => panic!("Expected OutputMessage"),
                }
            }
            _ => panic!("Expected ResponseOutputItemAddedEvent"),
        }

        // Check content part added event
        match &events[3] {
            sauropod_openai_api::ResponseStreamEvent::ResponseContentPartAddedEvent {
                content_index,
                output_index,
                ..
            } => {
                assert_eq!(*content_index, 0);
                assert_eq!(*output_index, 0);
            }
            _ => panic!("Expected ResponseContentPartAddedEvent"),
        }

        // Check text delta event
        match &events[4] {
            sauropod_openai_api::ResponseStreamEvent::ResponseTextDeltaEvent {
                content_index,
                output_index,
                delta,
                ..
            } => {
                assert_eq!(*content_index, 0);
                assert_eq!(*output_index, 0);
                assert_eq!(delta, "Hello");
            }
            _ => panic!("Expected ResponseTextDeltaEvent"),
        }

        // Second push should only create text delta event
        let events2 = creator.push_text(" World".to_string());
        assert_eq!(events2.len(), 1);

        match &events2[0] {
            sauropod_openai_api::ResponseStreamEvent::ResponseTextDeltaEvent { delta, .. } => {
                assert_eq!(delta, " World");
            }
            _ => panic!("Expected ResponseTextDeltaEvent"),
        }

        // Verify final state
        assert_eq!(creator.response.output.len(), 1);
        match &creator.response.output[0] {
            OutputItem::OutputMessage { content, .. } => {
                assert_eq!(content.len(), 1);
                match &content[0] {
                    sauropod_openai_api::OutputContent::OutputTextContent { text, .. } => {
                        assert_eq!(text, "Hello World");
                    }
                    _ => panic!("Expected OutputTextContent"),
                }
            }
            _ => panic!("Expected OutputMessage"),
        }
    }

    #[test]
    fn test_finish_closes_content_and_output_items() {
        let initial_response = create_test_response();
        let mut creator = ResponseStreamCreator::new(
            sauropod_output_parser::get_model_parser(sauropod_output_parser::ModelType::Unknown),
            initial_response,
        );

        // Push some text to create open content part and output item
        creator.push_text("Hello".to_string());

        // Finish should close both content part and output item, then emit completion event
        let finish_events = creator.finish();
        assert_eq!(finish_events.len(), 4);

        // Check text done event (comes first)
        match &finish_events[0] {
            sauropod_openai_api::ResponseStreamEvent::ResponseTextDoneEvent {
                content_index,
                output_index,
                text,
                ..
            } => {
                assert_eq!(*content_index, 0);
                assert_eq!(*output_index, 0);
                assert_eq!(text, "Hello");
            }
            _ => panic!("Expected ResponseTextDoneEvent"),
        }

        // Check content part done event (comes second)
        match &finish_events[1] {
            sauropod_openai_api::ResponseStreamEvent::ResponseContentPartDoneEvent {
                content_index,
                output_index,
                part,
                ..
            } => {
                assert_eq!(*content_index, 0);
                assert_eq!(*output_index, 0);
                match part {
                    sauropod_openai_api::OutputContent::OutputTextContent { text, .. } => {
                        assert_eq!(text, "Hello");
                    }
                    _ => panic!("Expected OutputTextContent"),
                }
            }
            _ => panic!("Expected ResponseContentPartDoneEvent"),
        }

        // Check output item done event
        match &finish_events[2] {
            sauropod_openai_api::ResponseStreamEvent::ResponseOutputItemDoneEvent {
                item,
                output_index,
                ..
            } => {
                assert_eq!(*output_index, 0);
                match item {
                    OutputItem::OutputMessage { status, .. } => {
                        assert!(matches!(status, Status::Completed));
                    }
                    _ => panic!("Expected OutputMessage"),
                }
            }
            _ => panic!("Expected ResponseOutputItemDoneEvent"),
        }

        // Check response completed event
        match &finish_events[3] {
            sauropod_openai_api::ResponseStreamEvent::ResponseCompletedEvent {
                response, ..
            } => {
                assert_eq!(response.id, "test_response");
                assert!(matches!(
                    response.status,
                    Some(sauropod_openai_api::ResponseStatus::Completed)
                ));
            }
            _ => panic!("Expected ResponseCompletedEvent"),
        }
    }

    #[test]
    fn test_finish_with_no_open_items() {
        let initial_response = create_test_response();
        let mut creator = ResponseStreamCreator::new(
            sauropod_output_parser::get_model_parser(sauropod_output_parser::ModelType::Unknown),
            initial_response,
        );

        // Finish without creating any content should only emit completion event
        let finish_events = creator.finish();
        assert_eq!(finish_events.len(), 1);

        // Check response completed event
        match &finish_events[0] {
            sauropod_openai_api::ResponseStreamEvent::ResponseCompletedEvent {
                response, ..
            } => {
                assert_eq!(response.id, "test_response");
                assert!(matches!(
                    response.status,
                    Some(sauropod_openai_api::ResponseStatus::Completed)
                ));
            }
            _ => panic!("Expected ResponseCompletedEvent"),
        }
    }

    #[test]
    fn test_text_done_event_not_emitted_for_empty_text() {
        let initial_response = create_test_response();
        let mut creator = ResponseStreamCreator::new(
            sauropod_output_parser::get_model_parser(sauropod_output_parser::ModelType::Unknown),
            initial_response,
        );

        // Push empty text to create open content part and output item
        creator.push_text("".to_string());

        // Close content part manually - should not emit ResponseTextDoneEvent for empty text
        let close_events = creator.close_current_content_part();
        assert_eq!(close_events.len(), 1); // Only ResponseContentPartDoneEvent, no ResponseTextDoneEvent

        // Check content part done event (should be the only event)
        match &close_events[0] {
            sauropod_openai_api::ResponseStreamEvent::ResponseContentPartDoneEvent {
                content_index,
                output_index,
                part,
                ..
            } => {
                assert_eq!(*content_index, 0);
                assert_eq!(*output_index, 0);
                match part {
                    sauropod_openai_api::OutputContent::OutputTextContent { text, .. } => {
                        assert_eq!(text, "");
                    }
                    _ => panic!("Expected OutputTextContent"),
                }
            }
            _ => panic!("Expected ResponseContentPartDoneEvent"),
        }
    }
}
