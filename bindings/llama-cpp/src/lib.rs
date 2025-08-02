//! Sauropod's bindings around [llama.cpp](https://github.com/ggml-org/llama.cpp).

mod inference_thread;
pub use inference_thread::ModelInferenceThread;

/// Error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to load model from GGUF file {0}")]
    FailedtoLoadModel(String),
    #[error("Failed to convert path to C string")]
    PathToCStringFailed(#[from] std::ffi::NulError),
    #[error("An error occurred during tokenization")]
    TokenizationError,
    #[error("An error occurred while applying the chat template: {0}")]
    ChatTemplateError(minijinja::Error),
    #[error("The buffer was too small for the chat template - tried to use {0} bytes")]
    ChatTemplateBufferTooSmallError(usize),
    #[error("Failed to create llama.cpp context")]
    FailedToCreateContext,
    #[error("Failed to create llama.cpp sampler")]
    FailedToCreateSampler,
    #[error("The model doesn't have a chat template")]
    NoChatTemplate,
    #[error("The model has an invalid chat template: {0}")]
    InvalidChatTemplate(#[from] minijinja::Error),
    #[error("GGUF metadata parsing error: {0}")]
    GgufMetadataParsingError(#[from] sauropod_gguf::GgufError),
}

/// The logging callback for `llama.cpp`.
extern "C" fn log_callback(
    level: llama_cpp_sys::ggml_log_level,
    text: *const ::std::os::raw::c_char,
    _user_data: *mut ::std::os::raw::c_void,
) {
    let c_str = unsafe { std::ffi::CStr::from_ptr(text) };
    let raw_message_content = c_str.to_string_lossy();
    let message_content = raw_message_content.trim_end();

    match level {
        llama_cpp_sys::ggml_log_level::GGML_LOG_LEVEL_DEBUG => {
            tracing::event!(
                target: "llama.cpp",
                tracing::Level::DEBUG,
                "{}",
                message_content
            );
        }
        llama_cpp_sys::ggml_log_level::GGML_LOG_LEVEL_INFO => {
            tracing::event!(
                target: "llama.cpp",
                tracing::Level::INFO,
                "{}",
                message_content
            );
        }
        llama_cpp_sys::ggml_log_level::GGML_LOG_LEVEL_WARN => {
            tracing::event!(
                target: "llama.cpp",
                tracing::Level::WARN,
                "{}",
                message_content
            );
        }
        llama_cpp_sys::ggml_log_level::GGML_LOG_LEVEL_ERROR => {
            tracing::event!(
                target: "llama.cpp",
                tracing::Level::ERROR,
                "{}",
                message_content
            );
        }
        llama_cpp_sys::ggml_log_level::GGML_LOG_LEVEL_CONT => {
            // TODO handle continuation messages
        }
        llama_cpp_sys::ggml_log_level::GGML_LOG_LEVEL_NONE => {}
    }
}

extern "C" fn log_progress(progress_zero_to_one: f32, user_data: *mut libc::c_void) -> bool {
    let progress_bar = unsafe { &*(user_data as *mut indicatif::ProgressBar) };
    progress_bar.set_position((progress_zero_to_one * (u8::MAX as f32)) as u64);
    true
}

/// Initialize the `llama.cpp` library.
pub fn init() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        unsafe {
            // Set the log callback
            llama_cpp_sys::ggml_log_set(Some(log_callback), std::ptr::null_mut());
            llama_cpp_sys::llama_log_set(Some(log_callback), std::ptr::null_mut());
        }

        // TODO call llama_attach_threadpool
    });
}

#[repr(transparent)]
pub struct Vocab(pub *const llama_cpp_sys::llama_vocab);
unsafe impl Send for Vocab {}

impl Vocab {
    pub fn tokenize(
        &self,
        prompt: &str,
    ) -> Result<Vec<sauropod_inference_engine_api::Token>, Error> {
        let c_str = std::ffi::CString::new(prompt).unwrap();
        let token_count = -unsafe {
            llama_cpp_sys::llama_tokenize(
                self.0,
                c_str.as_ptr(),
                c_str.as_bytes().len() as i32,
                std::ptr::null_mut(),
                0,
                true, // add_special
                true, // parse_special
            )
        };
        let mut tokens =
            vec![sauropod_inference_engine_api::Token::default(); token_count as usize];

        let tokenization_result = unsafe {
            llama_cpp_sys::llama_tokenize(
                self.0,
                c_str.as_ptr(),
                c_str.as_bytes().len() as i32,
                tokens.as_mut_ptr() as *mut i32,
                token_count,
                true, // add_special
                true, // parse_special
            )
        };
        if tokenization_result < 0 {
            return Err(Error::TokenizationError);
        }

        Ok(tokens)
    }

