use std::sync::Arc;

use crate::mtmd::MtmdBitmap;
use anyhow::Context as _;

type TokenReceiver =
    tokio::sync::mpsc::Receiver<anyhow::Result<sauropod_inference_engine_api::Token>>;
type TokenSender = tokio::sync::mpsc::Sender<anyhow::Result<sauropod_inference_engine_api::Token>>;
type InputTokenCountOneshot = tokio::sync::oneshot::Sender<i64>;

/// The minimum batch size for llama.cpp.
const MIN_BATCH_SIZE: i32 = llama_cpp_sys::ggml_kq_mask_pad as i32;

#[derive(Debug)]
pub enum GenerationRequestInput {
    Tokens(sauropod_inference_engine_api::TokenSequence),
    Text {
        content: String,
        multimodal_data: Vec<sauropod_prompt_templates::MultimodalData>,
    },
}

pub struct GenerationRequest {
    /// The input tokens to process.
    pub input: GenerationRequestInput,
    /// Sampler properties for the generation.
    pub sampler_properties: sauropod_inference_engine_api::SamplerProperties,
    /// A sender to return the output.
    ///
    /// The output is streamed back to the caller token by token.
    pub token_sender: TokenSender,
    /// A sender to return the count of input tokens.
    pub input_token_count_oneshot: InputTokenCountOneshot,
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
    pub async fn from_file(
        name: String,
        model_path: &std::path::Path,
        projector_model_path: Option<&std::path::Path>,
    ) -> anyhow::Result<Self> {
        let model = crate::Model::from_file(model_path, projector_model_path).await?;
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

    async fn generate_impl(
        &self,
        sampler_properties: sauropod_inference_engine_api::SamplerProperties,
        input: GenerationRequestInput,
    ) -> anyhow::Result<(i64, TokenReceiver)> {
        let (tx, rx) = tokio::sync::mpsc::channel(5);
        let (input_token_count_tx, input_token_count_rx) = tokio::sync::oneshot::channel();
        let request = GenerationRequest {
            sampler_properties,
            token_sender: tx,
            input_token_count_oneshot: input_token_count_tx,
            input,
            parent_span_id: tracing::Span::current().id(),
        };
        self.queue
            .send(request)
            .await
            .context("Failed to enqueue llama.cpp request")?;
        let input_tokens = input_token_count_rx.await?;
        Ok((input_tokens, rx))
    }

    async fn generate_from_string_impl(
        self: Arc<Self>,
        sampler_properties: sauropod_inference_engine_api::SamplerProperties,
        text: String,
        multimodal_data: Vec<sauropod_prompt_templates::MultimodalData>,
    ) -> anyhow::Result<(
        i64,
        impl tokio_stream::Stream<Item = anyhow::Result<String>>,
    )> {
        let vocab = self.model.get_vocab()?;
        let (input_token_count, mut receiver) = self
            .generate_impl(
                sampler_properties,
                GenerationRequestInput::Text {
                    content: text,
                    multimodal_data,
                },
            )
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
        Ok((input_token_count, stream))
    }
}

#[async_trait::async_trait]
impl sauropod_inference_engine_api::LlmModel for ModelInferenceThread {
    async fn generate_from_tokens(
        self: Arc<Self>,
        sampler_properties: sauropod_inference_engine_api::SamplerProperties,
        tokens: sauropod_inference_engine_api::TokenSequence,
    ) -> anyhow::Result<sauropod_inference_engine_api::TokenStream> {
        let (_, receiver) = self
            .generate_impl(sampler_properties, GenerationRequestInput::Tokens(tokens))
            .await?;
        Ok(
            Box::pin(tokio_stream::wrappers::ReceiverStream::new(receiver))
                as sauropod_inference_engine_api::TokenStream,
        )
    }

    async fn generate_from_text(
        self: Arc<Self>,
        sampler_properties: sauropod_inference_engine_api::SamplerProperties,
        text: String,
        multimodal_data: Vec<sauropod_prompt_templates::MultimodalData>,
    ) -> anyhow::Result<sauropod_inference_engine_api::GenerateFromTextResponse> {
        let (input_token_count, stream) = self
            .generate_from_string_impl(sampler_properties, text, multimodal_data)
            .await?;
        Ok(sauropod_inference_engine_api::GenerateFromTextResponse {
            stream: Box::pin(stream) as sauropod_inference_engine_api::PartStream,
            input_token_count,
        })
    }

