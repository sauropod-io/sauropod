//! Text to Speech (TTS) module

use std::sync::Arc;

pub mod kokoro;
pub mod orpheus;

/// Receiver for audio data produced by the TTS thread.
pub type AudioReceiver = tokio::sync::mpsc::Receiver<anyhow::Result<Vec<i16>>>;

/// Sender for audio data produced by the TTS thread.
pub type AudioSender = tokio::sync::mpsc::Sender<anyhow::Result<Vec<i16>>>;

/// Handle for a TTS thread.
pub struct TtsThread {
    /// The TTS inference thread.
    _thread: std::thread::JoinHandle<()>,
    /// The sender for TTS requests.
    tx: tokio::sync::mpsc::Sender<TtsRequest>,
}

impl TtsThread {
    /// Create a new configured TTS thread.
    fn new(provider: Box<dyn TtsProvider + Send>) -> anyhow::Result<Arc<Self>> {
        let name = provider.name().to_string();
        let (tx, mut rx) = tokio::sync::mpsc::channel::<TtsRequest>(8);
        let _thread = std::thread::Builder::new().name(name).spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime")
                .block_on(async move {
                    while let Some(request) = rx.recv().await {
                        let (audio_sender, audio_receiver) =
                            tokio::sync::mpsc::channel::<anyhow::Result<Vec<i16>>>(16);
                        if let Err(e) = request.sender.send(audio_receiver) {
                            tracing::warn!(
                                "Failed to send audio receiver back to the requester: {e:?}"
                            );
                            continue;
                        }

                        let provider_result = provider
                            .process(request.text, request.voice, &audio_sender)
                            .await;
                        if let Err(e) = provider_result {
                            tracing::error!("Error processing TTS request: {e:?}");
                            let _ = audio_sender.send(Err(e)).await;
                        }
                    }
                });
        })?;
        Ok(Arc::new(Self { _thread, tx }))
    }

    /// Put an input into the queue for processing.
    pub async fn enqueue(
        &self,
        input: String,
        voice: Option<String>,
    ) -> anyhow::Result<AudioReceiver> {
        let (tx, rx) = tokio::sync::oneshot::channel::<AudioReceiver>();
        self.tx
            .send(TtsRequest {
                text: input,
                voice,
                sender: tx,
            })
            .await?;
        match rx.await {
            Ok(handle) => Ok(handle),
            Err(_) => {
                anyhow::bail!("Error awaiting response")
            }
        }
    }
}

pub struct ConfiguredTtsThread {
    /// The TTS thread.
    thread: Arc<TtsThread>,
    /// The voice to use.
    voice: Option<String>,
}

impl ConfiguredTtsThread {
    /// Create a new configured TTS thread.
    pub fn new(thread: Arc<TtsThread>, voice: Option<String>) -> Arc<Self> {
        Arc::new(Self { thread, voice })
    }

    /// Enqueue a text input for synthesis.
    pub async fn enqueue(&self, input: String) -> anyhow::Result<AudioReceiver> {
        self.thread.enqueue(input, self.voice.clone()).await
    }
}

pub struct TtsRequest {
    /// The text to synthesize.
    pub text: String,
    /// The voice to use for synthesis.
    pub voice: Option<String>,
    /// The sender to return the output.
    sender: tokio::sync::oneshot::Sender<AudioReceiver>,
}

#[async_trait::async_trait]
trait TtsProvider {
    /// Get the name of the TTS provider.
    fn name(&self) -> &'static str;

    /// Process a TTS request for processing.
    async fn process(
        &self,
        text: String,
        voice: Option<String>,
        sender: &'async_trait AudioSender,
    ) -> anyhow::Result<()>;
}
