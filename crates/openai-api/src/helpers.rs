#![allow(clippy::derivable_impls)]

impl crate::CreateResponseInput {
    /// Invoke a function for each `InputItem`.
    pub fn for_each(&self, mut f: impl FnMut(&crate::InputItem)) {
        match self {
            crate::CreateResponseInput::Variant0(content) => {
                let message = crate::InputItem::EasyInputMessage(crate::EasyInputMessage {
                    role: crate::EasyInputMessageRole::User,
                    content: crate::EasyInputMessageContent::Variant0(content.clone()),
                    r#type: Some(crate::EasyInputMessageType::Message),
                });
                f(&message);
            }
            crate::CreateResponseInput::Variant1(x) => x.iter().for_each(f),
        }
    }
}

impl crate::EasyInputMessageContent {
    /// Invoke a function for each `InputItem`.
    pub fn for_each(&self, mut f: impl FnMut(&crate::InputContent)) {
        match self {
            crate::EasyInputMessageContent::Variant0(content) => {
                let content = crate::InputContent::InputTextContent(crate::InputTextContent {
                    text: content.clone(),
                    r#type: crate::InputTextContentType::InputText,
                });
                f(&content);
            }
            crate::EasyInputMessageContent::InputMessageContentList(
                crate::InputMessageContentList(x),
            ) => x.iter().for_each(f),
        }
    }
}

impl From<crate::RealtimeSessionCreateRequestMaxResponseOutputTokens>
    for crate::RealtimeSessionMaxResponseOutputTokens
{
    fn from(value: crate::RealtimeSessionCreateRequestMaxResponseOutputTokens) -> Self {
        match value {
            crate::RealtimeSessionCreateRequestMaxResponseOutputTokens::Inf => Self::Inf,
            crate::RealtimeSessionCreateRequestMaxResponseOutputTokens::Variant0(x) => {
                Self::Variant0(x)
            }
        }
    }
}

impl From<crate::RealtimeSessionCreateRequestInputAudioNoiseReduction>
    for crate::RealtimeSessionInputAudioNoiseReduction
{
    fn from(value: crate::RealtimeSessionCreateRequestInputAudioNoiseReduction) -> Self {
        Self {
            r#type: value.r#type,
        }
    }
}

impl From<crate::OutputMessageRole> for crate::ConversationItemRole {
    fn from(value: crate::OutputMessageRole) -> Self {
        match value {
            crate::OutputMessageRole::Assistant => Self::Assistant,
        }
    }
}

impl From<crate::ResponseStatus> for crate::RealtimeResponseStatus {
    fn from(value: crate::ResponseStatus) -> Self {
        match value {
            crate::ResponseStatus::Completed => Self::Completed,
            crate::ResponseStatus::Failed => Self::Failed,
            crate::ResponseStatus::Cancelled => Self::Cancelled,
            crate::ResponseStatus::InProgress | crate::ResponseStatus::Queued => Self::Incomplete,
            crate::ResponseStatus::Incomplete => Self::Incomplete,
        }
    }
}

impl From<crate::ResponseUsage> for crate::RealtimeResponseUsage {
    fn from(value: crate::ResponseUsage) -> Self {
        Self {
            input_token_details: Some(crate::RealtimeResponseUsageInputTokenDetails {
                audio_tokens: None,
                cached_tokens: Some(value.input_tokens_details.cached_tokens),
                text_tokens: Some(value.input_tokens),
            }),
            input_tokens: Some(value.input_tokens),
            output_token_details: Some(crate::RealtimeResponseUsageOutputTokenDetails {
                audio_tokens: None,
                text_tokens: Some(value.output_tokens),
            }),
            output_tokens: Some(value.output_tokens),
            total_tokens: Some(value.total_tokens),
        }
    }
}

impl From<crate::Response> for crate::RealtimeResponse {
    fn from(value: crate::Response) -> Self {
        Self {
            conversation_id: None,
            id: Some(value.id),
            max_output_tokens: None,
            metadata: value.model_response_properties.metadata,
            modalities: None,
            object: Some(crate::RealtimeResponseObject::RealtimeResponse),
            output: None, // Output types are different, would need complex conversion
            output_audio_format: None,
            status: value.status.map(|s| s.into()),
            status_details: None,
            temperature: value.model_response_properties.temperature,
            usage: value.usage.map(|u| u.into()),
            voice: None,
        }
    }
}

