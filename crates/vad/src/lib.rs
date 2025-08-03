//! Activity Detection (VAD) module

use anyhow::Context;

use sauropod_inference_thread::BatchInferenceThread;

#[cfg(test)]
mod tests;

const BATCH_SIZE: usize = 4;
const PREPROCESSOR_FILENAME: &str = "preprocessor.onnx";
const MODEL_FILENAME: &str = "frame_vad_multilingual_marblenet_v2.0.onnx";

/// Download VAD model files from Hugging Face.
pub async fn download_from_huggingface(
    model_source: &sauropod_config::ConfigModelSource,
) -> anyhow::Result<std::path::PathBuf> {
    let repo = match &model_source {
        sauropod_config::ConfigModelSource::HuggingFace(repo) => repo,
        sauropod_config::ConfigModelSource::LocalPath(dir) => {
            return Ok(std::path::PathBuf::from(dir));
        }
    };
    let files =
        sauropod_huggingface::download_onnx_files(repo, &[PREPROCESSOR_FILENAME, MODEL_FILENAME])
            .await?;

    // Return the directory containing the downloaded files
    let first_file = files.first().context("No files downloaded")?;
    let parent_dir = first_file.parent().context("No parent directory found")?;
    Ok(parent_dir.to_path_buf())
}

/// VAD model wrapper
pub struct Vad {
    /// The audio preprocessor.
    preprocessor: sauropod_audio::Preprocessor,
    /// ONNX Runtime session for the VAD model.
    ort_session: sauropod_onnxruntime::Session,
}

/// VAD classifier output.
#[derive(Debug, Clone)]
pub struct VadClassification {
    /// The range of samples that were classified.
    pub range: std::ops::Range<usize>,
    /// The classification strength.
    pub score: f32,
}

impl Vad {
    /// The size of each frame in samples.
    pub const FRAME_SIZE: usize = 16000 / 50; // 20ms at 16kHz for the pretrained Nemo model
    /// The number of frames that should be present in each process call.
    pub const CONTEXT_FRAMES: usize = 50;
    /// The number of samples in each context.
    pub const CONTEXT_SAMPLES: usize = Self::FRAME_SIZE * Self::CONTEXT_FRAMES;

    /// Load the VAD model from a directory path
    pub async fn new(
        ort_env: &sauropod_onnxruntime::Env,
        model_dir: &std::path::Path,
    ) -> anyhow::Result<Self> {
        let preprocessor_path = model_dir.join(PREPROCESSOR_FILENAME);
        let model_path = model_dir.join(MODEL_FILENAME);

        Ok(Self {
            preprocessor: sauropod_audio::Preprocessor::new(
                ort_env,
                &preprocessor_path,
                sauropod_onnxruntime::SessionUserOptions {
                    device_id: Some(0),
                    session_type: sauropod_onnxruntime::SessionType::CPU,
                    allow_cuda_graph: false,
                },
            )
            .await?,
            ort_session: ort_env.create_session(
                &model_path,
                sauropod_onnxruntime::SessionUserOptions {
                    device_id: Some(0),
                    session_type: sauropod_onnxruntime::SessionType::CPU,
                    allow_cuda_graph: false,
                },
            )?,
        })
    }
}

impl sauropod_inference_thread::InferenceProvider for Vad {
    type Input = Vec<f32>;
    type Output = Vec<VadClassification>;

    fn process(
        &self,
        input: &[Self::Input],
        output: &mut Vec<anyhow::Result<Self::Output>>,
    ) -> anyhow::Result<()> {
        let mut input_audio = vec![0.0f32; BATCH_SIZE * (Self::CONTEXT_SAMPLES - 1)];
        anyhow::ensure!(
            input.len() <= BATCH_SIZE,
            "Input batch size exceeds the maximum of {BATCH_SIZE}"
        );

        for (i, audio) in input.iter().enumerate() {
            if audio.len() != Self::CONTEXT_SAMPLES {
                return Err(anyhow::anyhow!(
                    "Audio length must be {}",
                    Self::CONTEXT_SAMPLES
                ));
            }
            input_audio[i * (Self::CONTEXT_SAMPLES - 1)..(i + 1) * (Self::CONTEXT_SAMPLES - 1)]
                .copy_from_slice(&audio[..Self::CONTEXT_SAMPLES - 1]);
        }

        let preprocessed = self
            .preprocessor
            .preprocess_batch(&input_audio, Self::CONTEXT_SAMPLES - 1)
            .with_context(|| "Error preprocessing audio for VAD".to_string())?;
        let io_session = self
            .ort_session
            .create_io_binding()
            .with_context(|| "Error creating IO binding for VAD model".to_string())?;
        io_session.bind_input("processed_signal", &preprocessed.features)?;
        io_session.bind_input("processed_signal_length", &preprocessed.features_length)?;

        let output_length = self
            .preprocessor
            .get_output_length(Self::CONTEXT_SAMPLES - 1)
            / 2;
        let mut softmax_output = self.ort_session.allocator.create_uninit_tensor::<f32>(&[
            BATCH_SIZE as i64,
            output_length,
            1,
        ])?;
        io_session.bind_output("softmax_output", &softmax_output)?;

        let _ = self
            .ort_session
            .run_with_io_binding(io_session)
            .with_context(|| "Error running VAD model".to_string())?;

        let softmax_output_data = softmax_output.get_tensor_mutable_data::<f32>()?;
        for batch_idx in 0..input.len() {
            let mut res = Vec::new();
            for (index, score) in softmax_output_data
                [batch_idx * (output_length as usize)..(batch_idx + 1) * (output_length as usize)]
                .iter()
                .enumerate()
            {
                let frame_begin = index * Self::FRAME_SIZE;
                res.push(VadClassification {
                    range: frame_begin..frame_begin + Self::FRAME_SIZE,
                    score: *score,
                });
            }
            output.push(Ok(res));
        }

        Ok(())
    }
}

/// VAD inference thread.
pub type VadThread = BatchInferenceThread<Vec<f32>, Vec<VadClassification>>;

/// Create a new VAD inference thread.
pub async fn make_vad_thread(
    env: &sauropod_onnxruntime::Env,
    model_dir: &std::path::Path,
) -> anyhow::Result<VadThread> {
    let provider = Vad::new(env, model_dir).await?;
    Ok(BatchInferenceThread::new(
        "vad".to_string(),
        BATCH_SIZE,
        provider,
    )?)
}
