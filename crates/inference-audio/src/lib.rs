use axum::response::sse::Event as SseEvent;
use axum::{response::IntoResponse, response::Sse};
use base64::Engine as _;
use tokio_stream::StreamExt;

use sauropod_openai_api::{CreateSpeechRequest, CreateSpeechRequestStreamFormat};

mod routes;
pub use routes::*;

async fn run_tts(
    global_state: std::sync::Arc<sauropod_global_state::GlobalState>,
    request: CreateSpeechRequest,
) -> anyhow::Result<sauropod_tts::AudioReceiver> {
    let voice = request.voice.0.as_str();
    let Some(tts_model) = global_state.get_voice_model(voice).await else {
        anyhow::bail!("{} is not an available voice", voice);
    };
    tts_model.enqueue(request.input).await
}

async fn create_speech_impl_stream(
    global_state: std::sync::Arc<sauropod_global_state::GlobalState>,
    request: CreateSpeechRequest,
) -> anyhow::Result<axum::response::Response> {
    let stream = run_tts(global_state, request).await?;

    let stream = async_stream::stream! {
        // Wrap the tokio mpsc receiver into a Stream
        let mut audio_stream = tokio_stream::wrappers::ReceiverStream::new(stream);

        while let Some(item) = audio_stream.next().await {
            let samples = item.map_err(axum::Error::new)?;
            // Convert i16 samples to little-endian bytes
            let mut bytes = Vec::with_capacity(samples.len() * 2);
            for s in samples {
                bytes.extend_from_slice(&s.to_le_bytes());
            }

            // Base64-encode using the recommended engine API
            let b64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
            yield Ok::<_, axum::Error>(SseEvent::default().json_data(sauropod_openai_api::CreateSpeechResponseStreamEvent::SpeechAudioDeltaEvent{
                audio: b64,
            }).unwrap());
        }

        yield Ok::<_, axum::Error>(SseEvent::default().json_data(sauropod_openai_api::CreateSpeechResponseStreamEvent::SpeechAudioDoneEvent {
            usage: sauropod_openai_api::SpeechAudioDoneEventUsage {
                input_tokens: 0,
                output_tokens: 0,
                total_tokens: 0,
            },
        }).unwrap());
    };

    Ok(Sse::new(stream).into_response())
}

async fn create_speech_impl_audio(
    global_state: std::sync::Arc<sauropod_global_state::GlobalState>,
    request: CreateSpeechRequest,
) -> anyhow::Result<axum::response::Response> {
    let mut stream = run_tts(global_state, request).await?;
    let mut full_data: Vec<u8> = Vec::with_capacity(4096);
    while let Some(data) = stream.recv().await {
        full_data.extend(data?.into_iter().flat_map(|x| x.to_le_bytes()));
    }

    Ok(axum::response::Response::builder()
        .header("Content-Type", "application/octet-stream")
        .body(axum::body::Body::from(full_data))?)
}

pub async fn create_speech_impl(
    global_state: std::sync::Arc<sauropod_global_state::GlobalState>,
    request: CreateSpeechRequest,
) -> anyhow::Result<axum::response::Response> {
    if !matches!(
        request.response_format,
        Some(sauropod_openai_api::CreateSpeechRequestResponseFormat::Pcm)
    ) {
        anyhow::bail!("Currently the speech endpoint only supports PCM output");
    }

    match &request.stream_format {
        Some(CreateSpeechRequestStreamFormat::Audio) | None => {
            create_speech_impl_audio(global_state, request).await
        }
        Some(CreateSpeechRequestStreamFormat::Sse) => {
            create_speech_impl_stream(global_state, request).await
        }
    }
}
