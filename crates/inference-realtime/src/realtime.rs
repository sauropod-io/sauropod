//! Realtime.

use std::{ops::Range, sync::Arc};

use base64::prelude::*;
use tokio_stream::StreamExt;
use tracing::Instrument as _;

use sauropod_openai_api::{
    RealtimeClientEvent, RealtimeConversationItem, RealtimeServerEvent,
    RealtimeServerEventErrorError, RealtimeSession, RealtimeSessionTurnDetectionType,
};

use crate::audio::*;
use crate::make_id;
use crate::socket::SocketWrapper;

pub(crate) struct RealtimeSessionState {
    /// The audio buffer for this session.
    audio_buffer: tokio::sync::Mutex<AudioBuffer>,
    /// The global state.
    global_state: Arc<sauropod_global_state::GlobalState>,
    /// The conversation state of this session.
    conversation: tokio::sync::Mutex<sauropod_conversation::Conversation>,
    /// The session configuration.
    pub(crate) session: tokio::sync::Mutex<RealtimeSession>,
}

impl crate::RealtimeFunctionality for RealtimeSessionState {
    async fn new(
        id: String,
        global_state: Arc<sauropod_global_state::GlobalState>,
    ) -> anyhow::Result<Self> {
        let config = RealtimeSession {
            speed: None,
            tracing: None,
            input_audio_format: Some(sauropod_openai_api::InputAudioStreamFormat::Pcm16),
            input_audio_transcription: None,
            turn_detection: crate::defaults::make_default_vad(),
            input_audio_noise_reduction: None,
            modalities: None,
            id: Some(id),
            instructions: None,
            max_response_output_tokens: None,
            model: None,
            output_audio_format: None,
            temperature: None,
            tool_choice: None,
            tools: None,
            voice: None,
        };

        Ok(Self {
            session: tokio::sync::Mutex::new(config),
            audio_buffer: tokio::sync::Mutex::new(AudioBuffer::new(&global_state)),
            global_state,
            conversation: tokio::sync::Mutex::new(sauropod_conversation::Conversation::new()),
        })
    }

    async fn session_created(&self, socket: &SocketWrapper) -> anyhow::Result<()> {
        socket
            .send_event(RealtimeServerEvent::SessionCreated {
                event_id: make_id(),
                session: self.session.lock().await.clone(),
            })
            .await?;
        Ok(())
    }

