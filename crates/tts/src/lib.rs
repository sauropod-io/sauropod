//! Text to Speech (TTS) module

use std::sync::Arc;

pub mod kokoro;
pub mod orpheus;

/// Text to Speech (TTS) inference thread.
pub type TtsThread = sauropod_inference_thread::BatchInferenceThread<TtsRequest, Vec<i16>>;

/// Handle for a TTS thread with a specific voice.
pub struct ConfiguredTtsThread {
    /// The TTS inference thread.
    thread: Arc<TtsThread>,
    /// The name of the voice to use.
    voice: Option<String>,
}

impl ConfiguredTtsThread {
    /// Create a new configured TTS thread.
    pub fn new(thread: Arc<TtsThread>, voice: Option<String>) -> Arc<Self> {
        Arc::new(Self { thread, voice })
    }

    /// Put an input into the queue for processing.
    pub async fn enqueue(
        &self,
        input: String,
    ) -> anyhow::Result<<orpheus::Orpheus as sauropod_inference_thread::InferenceProvider>::Output>
    {
        self.thread
            .enqueue(TtsRequest {
                text: input,
                voice: self.voice.clone(),
            })
            .await
    }
}

pub struct TtsRequest {
    /// The text to synthesize.
    pub text: String,
    /// The voice to use for synthesis.
    pub voice: Option<String>,
}
