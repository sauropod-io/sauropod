use std::sync::Arc;

use axum::{
    extract::{
        Query,
        ws::{Message, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use base64::prelude::*;
use serde::Deserialize;
use tracing::Instrument as _;
use uuid::Uuid;

use sauropod_openai_api::{
    RealtimeClientEvent, RealtimeServerEvent, RealtimeServerEventErrorError,
};

mod audio;
mod defaults;
mod realtime;
pub mod realtime_webrtc;
mod socket;
mod transcription;
use socket::*;
mod model_calling;
pub(crate) use transcription::Transcription;

fn make_id() -> String {
    Uuid::new_v4().to_string()
}

/// Trait for real-time functionality.
trait RealtimeFunctionality: Sized {
    /// Create a realtime session.
    async fn new(
        id: String,
        global_state: Arc<sauropod_global_state::GlobalState>,
    ) -> anyhow::Result<Self>;

    /// Send a session created event.
    async fn session_created(&self, socket: &SocketWrapper) -> anyhow::Result<()>;

    /// Process a message from the client.
    async fn process_message(
        self: &Arc<Self>,
        event: RealtimeClientEvent,
        socket: &SocketWrapper,
    ) -> anyhow::Result<()>;

    /// Process audio input from the client.
    async fn process_input_audio(
        self: &Arc<Self>,
        audio_pcm16: Vec<i16>,
        socket: &SocketWrapper,
    ) -> anyhow::Result<()>;
}

pub(crate) async fn handle_realtime_socket<Session: RealtimeFunctionality, S>(
    socket: S,
    global_state: Arc<sauropod_global_state::GlobalState>,
) -> anyhow::Result<()>
where
    S: SocketLike + Unpin + Send + 'static,
{
    let socket = SocketWrapper::new(Box::new(socket));
    let id = make_id();
    tracing::info!("Created new real-time session with ID: {id}");
    let session = Arc::new(Session::new(id.clone(), global_state).await?);
    session.session_created(&socket).await?;

    // Main WebSocket message processing loop
    'main: loop {
        match socket
            .next()
            .instrument(tracing::debug_span!("waiting for message", session_id = id))
            .await
        {
            // Received a message within the timeout period
            Some(Ok(msg)) => {
                match msg {
                    Message::Text(text) => {
                        let client_event = match serde_json::from_str::<RealtimeClientEvent>(&text)
                        {
                            Ok(event) => event,
                            Err(e) => {
                                tracing::info!("Failed to parse client message: {}", e);
                                let error_event = RealtimeServerEvent::Error {
                                    event_id: make_id(),
                                    error: RealtimeServerEventErrorError {
                                        message: format!("Failed to parse message: {e}"),
                                        code: Some("parse_error".to_string()),
                                        event_id: None,
                                        r#type: "invalid_request_error".to_string(),
                                        param: None,
                                    },
                                };

                                socket.send_event(error_event).await?;
                                return Err(e.into());
                            }
                        };
                        match client_event {
                            RealtimeClientEvent::InputAudioBufferAppend { audio, .. } => {
                                // tracing::debug!(
                                //     "Received client event: InputAudioBufferAppend {{ .. }}"
                                // );

                                if audio.is_empty() {
                                    tracing::warn!("Received empty audio data");
                                    return Ok(());
                                }
                                let decoded_audio = BASE64_STANDARD.decode(audio).map_err(|e| {
                                    anyhow::anyhow!("Failed to decode base64 audio data: {e}")
                                })?;

                                if decoded_audio.len() % 2 != 0 {
                                    return Err(anyhow::anyhow!(
                                        "Audio data length is not even - this would seem to indicate it's not 16bit PCM"
                                    ));
                                }

                                let audio_data_i16: Vec<i16> = decoded_audio
                                    .chunks_exact(2)
                                    .map(|x| i16::from_le_bytes([x[0], x[1]]))
                                    .collect();

                                session
                                    .process_input_audio(audio_data_i16, &socket)
                                    .instrument(tracing::info_span!("add_audio"))
                                    .await?;
                            }
                            client_event => {
                                tracing::debug!("Received client event: {:?}", client_event);

                                session
                                    .process_message(client_event, &socket)
                                    .instrument(tracing::debug_span!(
                                        "process_message",
                                        session_id = id
                                    ))
                                    .await?;
                            }
                        }
                    }
                    Message::Binary(_) => {
                        tracing::error!("Client sent a binary message (only text is supported)");

                        let error_event = RealtimeServerEvent::Error {
                            event_id: make_id(),
                            error: RealtimeServerEventErrorError {
                                message: "Invalid data - received a binary message".to_string(),
                                code: Some("invalid_data".to_string()),
                                r#type: "server_error".to_string(),
                                event_id: None,
                                param: None,
                            },
                        };

                        if socket.send_event(error_event).await.is_err() {
                            tracing::info!("Failed to send invalid data error message");
                        }
                    }
                    Message::Close(_) => {
                        tracing::info!(
                            "Client closed connection for session {session_id}",
                            session_id = id
                        );
                        break 'main;
                    }
                    Message::Ping(x) => {
                        socket.send_message(Message::Pong(x)).await?;
                    }
                    Message::Pong(_) => {
                        // Ignore pong messages
                    }
                }
            }
            // Error receiving message
            Some(Err(e)) => {
                tracing::error!(
                    "Error receiving message for session {session_id}: {e}",
                    session_id = id
                );
                break 'main;
            }
            // No more messages (None)
            None => {
                tracing::info!(
                    "WebSocket connection closed by client for session {session_id}",
                    session_id = id
                );
                break 'main;
            }
        }
    }

    // Clean up resources
    tracing::info!(
        "WebSocket connection closed for session {session_id}",
        session_id = id
    );
    Ok(())
}

/// Intent for WebSocket connections.
#[derive(Deserialize, Default, utoipa::ToSchema)]
enum RealtimeIntent {
    /// Realtime voice to voice.
    #[default]
    #[serde(rename = "realtime")]
    Realtime,
    /// Real-time audio transcription.
    #[serde(rename = "transcription")]
    Transcription,
}

/// Parameters for the WebSocket connection.
#[derive(Deserialize, utoipa::IntoParams)]
pub struct RealtimeParams {
    /// The intent of the realtime.
    #[serde(default)]
    intent: RealtimeIntent,
    /// The intent of the realtime.
    #[serde(rename = "model", default = "RealtimeParams::default_model")]
    _model: String,
}

impl RealtimeParams {
    /// The default model for realtime connections.
    fn default_model() -> String {
        "default".to_string()
    }
}

/// WebSocket handler for real-time connections.
///
/// This endpoint accepts WebSocket connections for real-time audio transcription.
/// The client can send audio data and configuration messages, and receive transcription results.
#[utoipa::path(
    get,
    path = "/v1/realtime",
    tag = "Realtime",
    responses(
        (status = 101, description = "Switching Protocols - WebSocket connection established")
    ),
    params(RealtimeParams)
)]
pub async fn get_v1_realtime(
    ws: WebSocketUpgrade,
    axum::extract::State(state): sauropod_global_state::AxumGlobalState,
    Query(realtime_params): Query<RealtimeParams>,
) -> impl IntoResponse {
    ws.on_upgrade(async move |ws| {
        let result = match realtime_params.intent {
            RealtimeIntent::Realtime => {
                handle_realtime_socket::<realtime::RealtimeSessionState, _>(ws, state).await
            }
            RealtimeIntent::Transcription => {
                handle_realtime_socket::<Transcription, _>(ws, state).await
            }
        };
        if let Err(err) = result {
            tracing::error!("Error handling WebSocket: {err:?}");
        }
    })
}
