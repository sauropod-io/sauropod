//! Speech to Text (STT) module

use tracing::Instrument as _;

use sauropod_inference_thread::BatchInferenceThread;

mod parakeet;
mod voxtral;

/// Speech to Text (STT) inference thread.
pub type SttThread = BatchInferenceThread<Vec<f32>, String>;

/// Create a new STT inference thread.
pub async fn make_stt_thread(
    env: &sauropod_onnxruntime::Env,
    stt_config: &sauropod_config::SpeechToTextConfig,
) -> anyhow::Result<SttThread> {
    match stt_config {
        sauropod_config::SpeechToTextConfig::Parakeet { model } => {
            let stt_model_dir = parakeet::download_from_huggingface(model)
                .instrument(tracing::info_span!("download STT model"))
                .await?;
            let provider = parakeet::Parakeet::new(env, &stt_model_dir).await?;
            Ok(BatchInferenceThread::new(
                "parakeet".to_string(),
                1,
                provider,
            )?)
        }
        sauropod_config::SpeechToTextConfig::Voxtral {
            model,
            multimodal_projector,
        } => {
            let provider = voxtral::Voxtral::new(model, multimodal_projector).await?;
            Ok(BatchInferenceThread::new(
                "voxtral".to_string(),
                1,
                provider,
            )?)
        }
    }
}
