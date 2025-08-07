use std::sync::Arc;

use axum::{extract::Query, response::IntoResponse};
use webrtc::{
    data_channel::{RTCDataChannel, data_channel_message::DataChannelMessage},
    rtp_transceiver::rtp_codec::RTCRtpCodecParameters,
};

const EVENTS_CHANNEL_LABEL: &str = "oai-events";

pub struct WebSocketInterface {
    /// The oai-events data channel.
    outgoing_message_tx: tokio::sync::mpsc::Sender<axum::extract::ws::Message>,
    /// Channel for receiving incoming messages from the data channel.
    incoming_message_rx: tokio::sync::mpsc::Receiver<axum::extract::ws::Message>,
}

impl tokio_stream::Stream for WebSocketInterface {
    type Item = Result<axum::extract::ws::Message, axum::Error>;
    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match this.incoming_message_rx.poll_recv(cx) {
            std::task::Poll::Ready(Some(msg)) => std::task::Poll::Ready(Some(Ok(msg))),
            std::task::Poll::Ready(None) => std::task::Poll::Ready(None),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

impl futures::Sink<axum::extract::ws::Message> for WebSocketInterface {
    type Error = axum::Error;

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        // The mpsc::Sender is bounded, so we check if it's ready by using try_reserve.
        // If the channel is full, we return Pending.
        let this = self.get_mut();
        if this.outgoing_message_tx.capacity() > 0 {
            std::task::Poll::Ready(Ok(()))
        } else {
            std::task::Poll::Pending
        }
    }

    fn start_send(
        self: std::pin::Pin<&mut Self>,
        item: axum::extract::ws::Message,
    ) -> Result<(), Self::Error> {
        let this = self.get_mut();
        this.outgoing_message_tx
            .try_send(item)
            .map_err(|e| axum::Error::new(std::io::Error::other(format!("mpsc send error: {e}"))))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        // mpsc::Sender does not buffer
        std::task::Poll::Ready(Ok(()))
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        let this = self.get_mut();
        match this.outgoing_message_tx.is_closed() {
            true => std::task::Poll::Ready(Ok(())),
            false => std::task::Poll::Pending,
        }
    }
}

impl crate::SocketLike for WebSocketInterface {}

/// Model for the WebRTC connection.
#[derive(serde::Deserialize, utoipa::IntoParams)]
pub struct ModelParam {
    /// The model of the realtime.
    #[serde(rename = "model", default = "ModelParam::default_model")]
    _model: String,
}

impl ModelParam {
    /// The default model for realtime connections.
    fn default_model() -> String {
        "default".to_string()
    }
}

/// Handler for WebRTC sessions.
pub async fn post_v1_realtime_impl(
    sdp_offer: String,
    state: Arc<sauropod_global_state::GlobalState>,
) -> anyhow::Result<axum::response::Response> {
    let offer =
        webrtc::peer_connection::sdp::session_description::RTCSessionDescription::offer(sdp_offer)?;

    let mut media_engine = webrtc::api::media_engine::MediaEngine::default();
    media_engine.register_codec(
        RTCRtpCodecParameters {
            capability: webrtc::rtp_transceiver::rtp_codec::RTCRtpCodecCapability {
                mime_type: webrtc::api::media_engine::MIME_TYPE_PCMA.to_owned(),
                clock_rate: 8_000,
                channels: 0,
                ..Default::default()
            },
            payload_type: 8,
            ..Default::default()
        },
        webrtc::rtp_transceiver::rtp_codec::RTPCodecType::Audio,
    )?;

    // Create the API object with the MediaEngine
    let api = webrtc::api::APIBuilder::new()
        .with_media_engine(media_engine)
        .build();

    // Prepare the configuration
    let config = webrtc::peer_connection::configuration::RTCConfiguration {
        ice_servers: vec![
            webrtc::ice_transport::ice_server::RTCIceServer {
                urls: vec!["stun:stun.l.google.com:19302".to_owned()],
                ..Default::default()
            },
            webrtc::ice_transport::ice_server::RTCIceServer {
                urls: vec!["stun:stun2.l.google.com:19302".to_owned()],
                ..Default::default()
            },
            webrtc::ice_transport::ice_server::RTCIceServer {
                urls: vec!["stun:stun.services.mozilla.com:3478".to_owned()],
                ..Default::default()
            },
        ],
        ..Default::default()
    };

    // Create a new RTCPeerConnection
    let peer_connection = Arc::new(api.new_peer_connection(config).await?);

    let (incoming_message_tx, incoming_message_rx) =
        tokio::sync::mpsc::channel::<axum::extract::ws::Message>(16);
    let (outgoing_message_tx, outgoing_message_rx) =
        tokio::sync::mpsc::channel::<axum::extract::ws::Message>(16);
    let data_channel_incoming_message_tx = incoming_message_tx.clone();

    let websocket_interface = WebSocketInterface {
        outgoing_message_tx,
        incoming_message_rx,
    };

    // Set the handler for Peer connection state
    // This will notify you when the peer has connected/disconnected
    peer_connection.on_peer_connection_state_change(Box::new(
        move |s: webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState| {
            tracing::info!("Peer Connection State has changed: {s}");

            if s == webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState::Failed
                && let Err(e) =
                    incoming_message_tx.blocking_send(axum::extract::ws::Message::Close(None))
            {
                tracing::error!("Failed to send close message: {e}");
            }

            Box::pin(async {})
        },
    ));

    peer_connection.on_track(Box::new(move |track, _rtp_rcv, _rtp_tx| {
        tracing::info!("Track received {}: {track:#?}", track.id());

        if track.kind() != webrtc::rtp_transceiver::rtp_codec::RTPCodecType::Audio {
            tracing::warn!("Received non-audio track: {track:#?}");
            return Box::pin(async {});
        }

        Box::pin(async move {
            // Handle the track here,
            loop {
                let mut b = vec![0; 4096];
                match track.read(&mut b).await {
                    Ok((packet, map)) => {
                        tracing::trace!("Received packet: {packet:#?}, map: {map:#?}");
                    }
                    Err(e) => {
                        tracing::error!("Error reading track: {e}");
                        break;
                    }
                }
            }
        })
    }));

    // Register data channel creation handling
    let outgoing_message_rx = Arc::new(tokio::sync::Mutex::new(outgoing_message_rx));
    peer_connection.on_data_channel(Box::new(move |data_channel: Arc<RTCDataChannel>| {
        let channel_label = data_channel.label().to_owned();
        let channel_id = data_channel.id();

        if channel_label == EVENTS_CHANNEL_LABEL {
            let data_channel_incoming_message_tx = data_channel_incoming_message_tx.clone();
            let outgoing_message_rx = outgoing_message_rx.clone();
            Box::pin(async move {
                let channel_label_clone = channel_label.clone();
                let channel_id_clone = channel_id;
                data_channel.on_close(Box::new(move || {
                    tracing::info!("Data channel closed");
                    Box::pin(async {})
                }));

                let data_channel_for_send = data_channel.clone();
                data_channel.on_open(Box::new(move || {
                    tracing::info!("Data channel {channel_label_clone} {channel_id_clone} open");

                    Box::pin(async move {
                        let mut outgoing_queue = outgoing_message_rx.lock().await;
                        while let Some(outgoing) = outgoing_queue.recv().await {
                            tracing::trace!("Sending message on data channel {channel_label} {channel_id}: {outgoing:?}");
                            match outgoing {
                                axum::extract::ws::Message::Text(text) => {
                                    if let Err(err) = data_channel_for_send.send(&text.into()).await {
                                        tracing::error!("Failed to send text message on data channel: {err}");
                                    }
                                }
                                axum::extract::ws::Message::Binary(data) => {
                                    if let Err(err) = data_channel_for_send.send(&data).await {
                                        tracing::error!("Failed to send binary message on data channel: {err}");
                                    }
                                }
                                axum::extract::ws::Message::Close(_) => {
                                    if let Err(err) = data_channel_for_send.close().await {
                                        tracing::error!("Failed to close data channel: {err}");
                                    }
                                }
                                _ => {
                                    tracing::warn!("Unsupported message type on data channel: {outgoing:?}");
                                }
                            }
                        }
                     })
                }));

                // Register text message handling
                data_channel.on_message(Box::new(move |msg: DataChannelMessage| {
                    let data_channel_incoming_message_tx = data_channel_incoming_message_tx.clone();
                    Box::pin(async move {
                        let Ok(data) = msg.data.try_into() else {
                            tracing::error!("Failed to convert DataChannel message to String");
                            return;
                        };
                        if let Err(err) = data_channel_incoming_message_tx.send(axum::extract::ws::Message::Text(data)).await {
                            tracing::error!("Failed to send message to incoming_message_tx: {err}");
                        }
                    })
                }));
            })
        } else {
            tracing::error!(
                "Received unexpected data channel with label: {channel_label}, id: {channel_id}. Expected only {EVENTS_CHANNEL_LABEL}"
            );
            Box::pin(async move {
                // Ignore other channels
            })
        }
    }));

    peer_connection.set_remote_description(offer).await?;

    let answer = peer_connection.create_answer(None).await?;
    // Create channel that is blocked until ICE Gathering is complete
    let mut gather_complete = peer_connection.gathering_complete_promise().await;
    // Sets the LocalDescription, and starts our UDP listeners
    peer_connection.set_local_description(answer).await?;
    // TODO switch to OnICECandidate events
    let _ = gather_complete.recv().await;

    // Return the local session description as a response
    if let Some(local_desc) = peer_connection.local_description().await {
        let mut response = local_desc.sdp.into_response();
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/sdp"),
        );

        #[allow(clippy::let_underscore_future)]
        // TODO allow intent to select mode
        let _ = tokio::spawn(crate::handle_realtime_socket::<crate::Transcription, _>(
            websocket_interface,
            state,
        ));
        Ok(response)
    } else {
        anyhow::bail!("Failed to generate local session description");
    }
}

