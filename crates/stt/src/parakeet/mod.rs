use std::collections::HashMap;

use anyhow::Context;

#[cfg(test)]
mod tests;

const PREPROCESSOR_FILENAME: &str = "preprocessor.onnx";
const TOKENIZER_FILENAME: &str = "tokenizer.json";
const DECODER_FILENAME: &str = "decoder_joint-parakeet-tdt-0.6b-v2.onnx";
const ENCODER_FILENAME: &str = "encoder-parakeet-tdt-0.6b-v2.onnx";

const OUTPUT_STATES_1: &str = "decoder_output_states_1";
const OUTPUT_STATES_2: &str = "decoder_output_states_2";
const INPUT_STATES_1: &str = "decoder_input_states_1";
const INPUT_STATES_2: &str = "decoder_input_states_2";

/// Download STT model files from Hugging Face.
pub async fn download_from_huggingface(
    model_source: &sauropod_config::ConfigModelSource,
) -> anyhow::Result<std::path::PathBuf> {
    let repo = match &model_source {
        sauropod_config::ConfigModelSource::HuggingFace(repo) => repo,
        sauropod_config::ConfigModelSource::LocalPath(dir) => {
            return Ok(std::path::PathBuf::from(dir));
        }
    };
    let files = sauropod_huggingface::download_onnx_files(
        repo,
        &[
            PREPROCESSOR_FILENAME,
            TOKENIZER_FILENAME,
            DECODER_FILENAME,
            ENCODER_FILENAME,
        ],
    )
    .await?;

    // Return the directory containing the downloaded files
    let first_file = files.first().context("No files downloaded")?;
    let parent_dir = first_file.parent().context("No parent directory found")?;
    Ok(parent_dir.to_path_buf())
}

type Token = i32;

#[derive(serde::Deserialize)]
struct TokenizerData {
    /// Token to ID mapping.
    pub vocab: HashMap<String, Token>,
}

/// Parakeet STT model wrapper
pub struct Parakeet {
    /// The audio preprocessor.
    preprocessor: sauropod_audio::Preprocessor,
    /// ONNX Runtime session for the encoder model.
    encoder_session: sauropod_onnxruntime::Session,
    /// ONNX Runtime session for the decoder model.
    decoder_session: sauropod_onnxruntime::Session,
    /// Memory info for input allocation.
    input_memory_info: sauropod_onnxruntime::MemoryInfo,
    /// Memory info for output allocation.
    output_memory_info: sauropod_onnxruntime::MemoryInfo,
    /// ID to part mapping for the tokenizer.
    ///
    /// Represented as a vector of strings, where the index corresponds to the token ID.
    id_to_part: Vec<String>,
    /// Blank token
    blank_token: Token,
}

const MAX_TOKENS_PER_STEP: usize = 10;

impl Parakeet {
    /// Load the speech to text model from a directory path.
    pub async fn new(
        ort_env: &sauropod_onnxruntime::Env,
        model_dir: &std::path::Path,
    ) -> anyhow::Result<Self> {
        let preprocessor_path = model_dir.join(PREPROCESSOR_FILENAME);
        let tokenizer_path = model_dir.join(TOKENIZER_FILENAME);
        let decoder_model_path = model_dir.join(DECODER_FILENAME);
        let encoder_model_path = model_dir.join(ENCODER_FILENAME);

        let tokenizer_data: TokenizerData =
            serde_json::from_str(&std::fs::read_to_string(&tokenizer_path)?)?;
        let max_token_id = tokenizer_data.vocab.values().max().unwrap_or(&0);
        let mut id_to_part = vec!["".to_string(); *max_token_id as usize + 1];
        for (part, id) in &tokenizer_data.vocab {
            id_to_part[*id as usize] = part.clone();
        }

        // <https://github.com/NVIDIA/NeMo/blob/31fa168e000db0ce8030e069810ee24174864184/nemo/collections/asr/parts/submodules/rnnt_decoding.py#L1374-L1375>
        let blank_token = id_to_part.len() as Token;

        let preprocessor = sauropod_audio::Preprocessor::new(
            ort_env,
            &preprocessor_path,
            sauropod_onnxruntime::SessionUserOptions {
                device_id: Some(0),
                session_type: sauropod_onnxruntime::SessionType::CPU,
                allow_cuda_graph: false,
            },
        )
        .await?;

        let encoder = ort_env.create_session(
            &encoder_model_path,
            sauropod_onnxruntime::SessionUserOptions {
                device_id: Some(0),
                session_type: sauropod_onnxruntime::SessionType::PreferCUDA,
                allow_cuda_graph: false,
            },
        )?;
        let decoder = ort_env.create_session(
            &decoder_model_path,
            sauropod_onnxruntime::SessionUserOptions {
                device_id: Some(0),
                session_type: sauropod_onnxruntime::SessionType::PreferTensorRT,
                allow_cuda_graph: false,
            },
        )?;

        let input_memory_info = sauropod_onnxruntime::MemoryInfo::cpu_input()?;
        let output_memory_info = sauropod_onnxruntime::MemoryInfo::cpu_output()?;
        Ok(Self {
            preprocessor,
            encoder_session: encoder,
            decoder_session: decoder,
            input_memory_info,
            output_memory_info,
            id_to_part,
            blank_token,
        })
    }

