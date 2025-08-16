//! Realtime transcription.

use std::sync::Arc;

use tracing::Instrument as _;

use sauropod_openai_api::{
    RealtimeClientEvent, RealtimeServerEvent, RealtimeSessionTurnDetectionType,
};

use crate::{audio::AudioBuffer, socket::SocketWrapper};

pub(crate) struct Transcription {
    /// The session configuration.
    pub(crate) session:
        tokio::sync::Mutex<sauropod_openai_api::RealtimeTranscriptionSessionCreateResponse>,
    /// The audio buffer for this session.
    audio_buffer: tokio::sync::Mutex<AudioBuffer>,
    /// The loaded models.
    global_state: Arc<sauropod_global_state::GlobalState>,
}

impl Transcription {
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
            let audio_data = audio_buffer
                .range(vad_result.range)
                .copied()
                .collect::<Vec<_>>();
            let text =
                crate::model_calling::call_speech_to_text_model(audio_data, &self.global_state)
                    .await?;

            tracing::info!(
                "Transcription completed for item_id: {}, text: {text}",
                vad_result.item_id
            );

            socket
            .send_event(
                RealtimeServerEvent::ConversationItemInputAudioTranscriptionCompleted {
                    event_id: crate::make_id(),
                    item_id: vad_result.item_id,
                    transcript: text.clone(),
                    logprobs: None,
                    // Each transcription is a new item so the content index is always 0.
                    content_index: 0,
                    usage: sauropod_openai_api::RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage::TranscriptTextUsageTokens(sauropod_openai_api::TranscriptTextUsageTokens { input_token_details: None, input_tokens: 0, output_tokens: 0, total_tokens: 0, r#type: sauropod_openai_api::TranscriptTextUsageTokensType::Tokens }),
                },
            )
            .await?;
        }
        Ok(())
    }
}

impl crate::RealtimeFunctionality for Transcription {
    async fn new(
        id: String,
        global_state: Arc<sauropod_global_state::GlobalState>,
    ) -> anyhow::Result<Self> {
        let config = sauropod_openai_api::RealtimeTranscriptionSessionCreateResponse {
            id,
            client_secret: None,
            input_audio_format: Some(sauropod_openai_api::InputAudioStreamFormat::Pcm16),
            modalities: None,
            turn_detection: crate::defaults::make_default_vad(),
            input_audio_transcription: None,
        };

        Ok(Self {
            session: tokio::sync::Mutex::new(config),
            audio_buffer: tokio::sync::Mutex::new(AudioBuffer::new(&global_state)),
            global_state,
        })
    }

    async fn session_created(&self, socket: &SocketWrapper) -> anyhow::Result<()> {
        socket
            .send_event(RealtimeServerEvent::TranscriptionSessionCreated {
                event_id: crate::make_id(),
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
            // For transcription mode this is how the session is configured.
            RealtimeClientEvent::TranscriptionSessionUpdate { session, .. } => {
                let mut inner_session = self.session.lock().await;

                macro_rules! update_field {
                    ($field:ident, $inner:ident, $session:expr) => {
                        if let Some(value) = $session.$field {
                            $inner.$field = Some(value.into());
                        }
                    };
                }

                update_field!(input_audio_format, inner_session, session);
                update_field!(input_audio_transcription, inner_session, session);
                update_field!(modalities, inner_session, session);
                update_field!(turn_detection, inner_session, session);

                socket
                    .send_event(RealtimeServerEvent::TranscriptionSessionUpdated {
                        event_id: crate::make_id(),
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
