//! Socket wrapper.
use futures::sink::SinkExt as _;

pub(crate) trait SocketLike:
    tokio_stream::Stream<Item = Result<axum::extract::ws::Message, axum::Error>>
    + futures::Sink<axum::extract::ws::Message, Error = axum::Error>
{
}

impl SocketLike for axum::extract::ws::WebSocket {}

/// Type alias for a socket.
pub(crate) type DynSocket = dyn SocketLike + Unpin + Send + 'static;

#[derive(Clone)]
pub(crate) struct SocketWrapper {
    /// The socket wrapped in an `Arc<Mutex<>>` to allow concurrent access.
    socket: std::sync::Arc<tokio::sync::Mutex<Box<DynSocket>>>,
}

impl SocketWrapper {
    /// Create a new `SocketWrapper` with the given socket.
    pub(crate) fn new(socket: Box<DynSocket>) -> Self {
        Self {
            socket: std::sync::Arc::new(tokio::sync::Mutex::new(socket)),
        }
    }

    /// Send an event to the socket.
    #[tracing::instrument(skip(self, event), level = "debug")]
    pub(crate) async fn send_event(
        &self,
        event: sauropod_openai_api::RealtimeServerEvent,
    ) -> anyhow::Result<()> {
        match &event {
            sauropod_openai_api::RealtimeServerEvent::ResponseAudioDelta {
                content_index,
                response_id,
                ..
            } => {
                tracing::debug!(
                    "Sending event: ResponseAudioDeltaEvent {{ content_index: {content_index:?}, response_id: {response_id:?}, .. }}"
                );
            }
            sauropod_openai_api::RealtimeServerEvent::ResponseContentPartDone {
                content_index,
                response_id,
                ..
            } => {
                tracing::debug!(
                    "Sending event: ResponseContentPartDone {{ content_index: {content_index:?}, response_id: {response_id:?}, .. }}"
                );
            }
            _ => {
                tracing::debug!("Sending event: {:?}", &event);
            }
        }
        let json = serde_json::to_string(&event)?;
        self.socket
            .lock()
            .await
            .send(axum::extract::ws::Message::Text(json.into()))
            .await?;
        Ok(())
    }

    pub(crate) async fn send_message(
        &self,
        message: axum::extract::ws::Message,
    ) -> Result<(), axum::Error> {
        self.socket.lock().await.send(message).await
    }

    /// Get the next message from the socket.
    pub(crate) async fn next(&self) -> Option<Result<axum::extract::ws::Message, axum::Error>> {
        use tokio_stream::StreamExt as _;

        let mut socket = self.socket.lock().await;

        socket.next().await
    }
}