    async fn process_message(
        self: &Arc<Self>,
        client_event: RealtimeClientEvent,
        socket: &SocketWrapper,
    ) -> anyhow::Result<()> {
        match client_event {
            // For voice to voice mode this is how the session is configured.
            RealtimeClientEvent::SessionUpdate { session, .. } => {
                let mut inner_session = self.session.lock().await;

                macro_rules! update_field {
                    ($field:ident, $inner:ident, $session:expr) => {
                        if let Some(value) = $session.$field {
                            $inner.$field = Some(value.into());
                        }
                    };
                }

                update_field!(input_audio_format, inner_session, session);
                update_field!(input_audio_noise_reduction, inner_session, session);
                update_field!(input_audio_transcription, inner_session, session);
                update_field!(instructions, inner_session, session);
                update_field!(max_response_output_tokens, inner_session, session);
                update_field!(modalities, inner_session, session);
                update_field!(model, inner_session, session);
                update_field!(output_audio_format, inner_session, session);
                update_field!(speed, inner_session, session);
                update_field!(temperature, inner_session, session);
                update_field!(tool_choice, inner_session, session);
                update_field!(tracing, inner_session, session);
                update_field!(voice, inner_session, session);
                update_field!(tools, inner_session, session);
                update_field!(turn_detection, inner_session, session);

                if let Some(voice) = &inner_session.voice {
                    if self.global_state.get_voice_model(&voice.0).await.is_none() {
                        return Err(anyhow::anyhow!("Voice {} is not available", voice.0));
                    }
                }

                socket
                    .send_event(RealtimeServerEvent::SessionUpdated {
                        event_id: make_id(),
                        session: inner_session.clone(),
                    })
                    .await?;
            }
            RealtimeClientEvent::InputAudioBufferAppend { .. } => {
                anyhow::bail!(
                    "Internal error: InputAudioBufferAppend is handled through a different code path"
                );
            }
            RealtimeClientEvent::InputAudioBufferClear { .. } => {
                self.get_audio_buffer().await.clear();
            }
            RealtimeClientEvent::InputAudioBufferCommit { .. } => {
                // Committing the input audio buffer will trigger input audio transcription (if enabled in session configuration),
                // but it will not create a response from the model. The server will respond with an input_audio_buffer.committed event.
                anyhow::bail!("Manual buffer commit is not supported yet");
            }
            RealtimeClientEvent::ConversationItemCreate {
                item,
                previous_item_id,
                ..
            } => {
                // Add a new Item to the Conversation's context, including messages, function calls, and function call responses. This event can be used both to populate a "history" of the conversation and to add new items mid-stream, but has the current limitation that it cannot populate assistant audio messages.
                // If successful, the server will respond with a conversation.item.created event, otherwise an error event will be sent.

                let cloned_item = item.clone();
                let status = item.status;
                let conversation_item = match &item.r#type {
                    Some(sauropod_openai_api::RealtimeConversationItemType::FunctionCall) => {
                        sauropod_openai_api::InputItem::Item(
                            sauropod_openai_api::Item::FunctionToolCall {
                                arguments: item.arguments.unwrap_or_default(),
                                call_id: item.call_id.unwrap_or_default(),
                                id: item.id.unwrap_or_default(),
                                name: item.name.unwrap_or_default(),
                                status,
                            },
                        )
                    }
                    Some(sauropod_openai_api::RealtimeConversationItemType::FunctionCallOutput) => {
                        sauropod_openai_api::InputItem::Item(
                            sauropod_openai_api::Item::FunctionCallOutputItemParam {
                                call_id: item.call_id.unwrap_or_default(),
                                id: item.id,
                                output: item.output.unwrap_or_default(),
                                status,
                            },
                        )
                    }
                    Some(sauropod_openai_api::RealtimeConversationItemType::Message)
                        if matches!(
                            item.role,
                            Some(sauropod_openai_api::ConversationItemRole::User)
                                | Some(sauropod_openai_api::ConversationItemRole::System)
                                | None
                        ) =>
                    {
                        let mut content = Vec::new();
                        if let Some(conversation_items) = item.content {
                            for item in conversation_items {
                                match item.r#type {
                                    None | Some(sauropod_openai_api::RealtimeConversationItemContentItemType::Text) | Some(sauropod_openai_api::RealtimeConversationItemContentItemType::InputText) => {
                                        content.push(sauropod_openai_api::InputTextContent{
                                            text: item.text.unwrap_or_default(),
                                            r#type: sauropod_openai_api::InputTextContentType::InputText,
                                        }.into());
                                    }
                                    Some(sauropod_openai_api::RealtimeConversationItemContentItemType::ItemReference) => {
                                        anyhow::bail!(
                                            "ItemReference is not supported yet"
                                        );
                                    }
                                    Some(sauropod_openai_api::RealtimeConversationItemContentItemType::Audio) | Some(sauropod_openai_api::RealtimeConversationItemContentItemType::InputAudio) => {
                                        tracing::warn!(
                                            "Received conversation item with InputAudio content type, this is not implemented yet"
                                        );
                                        return Ok(());
                                    }
                                }
                            }
                        }
                        let message_role = match item.role {
                            Some(sauropod_openai_api::ConversationItemRole::User) => {
                                Some(sauropod_openai_api::InputMessageRole::User)
                            }
                            Some(sauropod_openai_api::ConversationItemRole::System) => {
                                Some(sauropod_openai_api::InputMessageRole::System)
                            }
                            Some(sauropod_openai_api::ConversationItemRole::Assistant) => {
                                anyhow::bail!(
                                    "Received conversation item with role Assistant from client, this is not allowed yet"
                                );
                            }
                            None => None,
                        };
                        sauropod_openai_api::InputItem::Item(
                            sauropod_openai_api::Item::InputMessage(
                                sauropod_openai_api::InputMessage {
                                    content: sauropod_openai_api::InputMessageContentList(content),
                                    role: message_role
                                        .unwrap_or(sauropod_openai_api::InputMessageRole::User),
                                    status,
                                    r#type: Some(sauropod_openai_api::InputMessageType::Message),
                                },
                            ),
                        )
                    }
                    Some(sauropod_openai_api::RealtimeConversationItemType::Message) => {
                        // Note: User and System messages are handled above
                        todo!()
                    }
                    None => {
                        anyhow::bail!(
                            "Received conversation item without type, defaulting to User"
                        );
                    }
                };
                let mut conversation = self.conversation.lock().await;
                let previous_item_id =
                    conversation.add_input_item(conversation_item, previous_item_id)?;
                socket
                    .send_event(RealtimeServerEvent::ConversationItemCreated {
                        event_id: make_id(),
                        item: cloned_item,
                        previous_item_id: previous_item_id.map(|x| x.to_string()),
                    })
                    .await?;

                // Create response with the new conversation state
                self.create_response(&mut conversation, socket.clone())
                    .await?;
            }
            RealtimeClientEvent::ConversationItemRetrieve { .. } => {
                // Send this event when you want to retrieve the server's representation of a specific item in the conversation history. This is useful, for example, to inspect user audio after noise cancellation and VAD.
                // The server will respond with a conversation.item.retrieved event, unless the item does not exist in the conversation history, in which case the server will respond with an error.
                anyhow::bail!("ConversationItemRetrieve is not implemented yet");
            }
            RealtimeClientEvent::ConversationItemDelete { .. } => {
                // Send this event when you want to remove any item from the conversation history.
                // The server will respond with a conversation.item.deleted event, unless the item does not exist in the conversation history, in which case the server will respond with an error.
                anyhow::bail!("ConversationItemDelete is not implemented yet");
            }
            RealtimeClientEvent::ConversationItemTruncate { .. } => {
                // Send this event to truncate a previous assistant message’s audio. The server will produce audio faster than realtime, so this event is useful when the user interrupts to truncate audio that has already been sent to the client but not yet played. This will synchronize the server's understanding of the audio with the client's playback.
                // Truncating audio will delete the server-side text transcript to ensure there is not text in the context that hasn't been heard by the user.
                // If successful, the server will respond with a conversation.item.truncated event.
                anyhow::bail!("ConversationItemTruncate is not implemented yet");
            }