impl From<crate::OutputContent> for crate::RealtimeConversationItemContentItem {
    fn from(value: crate::OutputContent) -> Self {
        match value {
            crate::OutputContent::OutputTextContent(content) => {
                crate::RealtimeConversationItemContentItem {
                    audio: None,
                    id: None,
                    text: Some(content.text),
                    transcript: None,
                    r#type: Some(crate::RealtimeConversationItemContentItemType::Text),
                }
            }
            crate::OutputContent::RefusalContent(_) => {
                unimplemented!("Refusals are not currently supported")
            }
        }
    }
}

impl From<crate::OutputContent> for crate::RealtimeServerEventResponseContentPartAddedPart {
    fn from(value: crate::OutputContent) -> Self {
        match value {
            crate::OutputContent::OutputTextContent(content) => Self {
                audio: None,
                text: Some(content.text),
                transcript: None,
                r#type: Some(crate::Modalities::Text),
            },
            crate::OutputContent::RefusalContent(_) => {
                unimplemented!("Refusals are not currently supported")
            }
        }
    }
}
impl From<crate::OutputContent> for crate::RealtimeServerEventResponseContentPartDonePart {
    fn from(value: crate::OutputContent) -> Self {
        match value {
            crate::OutputContent::OutputTextContent(content) => Self {
                audio: None,
                text: Some(content.text),
                transcript: None,
                r#type: Some(crate::Modalities::Text),
            },
            crate::OutputContent::RefusalContent(_) => {
                unimplemented!("Refusals are not currently supported")
            }
        }
    }
}

impl From<crate::OutputItem> for crate::RealtimeConversationItem {
    fn from(value: crate::OutputItem) -> Self {
        match value {
            crate::OutputItem::OutputMessage {
                content,
                id,
                role,
                status,
            } => Self {
                id: Some(id),
                role: Some(role.into()),
                status: Some(status),
                r#type: Some(crate::RealtimeConversationItemType::Message),
                arguments: None,
                call_id: None,
                content: Some(
                    content
                        .into_iter()
                        .map(crate::RealtimeConversationItemContentItem::from)
                        .collect(),
                ),
                name: None,
                object: Some(crate::RealtimeConversationItemObject::RealtimeItem),
                output: None,
            },
            crate::OutputItem::FunctionToolCall {
                arguments,
                call_id,
                id,
                name,
                status,
            } => Self {
                id: Some(id),
                role: None,
                status,
                r#type: Some(crate::RealtimeConversationItemType::FunctionCall),
                arguments: Some(arguments),
                call_id: Some(call_id),
                content: None,
                name: Some(name),
                object: Some(crate::RealtimeConversationItemObject::RealtimeItem),
                output: None,
            },
            x => unimplemented!("Realtime support for {x:?} is not implemented yet"),
        }
    }
}