    fn get_model_chat_template(&self) -> &str {
        self.model.chat_template.as_str()
    }

    fn get_model_type(&self) -> sauropod_output_parser::ModelType {
        self.model.model_type
    }

    fn supports_vision(&self) -> bool {
        self.model.supports_vision()
    }

    fn supports_audio(&self) -> bool {
        self.model.supports_audio()
    }
}

/// Decode a batch of tokens in the llama.cpp context.
fn decode_batch(context: &crate::Context, batch: llama_cpp_sys::llama_batch) -> anyhow::Result<()> {
    match unsafe { llama_cpp_sys::llama_decode(context.0, batch) } {
        0 => Ok(()),
        1 => Err(anyhow::anyhow!("Could not find a KV slot for the batch")),
        2 => Err(anyhow::anyhow!("Decode aborted")),
        -1 => Err(anyhow::anyhow!("Invalid input batch")),
        _ => Err(anyhow::anyhow!(
            "An unknown error occurred during llama_decode"
        )),
    }
}

/// Run the decode loop for a single input.
fn run_for_tokens(
    model: &Arc<crate::Model>,
    mut tokens: sauropod_inference_engine_api::TokenSequence,
    sampler_properties: &sauropod_inference_engine_api::SamplerProperties,
    token_count_sender: InputTokenCountOneshot,
    sender: &TokenSender,
) -> anyhow::Result<()> {
    if let Err(e) = token_count_sender.send(tokens.len() as i64) {
        tracing::error!("Failed to send token count: {:#?}", e);
    }

    let vocab = model.get_vocab()?;
    let sampler = crate::Sampler::new(sampler_properties)?;
    let mut generated_token_count = 0usize;
    let start_time = std::time::Instant::now();

    let context =
        match model.llama_context(tokens.len() as i64, sampler_properties.max_predict as i64) {
            Ok(context) => context,
            Err(e) => {
                return Err(anyhow::anyhow!(e));
            }
        };

    let mut batch = unsafe {
        llama_cpp_sys::llama_batch_get_one(
            tokens.as_mut_ptr() as *mut i32, // u32 -> i32
            tokens.len() as i32,
        )
    };

    loop {
        let n_ctx_used =
            unsafe { llama_cpp_sys::llama_memory_seq_pos_max(context.get_memory(), 0) };
        let n_ctx_needed = n_ctx_used + batch.n_tokens;
        if n_ctx_needed > context.context_size() as i32 {
            return Err(anyhow::anyhow!(
                "Context size exceeded: {n_ctx_needed} > {}",
                context.context_size()
            ));
        }

        decode_batch(&context, batch)?;

        generated_token_count += 1;
        let mut new_token_id = sampler.sample(&context);
        if vocab.is_end_of_generation(new_token_id as u32) {
            let duration = std::time::Instant::now() - start_time;
            let tokens_per_second = generated_token_count as f64 / duration.as_secs_f64();
            tracing::debug!("Inference completed in {duration:.2?} with {tokens_per_second} tok/s",);

            return Ok(());
        }

        if let Err(send_error) = sender.blocking_send(Ok(new_token_id as u32)) {
            tracing::error!("Failed to send token back to sender: {:#?}", send_error);
            return Ok(()); // We break with `Ok` since we already know the sender is erroring
        }

        // The next batch is the previous sampled token
        batch = unsafe { llama_cpp_sys::llama_batch_get_one(&mut new_token_id as *mut _, 1) };
    }
}

fn run_for_multimodal(
    model: &Arc<crate::Model>,
    content: String,
    multimodal_data: Vec<sauropod_prompt_templates::MultimodalData>,
    sampler_properties: &sauropod_inference_engine_api::SamplerProperties,
    token_count_sender: InputTokenCountOneshot,
    sender: &TokenSender,
) -> anyhow::Result<()> {
    let Some(mtmd_context) = model.mtmd_context.as_ref() else {
        return Err(anyhow::anyhow!(
            "This model does not support multimodal inputs"
        ));
    };

    let vocab = model.get_vocab()?;
    let sampler = crate::Sampler::new(sampler_properties)?;
    let mut generated_token_count = 0usize;
    let start_time = std::time::Instant::now();

    let mut bitmaps = multimodal_data
        .iter()
        .map(|data| match data {
            sauropod_prompt_templates::MultimodalData::Image(image) => {
                MtmdBitmap::new_rgb(image.width(), image.height(), image.as_raw())
                    .map_err(|e| anyhow::anyhow!("Failed to create image bitmap: {:?}", e))
            }
            sauropod_prompt_templates::MultimodalData::Audio(data) => MtmdBitmap::new_audio(data)
                .map_err(|e| anyhow::anyhow!("Failed to create audio bitmap: {:?}", e)),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let chunks = mtmd_context
        .tokenize(&content, bitmaps.as_mut_slice())
        .map_err(|e| anyhow::anyhow!("Failed to tokenize multimodal input: {:?}", e))?;

    let n_tokens: usize = (0..chunks.len())
        .map(|i| unsafe { llama_cpp_sys::mtmd_input_chunk_get_n_tokens(chunks.get(i)) })
        .sum();
    if let Err(e) = token_count_sender.send(n_tokens as i64) {
        tracing::error!("Failed to send token count: {:#?}", e);
    }

    let context = model.llama_context(n_tokens as i64, sampler_properties.max_predict as i64)?;

    let mut number_of_processed_tokens: i32 = 0;
    let seq_id = 0;

    for i in 0..chunks.len() {
        let chunk_ptr = chunks.get(i);
        match unsafe { llama_cpp_sys::mtmd_input_chunk_get_type(chunk_ptr) } {
            llama_cpp_sys::mtmd_input_chunk_type::MTMD_INPUT_CHUNK_TYPE_TEXT => {
                let mut batch = crate::OwnedBatch(unsafe {
                    llama_cpp_sys::llama_batch_init(MIN_BATCH_SIZE, 0, 1)
                });
                let mut number_of_tokens = 0usize;
                let tokens_ptr = unsafe {
                    llama_cpp_sys::mtmd_input_chunk_get_tokens_text(
                        chunk_ptr,
                        &mut number_of_tokens,
                    )
                };
                let tokens_slice =
                    unsafe { std::slice::from_raw_parts(tokens_ptr, number_of_tokens) };

                let batch_token =
                    unsafe { std::slice::from_raw_parts_mut(batch.0.token, number_of_tokens) };
                let batch_pos =
                    unsafe { std::slice::from_raw_parts_mut(batch.0.pos, number_of_tokens) };
                let batch_n_seq_id =
                    unsafe { std::slice::from_raw_parts_mut(batch.0.n_seq_id, number_of_tokens) };
                let batch_seq_id =
                    unsafe { std::slice::from_raw_parts_mut(batch.0.seq_id, number_of_tokens) };
                let batch_logits =
                    unsafe { std::slice::from_raw_parts_mut(batch.0.logits, number_of_tokens) };

                for token_chunk in tokens_slice.chunks(MIN_BATCH_SIZE as usize) {
                    batch.0.n_tokens = token_chunk.len() as i32;

                    for (batch_token_index, &token) in token_chunk.iter().enumerate() {
                        batch_token[batch_token_index] = token;
                        batch_pos[batch_token_index] =
                            number_of_processed_tokens + batch_token_index as i32;
                        batch_n_seq_id[batch_token_index] = 1;
                        unsafe { *batch_seq_id[batch_token_index] = 0 };
                        batch_logits[batch_token_index] =
                            if batch_token_index == token_chunk.len() - 1 {
                                1
                            } else {
                                0
                            };
                    }

                    number_of_processed_tokens += token_chunk.len() as i32;

                    decode_batch(&context, batch.0)?;
                }
            }
            llama_cpp_sys::mtmd_input_chunk_type::MTMD_INPUT_CHUNK_TYPE_IMAGE
            | llama_cpp_sys::mtmd_input_chunk_type::MTMD_INPUT_CHUNK_TYPE_AUDIO => {
                let encode_result =
                    unsafe { llama_cpp_sys::mtmd_encode_chunk(mtmd_context.0, chunk_ptr) };
                if encode_result != 0 {
                    return Err(anyhow::anyhow!(
                        "Failed to encode multimodal chunk: {encode_result}",
                    ));
                }
                let embedding_ptr = unsafe { llama_cpp_sys::mtmd_get_output_embd(mtmd_context.0) };
                if embedding_ptr.is_null() {
                    return Err(anyhow::anyhow!("Failed to get encoded embeddings"));
                }

                // Decode the embeddings using helper function equivalent
                let number_of_tokens_in_chunk =
                    unsafe { llama_cpp_sys::mtmd_input_chunk_get_n_tokens(chunk_ptr) };
                let chunk_n_pos = unsafe { llama_cpp_sys::mtmd_input_chunk_get_n_pos(chunk_ptr) };

                let number_of_embeddings = unsafe {
                    llama_cpp_sys::llama_model_n_embd(llama_cpp_sys::llama_get_model(context.0))
                };

                // Process embeddings in batches
                let mut embedding_index = 0usize;
                while embedding_index < number_of_tokens_in_chunk as usize {
                    let tokens_to_process = std::cmp::min(
                        MIN_BATCH_SIZE as usize,
                        number_of_tokens_in_chunk as usize - embedding_index,
                    ) as i32;

                    let embd_offset = unsafe {
                        embedding_ptr.add(embedding_index * number_of_embeddings as usize)
                    };

                    let batch = llama_cpp_sys::llama_batch {
                        n_tokens: tokens_to_process,
                        token: std::ptr::null_mut(),
                        embd: embd_offset,
                        pos: std::ptr::null_mut(),
                        n_seq_id: std::ptr::null_mut(),
                        seq_id: std::ptr::null_mut(),
                        logits: std::ptr::null_mut(),
                    };

                    decode_batch(&context, batch)?;

                    embedding_index += tokens_to_process as usize;
                }
                number_of_processed_tokens += chunk_n_pos;
            }
            #[allow(unreachable_patterns)]
            x => {
                return Err(anyhow::anyhow!("Unsupported chunk type: {}", x as i32));
            }
        }
    }

    // Start generation loop
    loop {
        let n_ctx_used =
            unsafe { llama_cpp_sys::llama_memory_seq_pos_max(context.get_memory(), seq_id) };
        let n_ctx_needed = n_ctx_used + 1;
        if n_ctx_needed > context.context_size() as i32 {
            return Err(anyhow::anyhow!(
                "Context size exceeded: {n_ctx_needed} > {}",
                context.context_size()
            ));
        }

        generated_token_count += 1;
        let mut new_token_id = sampler.sample(&context);
        if vocab.is_end_of_generation(new_token_id as u32) {
            let duration = std::time::Instant::now() - start_time;
            let tokens_per_second = generated_token_count as f64 / duration.as_secs_f64();
            tracing::debug!("Inference completed in {duration:.2?} with {tokens_per_second} tok/s");
            return Ok(());
        }

        if let Err(send_error) = sender.blocking_send(Ok(new_token_id as u32)) {
            tracing::error!("Failed to send token back to sender: {:#?}", send_error);
            return Ok(());
        }

        // Continue with next token
        let batch = unsafe { llama_cpp_sys::llama_batch_get_one(&mut new_token_id as *mut _, 1) };
        decode_batch(&context, batch)?;
    }
}

/// The inference loop.
fn inference_thread(
    model: Arc<crate::Model>,
    mut input_rx: tokio::sync::mpsc::Receiver<GenerationRequest>,
) -> anyhow::Result<()> {
    loop {
        let Some(request) = input_rx.blocking_recv() else {
            // No more inputs, exit the loop.
            break;
        };

        let span = tracing::info_span!(parent: None, "llama.cpp inference");
        span.follows_from(request.parent_span_id);
        let _guard = span.enter();
        let maybe_error = match request.input {
            GenerationRequestInput::Tokens(tokens) => run_for_tokens(
                &model,
                tokens,
                &request.sampler_properties,
                request.input_token_count_oneshot,
                &request.token_sender,
            ),
            GenerationRequestInput::Text {
                content,
                multimodal_data,
            } if multimodal_data.is_empty() => match model.tokenize(&content) {
                Ok(tokens) => run_for_tokens(
                    &model,
                    tokens,
                    &request.sampler_properties,
                    request.input_token_count_oneshot,
                    &request.token_sender,
                ),
                Err(e) => Err(e.into()),
            },
            GenerationRequestInput::Text {
                content,
                multimodal_data,
            } => run_for_multimodal(
                &model,
                content,
                multimodal_data,
                &request.sampler_properties,
                request.input_token_count_oneshot,
                &request.token_sender,
            ),
        };

        if let Err(decode_error) = maybe_error {
            tracing::error!("Error during inference: {:#?}", decode_error);
            // Send an error back to the sender
            if let Err(send_error) = request.token_sender.blocking_send(Err(decode_error)) {
                tracing::error!("Failed to send error back to sender: {:#?}", send_error);
            }
        }
    }

    tracing::info!("LLM inference thread exiting");
    Ok(())
}