            RealtimeClientEvent::ResponseCreate { response, .. } => {
                // This event instructs the server to create a Response, which means triggering model inference. When in Server VAD mode, the server will create Responses automatically.
                // A Response will include at least one Item, and may have two, in which case the second will be a function call. These Items will be appended to the conversation history.
                // The server will respond with a response.created event, events for Items and content created, and finally a response.done event to indicate the Response is complete.
                // The response.create event includes inference configuration like instructions, and temperature. These fields will override the Session's configuration for this Response only.
                tracing::warn!("ResponseCreate is not implemented yet: {:#?}", response);
            }
            RealtimeClientEvent::ResponseCancel { .. } => {
                // Send this event to cancel an in-progress response. The server will respond with a response.cancelled event or an error if there is no response to cancel.
                tracing::warn!("ResponseCancel is not implemented yet");
            }
            _ => {
                tracing::warn!("Received unsupported client event: {:?}", client_event);
            }
        }
        Ok(())
    }

    async fn process_input_audio(
        self: &Arc<Self>,
        audio_pcm16: Vec<i16>,
        socket: &SocketWrapper,
    ) -> anyhow::Result<()> {
        self.get_audio_buffer().await.extend(audio_pcm16);
        self.run_turn_detection(socket).await?;
        Ok(())
    }
}

impl RealtimeSessionState {
    /// Get the name of the model for this session.
    async fn get_model_name(&self) -> String {
        self.session
            .lock()
            .await
            .model
            .as_ref()
            .map(|x| x.0.clone())
            .unwrap_or_else(|| "default".to_string())
    }

    /// Get the name of the voice for this session.
    async fn get_voice_name(&self) -> String {
        self.session
            .lock()
            .await
            .voice
            .as_ref()
            .map(|x| x.0.clone())
            .unwrap_or_else(|| "default".to_string())
    }