impl From<crate::OutputItem> for crate::Item {
    fn from(value: crate::OutputItem) -> Self {
        match value {
            crate::OutputItem::OutputMessage {
                content,
                id,
                role,
                status,
            } => crate::Item::OutputMessage {
                content,
                id,
                role,
                status,
            },
            crate::OutputItem::FileSearchToolCall {
                id,
                queries,
                results,
                status,
            } => crate::Item::FileSearchToolCall {
                id,
                queries,
                results,
                status,
            },
            crate::OutputItem::FunctionToolCall {
                arguments,
                call_id,
                id,
                name,
                status,
            } => crate::Item::FunctionToolCall {
                arguments,
                call_id,
                id,
                name,
                status,
            },
            crate::OutputItem::WebSearchToolCall { action, id, status } => {
                crate::Item::WebSearchToolCall { action, id, status }
            }
            crate::OutputItem::ComputerToolCall {
                action,
                call_id,
                id,
                pending_safety_checks,
                status,
            } => crate::Item::ComputerToolCall {
                action,
                call_id,
                id,
                pending_safety_checks,
                status,
            },
            crate::OutputItem::ReasoningItem {
                encrypted_content,
                id,
                status,
                summary,
            } => crate::Item::ReasoningItem {
                encrypted_content,
                id,
                status,
                summary,
            },
            crate::OutputItem::ImageGenToolCall { id, result, status } => {
                crate::Item::ImageGenToolCall { id, result, status }
            }
            crate::OutputItem::CodeInterpreterToolCall {
                code,
                container_id,
                id,
                outputs,
                status,
            } => crate::Item::CodeInterpreterToolCall {
                code,
                container_id,
                id,
                outputs,
                status,
            },
            crate::OutputItem::LocalShellToolCall {
                action,
                call_id,
                id,
                status,
            } => crate::Item::LocalShellToolCall {
                action,
                call_id,
                id,
                status,
            },
            crate::OutputItem::MCPToolCall {
                arguments,
                error,
                id,
                name,
                output,
                server_label,
            } => crate::Item::MCPToolCall {
                arguments,
                error,
                id,
                name,
                output,
                server_label,
            },
            crate::OutputItem::MCPListTools {
                error,
                id,
                server_label,
                tools,
            } => crate::Item::MCPListTools {
                error,
                id,
                server_label,
                tools,
            },
            crate::OutputItem::MCPApprovalRequest {
                arguments,
                id,
                name,
                server_label,
            } => crate::Item::MCPApprovalRequest {
                arguments,
                id,
                name,
                server_label,
            },
        }
    }
}

impl Default for crate::ModelResponseProperties {
    fn default() -> Self {
        Self {
            metadata: None,
            temperature: None,
            top_p: None,
            user: None,
            top_logprobs: None,
            service_tier: None,
        }
    }
}

impl Default for crate::ResponseProperties {
    fn default() -> Self {
        Self {
            background: None,
            max_output_tokens: None,
            model: None,
            previous_response_id: None,
            prompt: None,
            reasoning: None,
            text: None,
            tool_choice: None,
            tools: None,
            max_tool_calls: None,
            truncation: None,
        }
    }
}

impl Default for crate::CreateModelResponseProperties {
    fn default() -> Self {
        Self {
            model_response_properties: crate::ModelResponseProperties::default(),
            top_logprobs: None,
        }
    }
}

impl Default for crate::CreateResponse {
    fn default() -> Self {
        Self {
            create_model_response_properties: crate::CreateModelResponseProperties::default(),
            response_properties: crate::ResponseProperties::default(),
            include: None,
            input: None,
            instructions: None,
            parallel_tool_calls: None,
            store: None,
            stream: None,
        }
    }
}

impl crate::HasId for crate::Item {
    fn get_id(&self) -> Option<&str> {
        match self {
            crate::Item::InputMessage { .. } => None,
            crate::Item::OutputMessage { id, .. }
            | crate::Item::FileSearchToolCall { id, .. }
            | crate::Item::ComputerToolCall { id, .. }
            | crate::Item::WebSearchToolCall { id, .. }
            | crate::Item::FunctionToolCall { id, .. }
            | crate::Item::ReasoningItem { id, .. }
            | crate::Item::ImageGenToolCall { id, .. }
            | crate::Item::CodeInterpreterToolCall { id, .. }
            | crate::Item::LocalShellToolCall { id, .. }
            | crate::Item::LocalShellToolCallOutput { id, .. }
            | crate::Item::MCPListTools { id, .. }
            | crate::Item::MCPApprovalRequest { id, .. }
            | crate::Item::MCPToolCall { id, .. } => Some(id.as_str()),
            crate::Item::ComputerCallOutputItemParam { id, .. }
            | crate::Item::FunctionCallOutputItemParam { id, .. }
            | crate::Item::MCPApprovalResponse { id, .. } => id.as_deref(),
        }
    }
}

impl crate::HasId for crate::InputItem {
    fn get_id(&self) -> Option<&str> {
        match self {
            crate::InputItem::EasyInputMessage(_) => None,
            crate::InputItem::Item(item) => item.get_id(),
            crate::InputItem::ItemReferenceParam(item_ref_param) => item_ref_param.get_id(),
        }
    }
}