    pub fn is_end_of_generation(&self, token_id: sauropod_inference_engine_api::Token) -> bool {
        unsafe { llama_cpp_sys::llama_vocab_is_eog(self.0, token_id as i32) }
    }

    pub fn as_ptr(&self) -> *const llama_cpp_sys::llama_vocab {
        self.0
    }
}

#[repr(transparent)]
pub struct Context(pub *mut llama_cpp_sys::llama_context);
unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn context_size(&self) -> u32 {
        unsafe { llama_cpp_sys::llama_n_ctx(self.0) }
    }

    pub fn get_memory(&self) -> llama_cpp_sys::llama_memory_t {
        unsafe { llama_cpp_sys::llama_get_memory(self.0) }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { llama_cpp_sys::llama_free(self.0) };
        }
    }
}

#[repr(transparent)]
pub struct Sampler(pub *mut llama_cpp_sys::llama_sampler);
unsafe impl Send for Sampler {}
unsafe impl Sync for Sampler {}

impl Sampler {
    pub fn new(
        sampler_properties: &sauropod_inference_engine_api::SamplerProperties,
    ) -> Result<Self, Error> {
        let sampler = unsafe {
            llama_cpp_sys::llama_sampler_chain_init(
                llama_cpp_sys::llama_sampler_chain_default_params(),
            )
        };
        if sampler.is_null() {
            return Err(Error::FailedToCreateSampler);
        }

        if let Some(top_k) = sampler_properties.top_k {
            unsafe {
                llama_cpp_sys::llama_sampler_chain_add(
                    sampler,
                    llama_cpp_sys::llama_sampler_init_top_k(top_k as i32),
                );
            }
        }

        if let Some(top_p) = sampler_properties.top_p {
            unsafe {
                llama_cpp_sys::llama_sampler_chain_add(
                    sampler,
                    llama_cpp_sys::llama_sampler_init_top_p(top_p as f32, 1),
                );
            }
        }

        if let Some(min_p) = sampler_properties.min_p {
            unsafe {
                llama_cpp_sys::llama_sampler_chain_add(
                    sampler,
                    llama_cpp_sys::llama_sampler_init_min_p(min_p as f32, 1),
                );
            }
        }

        unsafe {
            llama_cpp_sys::llama_sampler_chain_add(
                sampler,
                llama_cpp_sys::llama_sampler_init_temp(sampler_properties.temperature as f32),
            );
        }

        let repetition_penalty = sampler_properties.repetition_penalty.unwrap_or(0.0);
        unsafe {
            llama_cpp_sys::llama_sampler_chain_add(
                sampler,
                llama_cpp_sys::llama_sampler_init_penalties(
                    if sampler_properties.repetition_penalty.is_some() {
                        1
                    } else {
                        0
                    },
                    repetition_penalty as f32,
                    0.0,
                    0.0,
                ),
            );
        }

        unsafe {
            llama_cpp_sys::llama_sampler_chain_add(
                sampler,
                llama_cpp_sys::llama_sampler_init_softmax(),
            );

            llama_cpp_sys::llama_sampler_chain_add(
                sampler,
                llama_cpp_sys::llama_sampler_init_dist(llama_cpp_sys::llama_default_seed),
            );
        }

        Ok(Self(sampler))
    }

    pub fn sample(&self, context: &Context) -> i32 {
        let token = unsafe { llama_cpp_sys::llama_sampler_sample(self.0, context.0, -1) };
        unsafe {
            llama_cpp_sys::llama_sampler_accept(self.0, token);
        }
        token
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { llama_cpp_sys::llama_sampler_free(self.0) };
        }
    }
}

/// Get the devices to use as a backend.
///
/// If no CUDA devices are found, an empty vector is returned causing a nullptr to be passed into `llama_cpp_sys::llama_model_load_from_file`
/// which will then try to do its own selection of the device to use (e.g. Vulkan or Metal).
fn get_devices() -> Vec<llama_cpp_sys::ggml_backend_dev_t> {
    let mut devices = Vec::new();
    let mut device_props = Vec::new();
    let backend_device_count = unsafe { llama_cpp_sys::ggml_backend_dev_count() };
    for backend_index in 0..backend_device_count {
        let device = unsafe { llama_cpp_sys::ggml_backend_dev_get(backend_index) };
        if device.is_null() {
            tracing::warn!("Failed to get device at index {}", backend_index);
            continue;
        }

        let mut props = std::mem::MaybeUninit::uninit();
        let props = unsafe {
            llama_cpp_sys::ggml_backend_dev_get_props(device, props.as_mut_ptr());
            props.assume_init()
        };
        devices.push(device);
        device_props.push(props);
    }

    let mut cuda_devices = Vec::with_capacity(1);
    for (device, props) in devices.into_iter().zip(device_props) {
        let name = unsafe { std::ffi::CStr::from_ptr(props.name) };
        if name.to_bytes().starts_with(b"CUDA") {
            cuda_devices.push(device);
        }
    }

    if !cuda_devices.is_empty() {
        cuda_devices
    } else {
        vec![]
    }
}

