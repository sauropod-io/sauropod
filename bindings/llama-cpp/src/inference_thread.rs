use std::sync::Arc;

use anyhow::Context as _;

type TokenReceiver =
    tokio::sync::mpsc::Receiver<anyhow::Result<sauropod_inference_engine_api::Token>>;
type TokenSender = tokio::sync::mpsc::Sender<anyhow::Result<sauropod_inference_engine_api::Token>>;

pub struct GenerationRequest {
    /// The input tokens to process.
    pub tokens: sauropod_inference_engine_api::TokenSequence,
    /// Sampler properties for the generation.
    pub sampler_properties: sauropod_inference_engine_api::SamplerProperties,
    /// A sender to return the output.
    ///
    /// The output is streamed back to the caller token by token.
    pub sender: TokenSender,
    /// Parent span ID.
    pub parent_span_id: Option<tracing::Id>,
}

/// A model and an inference thread.
pub struct ModelInferenceThread {
    /// The model to use for inference.
    pub model: Arc<crate::Model>,
    /// The queue for inputs to be processed.
    queue: tokio::sync::mpsc::Sender<GenerationRequest>,
    /// The thread handle for the inference worker.
    _thread_handle: std::thread::JoinHandle<()>,
}

impl ModelInferenceThread {
    /// Create a new inference thread from a model file.
    pub async fn from_file(name: String, model_path: &std::path::Path) -> anyhow::Result<Self> {
        let model = crate::Model::from_file(model_path).await?;
        Self::new(name, Arc::new(model))
    }

    /// Create a new inference thread.
    pub fn new(name: String, model: Arc<crate::Model>) -> anyhow::Result<Self> {
        let (input_tx, input_rx) = tokio::sync::mpsc::channel(32);
        let model_clone = model.clone();
        let thread_handle = std::thread::Builder::new().name(name).spawn(move || {
            if let Err(error) = inference_thread(model_clone, input_rx) {
                tracing::error!("Inference thread encountered an error: {:#?}", error);
            }
        })?;

        Ok(Self {
            model,
            queue: input_tx,
            _thread_handle: thread_handle,
        })
    }

    async fn generate_from_tokens_impl(
        &self,
        sampler_properties: sauropod_inference_engine_api::SamplerProperties,
        tokens: sauropod_inference_engine_api::TokenSequence,
    ) -> anyhow::Result<TokenReceiver> {
        let (tx, rx) = tokio::sync::mpsc::channel(5);
        let request = GenerationRequest {
            sampler_properties,
            sender: tx,
            tokens,
            parent_span_id: tracing::Span::current().id(),
        };
        self.queue
            .send(request)
            .await
            .context("Failed to enqueue llama.cpp request")?;
        Ok(rx)
    }

    async fn generate_from_string_impl(
        self: Arc<Self>,
        sampler_properties: sauropod_inference_engine_api::SamplerProperties,
        text: String,
    ) -> anyhow::Result<impl tokio_stream::Stream<Item = anyhow::Result<String>>> {
        let vocab = self.model.get_vocab()?;
        let tokens = vocab.tokenize(&text)?;
        let mut receiver = self
            .generate_from_tokens_impl(sampler_properties, tokens)
            .await?;

        let mut part_buffer = [0u8; 256];
        let stream = async_stream::stream! {
            loop {
                match receiver.recv().await {
                    Some(Ok(token)) => {
                        let n = unsafe {
                            llama_cpp_sys::llama_token_to_piece(
                                vocab.as_ptr(),
                                token as i32, // Convert u32 to i32 for llama_cpp_sys
                                part_buffer.as_mut_ptr() as *mut libc::c_char,
                                part_buffer.len() as i32,
                                0,    // flags
                                true, // add_special
                            )
                        };
                        if n < 0 {
                            yield Err(anyhow::anyhow!(
                                "Failed to convert token to piece"
                            ));
                        }
                        yield Ok(String::from_utf8_lossy(&part_buffer[..n as usize]).to_string());
                    }
                    Some(Err(e)) => {
                        yield Err(e);
                        break;
                    }
                    None => {
                        break;
                    }
                };
            }
        };
        Ok(stream)
    }
}

#[async_trait::async_trait]
impl sauropod_inference_engine_api::LlmModel for ModelInferenceThread {
    async fn generate_from_tokens(
        self: Arc<Self>,
        sampler_properties: sauropod_inference_engine_api::SamplerProperties,
        tokens: sauropod_inference_engine_api::TokenSequence,
    ) -> anyhow::Result<sauropod_inference_engine_api::TokenStream> {
        let receiver = self
            .generate_from_tokens_impl(sampler_properties, tokens)
            .await?;
        Ok(
            Box::pin(tokio_stream::wrappers::ReceiverStream::new(receiver))
                as sauropod_inference_engine_api::TokenStream,
        )
    }

