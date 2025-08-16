/// Call the text to speech model.
pub async fn call_speech_to_text_model(
    audio_data: Vec<f32>,
    global_state: &sauropod_global_state::GlobalState,
) -> anyhow::Result<String> {
    if let Some(stt_model) = global_state.get_loaded_models().stt_model.clone() {
        match stt_model.enqueue(audio_data.clone()).await {
            Ok(text) => Ok(text),
            Err(e) => {
                tracing::warn!("Speech to text transcription failed: {e}");
                anyhow::bail!("Speech to text transcription failed: {e}")
            }
        }
    } else {
        anyhow::bail!(
            "The LLM does not support speech-to-text and no speech-to-text model is loaded"
        )
    }
}