pub struct Model {
    ptr: *mut llama_cpp_sys::llama_model,
    chat_template: String,
    model_type: sauropod_output_parser::ModelType,
}
unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Model {
    /// Create a new model from a file.
    pub async fn from_file(path: &std::path::Path) -> Result<Self, Error> {
        init();

        let mut devices = get_devices();
        let mut progress_bar = indicatif::ProgressBar::new(u8::MAX as u64);

        let mut init_params = unsafe { llama_cpp_sys::llama_model_default_params() };
        init_params.n_gpu_layers = i32::MAX;
        init_params.progress_callback = Some(log_progress);
        init_params.progress_callback_user_data =
            &mut progress_bar as *mut _ as *mut std::os::raw::c_void;
        init_params.devices = if !devices.is_empty() {
            devices.push(std::ptr::null_mut());
            devices.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };

        let path_str = std::ffi::CString::new(path.as_os_str().as_encoded_bytes())?;
        let ctx =
            unsafe { llama_cpp_sys::llama_model_load_from_file(path_str.as_ptr(), init_params) };
        if ctx.is_null() {
            return Err(Error::FailedtoLoadModel(format!("{}", path.display())));
        }

        let mut metadata = sauropod_gguf::GgufMetadataParser::from_file(path).await?;
        let mut model_architecture = None;
        let mut model_chat_template = None;
        while let Some(entry) = metadata.get_next().await? {
            if entry.key == sauropod_gguf::CHAT_TEMPLATE_KEY {
                if let sauropod_gguf::GgufValue::String(template) = entry.value {
                    model_chat_template = Some(template);
                } else {
                    tracing::error!(
                        "Expected chat template to be a string, but got: {:#?}",
                        entry.value
                    );
                }
            } else if entry.key == sauropod_gguf::ARCHITECTURE_KEY {
                if let sauropod_gguf::GgufValue::String(template) = entry.value {
                    model_architecture = Some(template);
                } else {
                    tracing::error!(
                        "Expected architecture to be a string, but got: {:#?}",
                        entry.value
                    );
                }
            }
        }

        let model_type = match model_architecture.as_deref() {
            Some(x) if x.starts_with("gemma") => {
                sauropod_output_parser::ModelType::MarkdownToolCall
            }
            Some(x) if x.starts_with("qwen") => sauropod_output_parser::ModelType::Qwen3,
            _ => sauropod_output_parser::ModelType::Unknown,
        };

        Ok(Self {
            ptr: ctx,
            chat_template: match model_chat_template {
                Some(template) => template,
                None => {
                    return Err(Error::NoChatTemplate);
                }
            },
            model_type,
        })
    }

    pub fn get_vocab(&self) -> Result<Vocab, Error> {
        let vocab = unsafe { llama_cpp_sys::llama_model_get_vocab(self.ptr) };
        if vocab.is_null() {
            Err(Error::FailedtoLoadModel("Failed to get vocab".to_string()))
        } else {
            Ok(Vocab(vocab))
        }
    }

    fn llama_context(&self, prompt_length: i64, prediction_length: i64) -> Result<Context, Error> {
        let mut context_params = unsafe { llama_cpp_sys::llama_context_default_params() };
        context_params.n_ctx = prompt_length as u32 + prediction_length as u32 - 1;
        context_params.flash_attn = true;
        context_params.n_batch = (prompt_length as u32).max(llama_cpp_sys::ggml_kq_mask_pad);
        context_params.n_ubatch = llama_cpp_sys::ggml_kq_mask_pad;
        context_params.n_seq_max = 1;
        context_params.kv_unified = true;
        context_params.swa_full = true;
        context_params.no_perf = false;

        let ctx = unsafe { llama_cpp_sys::llama_init_from_model(self.ptr, context_params) };
        if ctx.is_null() {
            return Err(Error::FailedToCreateContext);
        }
        Ok(Context(ctx))
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { llama_cpp_sys::llama_model_free(self.ptr) };
        }
    }
}
