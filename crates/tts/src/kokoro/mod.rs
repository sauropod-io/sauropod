//! Text to Speech (TTS) module

use anyhow::Context as _;

use sauropod_huggingface::{HuggingfaceRepo, RepositoryInterface};
use sauropod_onnxruntime::{MemoryInfo, Session};

mod tokenizer;
pub use tokenizer::Tokenizer;

const MODEL_FILENAME: &str = "onnx/model.onnx";
const VOICE_FILENAME: &str = "voices/af_heart.bin";
const TOKENIZER_FILENAME: &str = "tokenizer.json";

/// Download TTS model files from Hugging Face.
pub async fn download_from_huggingface(
    repo: &HuggingfaceRepo,
) -> anyhow::Result<std::path::PathBuf> {
    let repo_interface = RepositoryInterface::new()?;
    let repo_info = repo_interface.get_repository_metadata(repo).await?;

    let _downloaded_files = repo_info
        .download(&[MODEL_FILENAME, VOICE_FILENAME, TOKENIZER_FILENAME])
        .await?;

    // Return the repository root directory (parent of the tokenizer file's parent)
    let model_path = repo_info
        .get_path(TOKENIZER_FILENAME)
        .context("Failed to get model path")?;
    let repo_dir = model_path.parent().context("No parent directory found")?;
    Ok(repo_dir.to_path_buf())
}

/// Kokoro TTS model wrapper
pub struct Kokoro {
    session: Session,
    tokenizer: Tokenizer,
    input_memory_info: MemoryInfo,
    output_memory_info: MemoryInfo,
    style_data: Vec<Vec<f32>>,
    speed_tensor: sauropod_onnxruntime::Value<'static>,
}

impl Kokoro {
    /// Load the TTS model from a directory path
    pub async fn new(
        ort_env: &sauropod_onnxruntime::Env,
        model_dir: &std::path::Path,
    ) -> anyhow::Result<Self> {
        let model_path = model_dir.join("onnx").join("model.onnx");
        let voice_path = model_dir.join("voices").join("af_heart.bin");
        let tokenizer_path = model_dir.join(TOKENIZER_FILENAME);

        let session = ort_env.create_session(
            &model_path,
            sauropod_onnxruntime::SessionUserOptions {
                device_id: Some(0),
                session_type: sauropod_onnxruntime::SessionType::PreferCUDA,
                allow_cuda_graph: false,
            },
        )?;

        let voice_data = std::fs::read(&voice_path).context("Failed to read voice style file")?;
        let voice_data_f32 = voice_data
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>();

        let tokenizer = tokenizer::Tokenizer::from_file(tokenizer_path).unwrap();
        let input_memory_info = MemoryInfo::cpu_input()?;
        let output_memory_info = MemoryInfo::cpu_output()?;
        let speed_tensor = session.allocator.create_tensor_with_value(&[1f32], &[1])?;

        Ok(Self {
            session,
            tokenizer,
            input_memory_info,
            output_memory_info,
            style_data: voice_data_f32.chunks_exact(256).map(Vec::from).collect(),
            speed_tensor,
        })
    }
}

impl sauropod_inference_thread::InferenceProvider for Kokoro {
    type Input = crate::TtsRequest;
    type Output = Vec<i16>;

    fn process(
        &self,
        input: &[Self::Input],
        output: &mut Vec<anyhow::Result<Self::Output>>,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(input.len() == 1, "TTS model expects exactly one input");

        let text = &input[0];
        let tokens = self
            .tokenizer
            .tokenize(&text.text)
            .context("Tokenizing text")?;
        let token_count = tokens.len();
        if token_count > 510 {
            return Err(anyhow::anyhow!(
                "Input text is too long, maximum length is 510 tokens, got {}",
                token_count
            ));
        }
        // Pad start and end with zeros
        let tokens: Vec<i64> = std::iter::once(0i64)
            .chain(tokens.into_iter().map(|x| x as i64))
            .chain(std::iter::once(0))
            .collect::<Vec<_>>();

        let text_input = self
            .input_memory_info
            .create_tensor_with_data_as_ort_value(&tokens, &[1, tokens.len() as i64])?;

        let style = self.style_data[token_count].as_slice();
        let style = self
            .input_memory_info
            .create_tensor_with_data_as_ort_value(style, &[1, 256])?;
        let io_binding = self.session.create_io_binding()?;
        io_binding.bind_input("input_ids", &text_input)?;
        io_binding.bind_input("style", &style)?;
        io_binding.bind_input("speed", &self.speed_tensor)?;
        io_binding.bind_output_with_memory_info("waveform", &self.output_memory_info)?;

        let io_bindings = self.session.run_with_io_binding(io_binding)?;
        let mut outputs_iter = io_bindings
            .get_bound_output_values(&self.session.allocator)?
            .into_iter();
        let mut waveform_output = outputs_iter.next().unwrap();
        let waveform_data = waveform_output.get_tensor_mutable_data::<f32>()?;

        let audio_data: Vec<i16> = waveform_data
            .iter()
            .map(|&x| (x * 32767.0) as i16)
            .collect();

        output.push(Ok(audio_data));
        Ok(())
    }
}

/// Create a new Kokoro TTS inference thread.
pub async fn make_tts_thread(
    env: &sauropod_onnxruntime::Env,
    model_dir: &std::path::Path,
) -> anyhow::Result<crate::TtsThread> {
    let provider = Kokoro::new(env, model_dir).await?;
    Ok(crate::TtsThread::new("kokoro".to_string(), 1, provider)?)
}
