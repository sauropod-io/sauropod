//! [Orpheus](https://huggingface.co/canopylabs/orpheus-3b-0.1-ft)-based text-to-speech (TTS) model implementation.

use std::collections::VecDeque;

use sauropod_huggingface::HuggingfaceRepo;
use tokio_stream::StreamExt as _;

/// TTS model wrapper
pub struct Orpheus {
    model: sauropod_inference_engine::ModelPointer,
    tokenizer: tokenizers::Tokenizer,
    decoder: sauropod_audio::SnacDecoder,
}

impl Orpheus {
    /// Load the TTS model from a directory path
    pub async fn new(
        ort_env: &sauropod_onnxruntime::Env,
        model_source: &sauropod_inference_engine_api::ModelSource,
    ) -> anyhow::Result<Self> {
        let model_path = sauropod_inference_engine::get_model_path(model_source).await?;
        let model =
            sauropod_inference_engine::load_model("orpheus".to_string(), &model_path).await?;
        let tokenizer = load_tokenizer(HuggingfaceRepo {
            repo: "canopylabs/orpheus-3b-0.1-ft".to_string(),
            revision: None,
            quantization: None,
        })
        .await?;

        let snac_repo_interface = sauropod_huggingface::RepositoryInterface::new()?;
        let snac_repo_info = snac_repo_interface
            .get_repository_metadata(&sauropod_huggingface::HuggingfaceRepo {
                repo: "onnx-community/snac_24khz-ONNX".to_string(),
                revision: None,
                quantization: None,
            })
            .await?;
        let snac_downloaded_files = snac_repo_info
            .download(&["onnx/decoder_model.onnx"])
            .await?;
        let snac_model_path = snac_downloaded_files
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No downloaded decoder files found"))?;

        let decoder = sauropod_audio::SnacDecoder::new(
            ort_env,
            &snac_model_path,
            sauropod_onnxruntime::SessionUserOptions {
                device_id: Some(0),
                session_type: sauropod_onnxruntime::SessionType::PreferCUDA,
                allow_cuda_graph: false,
            },
        )
        .await?;

        Ok(Self {
            model,
            tokenizer,
            decoder,
        })
    }

    async fn generate_from_request(&self, request: &crate::TtsRequest) -> anyhow::Result<Vec<i16>> {
        // The input format is: <custom_token_3><|begin_of_text|>VOICE: PROMPT<|eot_id|><custom_token_4><custom_token_5><custom_token_1>
        let text = format!(
            "{}: {}",
            request.voice.as_deref().unwrap_or("tara"),
            request.text
        );
        let mut tokenized = vec![128259];
        tokenized.extend(
            self.tokenizer
                .encode(text.as_str(), true)
                .map_err(|e| anyhow::anyhow!(e))?
                .get_ids()
                .iter(),
        );
        tokenized.extend([128009, 128260, 128261, 128257]);

        let mut token_stream = self
            .model
            .clone()
            .generate_from_tokens(
                sauropod_inference_engine_api::SamplerProperties {
                    max_predict: 1000,
                    temperature: 0.75,
                    top_p: Some(0.9),
                    top_k: None,
                    min_p: None,
                    repetition_penalty: Some(1.3),
                },
                tokenized,
            )
            .await?;

        let mut audio = Vec::new();
        // We process 7 tokens per batch, 4 batches at a time. The preceeding 3 batches are used for context.
        let mut snac_values = VecDeque::with_capacity(7 * 4);
        while let Some(part) = token_stream.next().await {
            let token = part?;
            if token < 128256 {
                // Skip tokens that are not in the SNAC range
                continue;
            }

            let number_value = token as i64 - 128256; // Adjust token value to match the number in the token name
            let snac_value = number_value - 10 - ((snac_values.len() as i64 % 7) * 4096);
            snac_values.push_back(snac_value);

            if snac_values.len() % 7 == 0 && snac_values.len() >= 28 {
                let input = snac_values.make_contiguous();
                audio.extend(
                    self.decoder
                        .decode(input)?
                        .iter()
                        .map(|&t| (t * 32767.0) as i16), // Scale float to i16 range
                );
                snac_values.drain(0..7);
            }
        }
        Ok(audio)
    }
}

impl sauropod_inference_thread::InferenceProvider for Orpheus {
    type Input = crate::TtsRequest;
    type Output = Vec<i16>;

    fn process(
        &self,
        input: &[Self::Input],
        output: &mut Vec<anyhow::Result<Self::Output>>,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(input.len() == 1, "TTS model expects exactly one input");
        let result = futures_lite::future::block_on(self.generate_from_request(&input[0]));
        output.push(result);
        Ok(())
    }
}

/// Create a new TTS inference thread.
pub async fn make_tts_thread(
    ort_env: &sauropod_onnxruntime::Env,
    model_source: &sauropod_inference_engine_api::ModelSource,
) -> anyhow::Result<crate::TtsThread> {
    let provider = Orpheus::new(ort_env, model_source).await?;
    Ok(crate::TtsThread::new("orpheus".to_string(), 1, provider)?)
}

/// Load a tokenizer from a Hugging Face repository asynchronously.
async fn load_tokenizer(repo: HuggingfaceRepo) -> anyhow::Result<tokenizers::Tokenizer> {
    let interface = sauropod_huggingface::RepositoryInterface::new()?;
    let repo_info = interface.get_repository_metadata(&repo).await?;
    let files = repo_info.download(&["tokenizer.json"]).await?;
    let path = files
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("No tokenizer.json found"))?;
    let tokenizer = tokenizers::Tokenizer::from_file(&path).map_err(|e| anyhow::anyhow!(e))?;
    Ok(tokenizer)
}