    /// Run the encoder model
    #[tracing::instrument(skip(self, input), level = "info")]
    fn run_encoder(
        &self,
        input: sauropod_audio::Features,
    ) -> anyhow::Result<(
        sauropod_onnxruntime::Value<'static>,
        sauropod_onnxruntime::Value<'static>,
    )> {
        let io_binding = self
            .encoder_session
            .create_io_binding()
            .context("Creating IO binding for STT encoder")?;
        io_binding.bind_input("audio_signal", &input.features)?;
        io_binding.bind_input("length", &input.features_length)?;
        io_binding.bind_output_with_memory_info("outputs", &self.output_memory_info)?;
        io_binding.bind_output_with_memory_info("encoded_lengths", &self.output_memory_info)?;

        let io_binding = self
            .encoder_session
            .run_with_io_binding(io_binding)
            .context("Running STT encoder")?;

        let outputs = io_binding.get_bound_output_values(&self.encoder_session.allocator)?;
        let mut outputs_iter = outputs.into_iter();
        let outputs = outputs_iter.next().unwrap();
        let encoded_lengths = outputs_iter.next().unwrap();

        Ok((outputs, encoded_lengths))
    }

    /// Run the decoder model iteratively
    #[tracing::instrument(skip(self, encoder_output, encoded_lengths), level = "info")]
    fn run_decoder(
        &self,
        encoder_output: sauropod_onnxruntime::Value<'_>,
        mut encoded_lengths: sauropod_onnxruntime::Value<'_>,
    ) -> anyhow::Result<String> {
        let mut io_binding = self
            .decoder_session
            .create_io_binding()
            .context("Creating IO binding for STT decoder")?;

        let mut tokens = vec![self.blank_token];
        let mut result = String::with_capacity(128);
        let mut max_tokens = MAX_TOKENS_PER_STEP;

        let output_states_1 = self
            .decoder_session
            .allocator
            .create_tensor(0f32, &[2, 1, 640])?;
        let output_states_2 = self
            .decoder_session
            .allocator
            .create_tensor(0f32, &[2, 1, 640])?;
        let target_length = &[1_i32];
        let target_length = self
            .input_memory_info
            .create_tensor_with_data_as_ort_value(target_length, &[1])?;
        let encoded_lengths = encoded_lengths.get_tensor_mutable_data::<i64>()?;

        io_binding.bind_input("encoder_outputs", &encoder_output)?;
        io_binding.bind_input("decoder_target_length", &target_length)?;
        io_binding.bind_input(INPUT_STATES_1, &output_states_1)?;
        io_binding.bind_input(INPUT_STATES_2, &output_states_2)?;

        io_binding.bind_output(OUTPUT_STATES_1, &output_states_1)?;
        io_binding.bind_output(OUTPUT_STATES_2, &output_states_2)?;
        let mut decoder_tokens = self
            .decoder_session
            .allocator
            .create_uninit_tensor::<i64>(&[1, 1, 1])?;
        io_binding.bind_output("decoder_tokens", &decoder_tokens)?;

        let mut next_index: i64 = 0;
        while next_index < encoded_lengths[0] {
            let index = [next_index];
            let index = self
                .input_memory_info
                .create_tensor_with_data_as_ort_value(index.as_slice(), &[1])?;
            io_binding.bind_input("index", &index)?;

            let targets: [i32; 1] = [*tokens.last().unwrap_or(&self.blank_token)];
            let targets = self
                .input_memory_info
                .create_tensor_with_data_as_ort_value(&targets, &[1, 1])?;
            io_binding.bind_input("decoder_targets", &targets)?;

            // Prepare tokens input
            io_binding = self
                .decoder_session
                .run_with_io_binding(io_binding)
                .context("Running STT decoder")?;

            let tokens_output = decoder_tokens.get_tensor_mutable_data::<Token>()?;
            if tokens_output.len() > 1 {
                return Err(anyhow::anyhow!(
                    "Unexpected number of tokens: {}",
                    tokens_output.len()
                ));
            }

            let next_token = *tokens_output.last().unwrap();
            if next_token != self.blank_token {
                if next_token as usize >= self.id_to_part.len() {
                    tracing::error!("Token ID {next_token} out of bounds for id_to_part mapping");
                    return Err(anyhow::anyhow!(
                        "Token ID {next_token} out of bounds for id_to_part mapping",
                    ));
                }

                tokens.push(next_token);
                result.push_str(&self.id_to_part[next_token as usize]);
                max_tokens -= 1;
            }

            if max_tokens == 0 || next_token == self.blank_token {
                next_index += 1;
                max_tokens = MAX_TOKENS_PER_STEP;
            }
        }

        Ok(result.trim_start().to_string())
    }
}

impl sauropod_inference_thread::InferenceProvider for Parakeet {
    type Input = Vec<f32>;
    type Output = String;

    fn process(
        &self,
        input: &[Self::Input],
        output: &mut Vec<anyhow::Result<Self::Output>>,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(input.len() == 1, "STT model expects a single audio input");

        // Calculate mel spectrogram
        let processed_inputs = self.preprocessor.preprocess(&input[0])?;

        // Run encoder
        let (outputs, encoded_lengths) = self.run_encoder(processed_inputs)?;

        // Run decoder iteratively to generate tokens
        let text = self.run_decoder(outputs, encoded_lengths)?;
        output.push(Ok(text));

        Ok(())
    }
}
