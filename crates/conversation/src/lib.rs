//! In-memory state management for conversations.

use sauropod_openai_api::HasId;

/// Error type for conversation operations.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No previous item with ID {0} found")]
    NoPreviousItem(String),
}

/// A message in a conversation.
#[derive(Clone, Debug)]
pub enum Message {
    /// A user or system message input.
    InputItem(sauropod_openai_api::InputItem),
    /// An assistant message output.
    OutputItem(sauropod_openai_api::OutputItem),
}

impl From<sauropod_openai_api::InputItem> for Message {
    fn from(val: sauropod_openai_api::InputItem) -> Self {
        Message::InputItem(val)
    }
}

impl From<sauropod_openai_api::OutputItem> for Message {
    fn from(val: sauropod_openai_api::OutputItem) -> Self {
        Message::OutputItem(val)
    }
}

impl sauropod_openai_api::HasId for Message {
    fn get_id(&self) -> Option<&str> {
        match self {
            Message::InputItem(item) => item.get_id(),
            Message::OutputItem(item) => item.get_id(),
        }
    }
}

impl From<Message> for sauropod_openai_api::InputItem {
    fn from(val: Message) -> Self {
        match val {
            Message::InputItem(x) => x,
            Message::OutputItem(x) => sauropod_openai_api::InputItem::Item(x.into()),
        }
    }
}

/// Conversation history.
#[derive(Debug)]
pub struct Conversation {
    messages: Vec<Message>,
}

impl Default for Conversation {
    fn default() -> Self {
        Self::new()
    }
}

impl Conversation {
    /// Create a new empty conversation.
    pub fn new() -> Self {
        Conversation {
            messages: Vec::with_capacity(4),
        }
    }

    /// Add a user message to the conversation.
    pub fn add_user_message(&mut self, message: &str) {
        let content = sauropod_openai_api::InputContent::InputTextContent(
            sauropod_openai_api::InputTextContent {
                text: message.to_string(),
                r#type: sauropod_openai_api::InputTextContentType::InputText,
            },
        );
        let msg = sauropod_openai_api::InputMessage {
            content: sauropod_openai_api::InputMessageContentList(vec![content]),
            role: sauropod_openai_api::InputMessageRole::User,
            status: Some(sauropod_openai_api::Status::Completed),
            r#type: Some(sauropod_openai_api::InputMessageType::Message),
        };
        self.messages
            .push(Message::InputItem(sauropod_openai_api::InputItem::Item(
                msg.into(),
            )));
    }

    /// Add an assistant text message to the conversation.
    pub fn add_assistant_message(&mut self, message: &str) {
        let content = sauropod_openai_api::OutputContent::OutputTextContent(
            sauropod_openai_api::OutputTextContent {
                annotations: Vec::new(),
                logprobs: None,
                text: message.to_string(),
                r#type: sauropod_openai_api::OutputTextContentType::OutputText,
            },
        );
        self.messages.push(Message::OutputItem(
            sauropod_openai_api::OutputItem::OutputMessage {
                content: vec![content],
                id: uuid::Uuid::new_v4().to_string(),
                role: sauropod_openai_api::OutputMessageRole::Assistant,
                status: sauropod_openai_api::Status::Completed,
            },
        ));
    }

    /// Get a reference to the messages in the conversation.
    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    /// Get the conversation as a create response request.
    pub fn make_request(&self) -> sauropod_openai_api::CreateResponse {
        let inputs: Vec<sauropod_openai_api::InputItem> = self
            .messages
            .iter()
            .cloned()
            .map(|message| message.into())
            .collect();
        sauropod_openai_api::CreateResponse {
            input: Some(sauropod_openai_api::CreateResponseInput::Variant1(inputs)),
            ..sauropod_openai_api::CreateResponse::default()
        }
    }

    /// Add a response to the conversation.
    pub fn add_response(&mut self, response: sauropod_openai_api::Response) {
        for output in response.output {
            tracing::info!("Adding output item: {:#?}", &output);
            self.messages.push(Message::OutputItem(output));
        }
    }

    /// Add an item to the conversation.
    pub fn add_input_item(
        &mut self,
        item: sauropod_openai_api::InputItem,
        previous_item_id: Option<String>,
    ) -> Result<Option<&str>, Error> {
        let position = previous_item_id.as_deref().and_then(|previous_item_id| {
            self.messages
                .iter()
                .position(|x| x.get_id() == Some(previous_item_id))
        });

        if let Some(previous_item_id) = previous_item_id
            && position.is_none()
        {
            tracing::warn!("Previous item ID {previous_item_id} not found in conversation");
            return Err(Error::NoPreviousItem(previous_item_id));
        }

        let insert_index = if let Some(index) = position {
            self.messages.insert(index + 1, Message::InputItem(item));
            index + 1
        } else {
            self.messages.push(Message::InputItem(item));
            self.messages.len() - 1
        };

        if insert_index > 0 {
            Ok(self.messages[insert_index - 1].get_id())
        } else {
            Ok(None)
        }
    }
}