    async fn generate_from_text(
        self: Arc<Self>,
        sampler_properties: sauropod_inference_engine_api::SamplerProperties,
        tokens: String,
    ) -> anyhow::Result<sauropod_inference_engine_api::PartStream> {
        let stream = self
            .generate_from_string_impl(sampler_properties, tokens)
            .await?;
        Ok(Box::pin(stream) as sauropod_inference_engine_api::PartStream)
    }

    fn get_model_chat_template(&self) -> &str {
        self.model.chat_template.as_str()
    }

    fn get_model_type(&self) -> sauropod_output_parser::ModelType {
        self.model.model_type
    }
}

/// The inference loop.
fn inference_thread(
    model: Arc<crate::Model>,
    mut input_rx: tokio::sync::mpsc::Receiver<GenerationRequest>,
) -> anyhow::Result<()> {
    let vocab = model.get_vocab()?;

    loop {
        let Some(mut request) = input_rx.blocking_recv() else {
            // No more inputs, exit the loop.
            break;
        };

        let span = tracing::info_span!(parent: None, "llama.cpp inference");
        span.follows_from(request.parent_span_id);
        let _guard = span.enter();

        let sampler = crate::Sampler::new(&request.sampler_properties)?;
        let mut generated_token_count = 0usize;
        let start_time = std::time::Instant::now();
        let maybe_error = 'decode_loop: {
            let context = match model.llama_context(
                request.tokens.len() as i64,
                request.sampler_properties.max_predict as i64,
            ) {
                Ok(context) => context,
                Err(e) => {
                    break 'decode_loop Err(anyhow::anyhow!(e));
                }
            };

            let mut batch = unsafe {
                llama_cpp_sys::llama_batch_get_one(
                    request.tokens.as_mut_ptr() as *mut i32, // u32 -> i32
                    request.tokens.len() as i32,
                )
            };

            loop {
                let n_ctx_used =
                    unsafe { llama_cpp_sys::llama_memory_seq_pos_max(context.get_memory(), 0) };
                let n_ctx_needed = n_ctx_used + batch.n_tokens;
                if n_ctx_needed > context.context_size() as i32 {
                    break 'decode_loop Err(anyhow::anyhow!(
                        "Context size exceeded: {n_ctx_needed} > {}",
                        context.context_size()
                    ));
                }

                let decode_result = unsafe { llama_cpp_sys::llama_decode(context.0, batch) };
                match decode_result {
                    0 => {}
                    1 => {
                        break 'decode_loop Err(anyhow::anyhow!(
                            "Could not find a KV slot for the batch"
                        ));
                    }
                    2 => {
                        break 'decode_loop Err(anyhow::anyhow!("Decode aborted"));
                    }
                    -1 => {
                        break 'decode_loop Err(anyhow::anyhow!("Invalid input batch"));
                    }
                    _ => {
                        break 'decode_loop Err(anyhow::anyhow!(
                            "An unknown error occured during llama_decode"
                        ));
                    }
                }

                generated_token_count += 1;
                let mut new_token_id = sampler.sample(&context);
                if vocab.is_end_of_generation(new_token_id as u32) {
                    let duration = std::time::Instant::now() - start_time;
                    let tokens_per_second = generated_token_count as f64 / duration.as_secs_f64();
                    span.record("generated_tokens", generated_token_count);
                    tracing::debug!(
                        "Inference completed in {duration:.2?} with {tokens_per_second} tok/s",
                    );

                    break Ok(());
                }

                if let Err(send_error) = request.sender.blocking_send(Ok(new_token_id as u32)) {
                    tracing::error!("Failed to send token back to sender: {:#?}", send_error);
                    break Ok(()); // We break with `Ok` since we already know the sender is erroring
                }

                // The next batch is the previous sampled token
                batch =
                    unsafe { llama_cpp_sys::llama_batch_get_one(&mut new_token_id as *mut _, 1) };
            }
        };
        if let Err(decode_error) = maybe_error {
            tracing::error!("Error during inference: {:#?}", decode_error);
            // Send an error back to the sender
            if let Err(send_error) = request.sender.blocking_send(Err(decode_error)) {
                tracing::error!("Failed to send error back to sender: {:#?}", send_error);
            }
        }
    }

    tracing::info!("LLM inference thread exiting");
    Ok(())
}