    async fn get_audio_buffer(&self) -> tokio::sync::MutexGuard<'_, AudioBuffer> {
        self.audio_buffer
            .lock()
            .instrument(tracing::debug_span!("lock audio buffer"))
            .await
    }

    /// Detect when speech begins and ends.
    async fn run_turn_detection(self: &Arc<Self>, socket: &SocketWrapper) -> anyhow::Result<()> {
        let (silence_duration_ms, prefix_padding_ms, vad_threshold) = {
            let session = self.session.lock().await;
            let Some(turn_detection) = &session.turn_detection else {
                return Ok(());
            };
            if !matches!(
                turn_detection.r#type,
                Some(RealtimeSessionTurnDetectionType::ServerVad)
            ) {
                anyhow::bail!(
                    "Unsupported turn detection type: {:#?}",
                    turn_detection.r#type
                )
            }

            let silence_duration_ms = turn_detection.silence_duration_ms.unwrap_or(750) as u32;
            let prefix_padding_ms = turn_detection.prefix_padding_ms.unwrap_or(400);
            let vad_threshold = turn_detection.threshold.unwrap_or(0.5) as f32;
            (silence_duration_ms, prefix_padding_ms, vad_threshold)
        };

        let mut audio_buffer = self.get_audio_buffer().await;

        let ranges = audio_buffer
            .run_vad(
                socket,
                silence_duration_ms,
                prefix_padding_ms as u32,
                vad_threshold,
            )
            .await?;

        for vad_result in ranges {
            // Process any remaining audio and send final results
            self.process_audio_to_items(
                &mut audio_buffer,
                vad_result.range,
                vad_result.item_id,
                socket,
            )
            .await?;
        }
        Ok(())
    }

    async fn process_audio(
        self: &Arc<Self>,
        audio_buffer: &mut AudioBuffer,
        audio_range: Range<usize>,
    ) -> anyhow::Result<String> {
        // Get the STT model from the inner lock
        let stt_model = self.global_state.get_loaded_models().stt_model.clone();

        // Get the audio data from the audio buffer lock
        let audio_data = audio_buffer.range(audio_range).copied().collect::<Vec<_>>();

        // Process the audio with the STT model
        let text = match stt_model.enqueue(audio_data.clone()).await {
            Ok(text) => text,
            Err(e) => {
                tracing::warn!("Speech to text transcription failed: {e}");
                anyhow::bail!("Speech to text transcription failed: {e}")
            }
        };

        Ok(text)
    }

    async fn create_response(
        &self,
        conversation_state: &mut sauropod_conversation::Conversation,
        socket: SocketWrapper,
    ) -> anyhow::Result<()> {
        let model_name = self.get_model_name().await;
        let Some(model) = self.global_state.get_model(&model_name).await else {
            socket
                .send_event(RealtimeServerEvent::Error {
                    event_id: make_id(),
                    error: RealtimeServerEventErrorError {
                        message: format!("Model {model_name} not found"),
                        code: Some("model_not_found".to_string()),
                        r#type: "server_error".to_string(),
                        event_id: None,
                        param: None,
                    },
                })
                .await?;
            return Ok(());
        };

        let mut request = conversation_state.make_request();
        let mut text_modality = true;
        let mut audio_modality = true;
        {
            let session = self.session.lock().await;
            if let Some(modalities) = &session.modalities {
                text_modality = modalities
                    .iter()
                    .any(|x| matches!(x, sauropod_openai_api::Modalities::Text));
                audio_modality = modalities
                    .iter()
                    .any(|x| matches!(x, sauropod_openai_api::Modalities::Audio));
            }

            request.response_properties.max_output_tokens = match &session
                .max_response_output_tokens
            {
                Some(sauropod_openai_api::RealtimeSessionMaxResponseOutputTokens::Inf) | None => {
                    None
                }
                Some(sauropod_openai_api::RealtimeSessionMaxResponseOutputTokens::Variant0(
                    max_tokens,
                )) => Some(*max_tokens),
            };
            request.response_properties.tools = session.tools.clone();
            request.instructions = session.instructions.clone();
            request
                .create_model_response_properties
                .model_response_properties
                .temperature = session.temperature;
        }

        let render_context = sauropod_prompt_templates::RenderContext::from_create_response(
            &request,
            model.get_system_prompt(),
        )?;
        let mut stream = model.generate_stream(request, render_context).await?;
        let mut is_generated_response = false;
        let mut response_id = None;
        let mut last_output_index = 0;

        loop {
            let event = stream.next().await;
            match event {
                Some(Ok(sauropod_openai_api::ResponseStreamEvent::ResponseCreatedEvent {
                    response,
                    ..
                })) => {
                    response_id = Some(response.id.clone());
                    socket
                        .send_event(RealtimeServerEvent::ResponseCreated {
                            response: response.into(),
                            event_id: make_id(),
                        })
                        .await?;
                }

                Some(Ok(sauropod_openai_api::ResponseStreamEvent::ResponseInProgressEvent {
                    ..
                })) => {}

                Some(Ok(sauropod_openai_api::ResponseStreamEvent::ResponseCompletedEvent {
                    response,
                    ..
                })) => {
                    is_generated_response = true;
                    conversation_state
                        .add_response(response.clone());

                    socket
                        .send_event(RealtimeServerEvent::ResponseDone {
                            response: response.into(),
                            event_id: make_id(),
                        })
                        .await?;
                }

                Some(Ok(
                    sauropod_openai_api::ResponseStreamEvent::ResponseOutputItemAddedEvent {
                        item,
                        output_index,
                        ..
                    },
                )) if text_modality => {
                    last_output_index = output_index + 1;
                    socket
                        .send_event(RealtimeServerEvent::ResponseOutputItemAdded {
                            output_index,
                            event_id: make_id(),
                            item: RealtimeConversationItem::from(item),
                            response_id: response_id.clone().unwrap_or_default(),
                        })
                        .await?;
                }

                Some(Ok(
                    sauropod_openai_api::ResponseStreamEvent::ResponseOutputItemDoneEvent {
                        item,
                        output_index,
                        ..
                    },
                )) if text_modality => {
                    socket
                        .send_event(RealtimeServerEvent::ResponseOutputItemDone {
                            output_index,
                            event_id: make_id(),
                            item: RealtimeConversationItem::from(item),
                            response_id: response_id.clone().unwrap_or_default(),
                        })
                        .await?;
                }

                Some(Ok(
                    sauropod_openai_api::ResponseStreamEvent::ResponseContentPartAddedEvent {
                        content_index,
                        item_id,
                        output_index,
                        part,
                        ..
                    },
                )) if text_modality => {
                    socket
                        .send_event(RealtimeServerEvent::ResponseContentPartAdded {
                            content_index,
                            event_id: make_id(),
                            item_id,
                            output_index,
                            part: part.into(),
                            response_id: response_id.clone().unwrap_or_default(),
                        })
                        .await?;
                }

                Some(Ok(
                    sauropod_openai_api::ResponseStreamEvent::ResponseContentPartDoneEvent {
                        content_index,
                        item_id,
                        output_index,
                        part,
                        ..
                    },
                )) if text_modality => {
                    socket
                        .send_event(RealtimeServerEvent::ResponseContentPartDone {
                            content_index,
                            event_id: make_id(),
                            item_id,
                            output_index,
                            part: part.into(),
                            response_id: response_id.clone().unwrap_or_default(),
                        })
                        .await?;
                }

                Some(Ok(sauropod_openai_api::ResponseStreamEvent::ResponseTextDeltaEvent {
                    content_index,
                    delta,
                    item_id,
                    output_index,
                    ..
                })) if text_modality => {
                    socket
                        .send_event(RealtimeServerEvent::ResponseTextDelta {
                            content_index,
                            event_id: make_id(),
                            delta,
                            item_id,
                            output_index,
                            response_id: response_id.clone().unwrap_or_default(),
                        })
                        .await?;
                }

                Some(Ok(sauropod_openai_api::ResponseStreamEvent::ResponseTextDoneEvent {
                    content_index,
                    item_id,
                    output_index,
                    text,
                    ..
                })) => {
                    if text_modality {
                        socket
                            .send_event(RealtimeServerEvent::ResponseTextDone {
                                content_index,
                                event_id: make_id(),
                                text: text.clone(),
                                item_id: item_id.clone(),
                                output_index,
                                response_id: response_id.clone().unwrap_or_default(),
                            })
                            .await?;
                    }

                    if audio_modality {
                        let voice_name = self.get_voice_name().await;
                        let Some(tts_model) = self.global_state.get_voice_model(&voice_name).await else {
                            anyhow::bail!("{} is not an available voice", voice_name);
                        };

                        let pcm16_audio = tts_model.enqueue(text.to_string()).await?;
                        let mut audio_bytes = Vec::with_capacity(pcm16_audio.len() * 2);
                        for sample in pcm16_audio.iter() {
                            audio_bytes.extend_from_slice(&sample.to_le_bytes());
                        }
                        let base64_audio = BASE64_STANDARD.encode(&audio_bytes);

                        socket
                            .send_event(RealtimeServerEvent::ResponseContentPartAdded { content_index: last_output_index,
                                event_id: make_id(),
                                item_id: item_id.clone(),
                                output_index: last_output_index,
                                part: sauropod_openai_api::RealtimeServerEventResponseContentPartAddedPart { audio: Some("".to_string()), text: None, transcript: Some(text.clone()), r#type: Some(sauropod_openai_api::Modalities::Audio) },
                                response_id: response_id.clone().unwrap_or_default()
                            })
                            .await?;
                        socket
                            .send_event(RealtimeServerEvent::ResponseAudioDelta {
                                content_index,
                                delta: base64_audio.clone(),
                                event_id: make_id(),
                                item_id: item_id.clone(),
                                output_index: last_output_index,
                                response_id: response_id.clone().unwrap_or_default(),
                            })
                            .await?;
                        socket
                            .send_event(RealtimeServerEvent::ResponseAudioDone {
                                content_index,
                                event_id: make_id(),
                                item_id: item_id.clone(),
                                output_index: last_output_index,
                                response_id: response_id.clone().unwrap_or_default(),
                            })
                            .await?;
                        socket
                            .send_event(RealtimeServerEvent::ResponseContentPartDone {
                                content_index,
                                event_id: make_id(),
                                item_id: item_id.clone(),
                                output_index: last_output_index,
                                part: sauropod_openai_api::RealtimeServerEventResponseContentPartDonePart { audio: Some(base64_audio), text: None, transcript: Some(text.clone()), r#type: Some(sauropod_openai_api::Modalities::Audio) },
                                response_id: response_id.clone().unwrap_or_default(),
                            })
                            .await?;

                        }
                }

                Some(Ok(sauropod_openai_api::ResponseStreamEvent::ResponseFunctionCallArgumentsDoneEvent {
                    arguments,
                    item_id,
                    output_index,
                    ..
                })) => {
                    socket
                        .send_event(RealtimeServerEvent::ResponseFunctionCallArgumentsDone {
                            event_id: make_id(),
                            arguments,
                            call_id: item_id.clone(),
                            item_id,
                            output_index,
                            response_id: response_id.clone().unwrap_or_default(),
                        })
                        .await?;
                }

                Some(Ok(sauropod_openai_api::ResponseStreamEvent::ResponseFunctionCallArgumentsDeltaEvent {
                    delta,
                    item_id,
                    output_index,
                    ..
                })) => {
                    socket
                        .send_event(RealtimeServerEvent::ResponseFunctionCallArgumentsDelta {
                            event_id: make_id(),
                            delta,
                            call_id: item_id.clone(),
                            item_id,
                            output_index,
                            response_id: response_id.clone().unwrap_or_default(),
                        })
                        .await?;
                }

                None => {
                    tracing::info!("Response stream ended");
                    break;
                }
                _ => {
                    tracing::error!("Received unhandled event: {event:?}");
                }
            }
        }

        if !is_generated_response {
            // TODO remove the last conversation item
            tracing::error!("No response was generated from the model");
        }

        Ok(())
    }

    async fn process_audio_to_items(
        self: &Arc<Self>,
        audio_buffer: &mut AudioBuffer,
        audio_range: Range<usize>,
        item_id: String,
        socket: &SocketWrapper,
    ) -> anyhow::Result<()> {
        let text = self
            .process_audio(audio_buffer, audio_range.clone())
            .await?;

        // TODO populate logprobs and content_index
        // let avg_logprob = if self
        //     .session
        //     .include
        //     .as_ref()
        //     .map(|includes| {
        //         includes.contains(&"item.input_audio_transcription.logprobs".to_string())
        //     })
        //     .unwrap_or(false)
        // {
        //     Some(segment.decoding_result.avg_logprob)
        // } else {
        //     None
        // };

        socket
            .send_event(
                RealtimeServerEvent::ConversationItemInputAudioTranscriptionCompleted {
                    event_id: crate::make_id(),
                    item_id,
                    transcript: text.clone(),
                    logprobs: None,   // TODO: Implement logprobs
                    content_index: 0, // TODO: Implement content_index
                    usage: sauropod_openai_api::RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage::TranscriptTextUsageTokens(sauropod_openai_api::TranscriptTextUsageTokens { input_token_details: None, input_tokens: 0, output_tokens: 0, total_tokens: 0, r#type: sauropod_openai_api::TranscriptTextUsageTokensType::Tokens }),
                },
            )
            .await?;

        // Consume the audio buffer after processing
        audio_buffer.consume_from_buffer(audio_range);

        // Add the audio message to the conversation
        {
            let cloned_self = Arc::clone(self);
            let cloned_socket = socket.clone();
            std::mem::drop(tokio::spawn(async move {
                let mut conversation_state = cloned_self.conversation.lock().await;
                conversation_state.add_user_message(text.trim());
                if let Err(e) = cloned_self
                    .create_response(&mut conversation_state, cloned_socket)
                    .await
                {
                    tracing::error!("Failed to create response: {e:#?}");
                }
            }));
        }
        Ok(())
    }
}