/// Handler for WebRTC sessions.
#[utoipa::path(
    post,
    path = "/v1/realtime",
    tag = "Realtime",
    responses(
        (status = 200, description = "Session description", body = String, content_type = "application/sdp"),
        (status = 500, description = "Error occured", body = sauropod_inference_http::Error)
    ),
    request_body(content = String, description = "Session description", content_type = "application/sdp"),
    params(ModelParam)
)]
pub async fn post_v1_realtime(
    axum::extract::State(state): sauropod_global_state::AxumGlobalState,
    _query: Query<ModelParam>,
    sdp_offer: String,
) -> impl IntoResponse {
    match post_v1_realtime_impl(sdp_offer, state).await {
        Ok(response) => response,
        Err(err) => {
            tracing::error!("Error processing WebRTC client: {:?}", err);
            axum::response::Json(sauropod_inference_http::Error {
                error: "Error processing WebRTC client".to_string(),
            })
            .into_response()
        }
    }
}

//// Used to get a token for WebRTC connections.
#[utoipa::path(
    post,
    path = "/v1/realtime/sessions",
    tag = "Realtime",
    request_body = sauropod_openai_api::RealtimeSessionCreateRequest,
    responses(
        (status = 200, description = "Session created", body = sauropod_openai_api::RealtimeSessionCreateResponse),
        (status = 500, description = "Error occured", body = sauropod_inference_http::Error)
    )

)]
pub async fn v1_realtime_sessions(
    axum::extract::State(_state): sauropod_global_state::AxumGlobalState,
    axum::extract::Json(_body): axum::extract::Json<
        sauropod_openai_api::RealtimeSessionCreateRequest,
    >,
) -> impl IntoResponse {
    axum::response::Json(sauropod_openai_api::RealtimeSessionCreateResponse {
        client_secret: sauropod_openai_api::RealtimeSessionCreateResponseClientSecret {
            expires_at: 0, // TODO
            value: crate::make_id(),
        },
        input_audio_format: None,
        input_audio_transcription: None,
        instructions: None,
        max_response_output_tokens: None,
        modalities: None,
        output_audio_format: None,
        speed: None,
        temperature: None,
        tool_choice: None,
        tools: None,
        tracing: None,
        turn_detection: None,
        voice: None,
    })
    .into_response()
}
