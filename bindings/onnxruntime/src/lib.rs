//! Sauropod's bindings around [ONNX Runtime](https://onnxruntime.ai/).

use std::os::unix::ffi::OsStrExt;

use onnxruntime_sys::OrtLoggingLevel;

mod helpers;
mod traits;
pub use traits::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("ONNX Runtime error: {0}")]
    OnnxRuntimeError(String),
    #[error("An unexpected nullptr was encountered: {0}")]
    UnexpectedNullPtr(String),
    #[error("JoinError: {0}")]
    JoinError(#[from] tokio::task::JoinError),
    #[error("InvalidShapeError: {0}")]
    InvalidShapeError(&'static str),
}

macro_rules! create_pointer_wrapper {
    ($(#[$outer:meta])* $vis:vis $name:ident ( $pointer:ident )) => {
        $(#[$outer])*
        #[repr(transparent)]
        $vis struct $name(*mut onnxruntime_sys::$pointer);

        unsafe impl Send for $name {}

        impl $name {
            /// Returns a pointer to the inner ONNX Runtime pointer.
            #[allow(dead_code)]
            $vis fn as_ptr(&self) -> *const onnxruntime_sys::$pointer {
                unsafe { *std::mem::transmute::<&Self, &*const onnxruntime_sys::$pointer>(self) }
            }

            /// Returns a mutable pointer to the inner ONNX Runtime pointer.
            #[allow(dead_code)]
            $vis fn as_mut_ptr(&self) -> *mut onnxruntime_sys::$pointer {
                unsafe { *std::mem::transmute::<&Self, &*mut onnxruntime_sys::$pointer>(self) }
            }

            /// Whether the pointer is null.
            #[allow(dead_code)]
            $vis fn is_null(&self) -> bool {
                self.as_ptr().is_null()
            }
        }


        impl From<*mut onnxruntime_sys::$pointer> for $name {
            fn from(pointer: *mut onnxruntime_sys::$pointer) -> Self {
                $name(pointer)
            }
        }
    };
    ($(#[$outer:meta])* $vis:vis $name:ident ( $pointer:ident ) drop : $drop:ident) => {
        create_pointer_wrapper!(
            $(#[$outer])*
            $vis $name ($pointer)
        );

        impl Drop for $name {
            fn drop(&mut self) {
                if !self.0.is_null() {
                    call_ort!($drop, self.0);
                }
            }
        }
    };
    ($(#[$outer:meta])* $vis:vis $name:ident ( $pointer:ident ) new: $new:ident drop: $drop:ident) => {
        create_pointer_wrapper!(
            $(#[$outer])*
            $vis $name ($pointer) drop: $drop
        );

        impl $name {
            $vis fn new() -> Result<Self, Error> {
                let mut pointer = std::ptr::null_mut();
                call_ort_checked!($new, &mut pointer);
                if pointer.is_null() {
                    return Err(Error::UnexpectedNullPtr(
                        "Failed to create ONNX Runtime pointer".to_string(),
                    ));
                }
                Ok($name(pointer))
            }
        }
    };
    ($vis:vis $name:ident ( $pointer:ident ) new: $new:ident drop: $drop:ident) => {
        create_pointer_wrapper!(
            /// A wrapper around an ONNX Runtime pointer type.
            $vis $name ($pointer) drop: $drop
        );
    };
}

/// Call an ONNX Runtime function using the API object.
macro_rules! call_ort {
    ($target:ident.$func:ident $(, $arg:expr)* $(,)?) => {
        unsafe {
            let Some(function) = $target.0.$func else {
                panic!("The ONNX Runtime function {} is not available.", stringify!($func));
            };
            function($($arg),*)
        }
    };
    ($func:ident $(, $arg:expr)* $(,)?) => {{
        let api = $crate::init_onnx_runtime_api();
        call_ort!(api.$func $(, $arg)*)
    }};
}

/// Call an ONNX Runtime function and check the status.
macro_rules! call_ort_checked {
    ($func:ident, $($arg:tt)*) => {
        let status = call_ort!($func, $($arg)*);
        $crate::Status::from(status).into_result()?;
    };
}

static ONNX_RUNTIME_API: std::sync::OnceLock<OrtApi> = std::sync::OnceLock::new();

/// Initialize the ONNX Runtime API.
pub fn init_onnx_runtime_api() -> &'static OrtApi {
    ONNX_RUNTIME_API.get_or_init(|| unsafe {
        let api_base = &*onnxruntime_sys::OrtGetApiBase();
        let Some(get_api) = api_base.GetApi else {
            panic!("The GetApi function in ONNX runtime is not available.");
        };

        let ort_api_pointer = get_api(onnxruntime_sys::OrtApiVersion);
        if ort_api_pointer.is_null() {
            panic!("The ONNX Runtime API pointer is null. Ensure ONNX Runtime is properly linked.");
        }
        OrtApi(&*ort_api_pointer)
    })
}

create_pointer_wrapper!(
    /// Onnx Runtime Status.
    ///
    /// If the inner value is a non-null pointer this represents an error status.
    Status(OrtStatus) drop:ReleaseStatus
);

impl Status {
    /// Convert the status into a `Result`.
    pub fn into_result(self) -> Result<(), Error> {
        if self.0.is_null() {
            Ok(())
        } else {
            let error_message_ptr = call_ort!(GetErrorMessage, self.0);
            let error_message = unsafe { std::ffi::CStr::from_ptr(error_message_ptr) };
            let error_message_string = error_message.to_string_lossy();
            Err(Error::OnnxRuntimeError(error_message_string.to_string()))
        }
    }
}

impl From<Status> for Result<(), Error> {
    fn from(val: Status) -> Self {
        val.into_result()
    }
}

/// ONNX Runtime API.
///
/// See <https://onnxruntime.ai/docs/api/c/struct_ort_api.html>
#[derive(Clone, Copy)]
pub struct OrtApi(&'static onnxruntime_sys::OrtApi);

unsafe impl Send for OrtApi {}

extern "C" fn onnxruntime_log(
    _param: *mut ::std::os::raw::c_void,
    severity: OrtLoggingLevel,
    _category: *const ::std::os::raw::c_char,
    _logid: *const ::std::os::raw::c_char,
    _code_location: *const ::std::os::raw::c_char,
    message: *const ::std::os::raw::c_char,
) {
    let c_str = unsafe { std::ffi::CStr::from_ptr(message) };
    let message_content = c_str.to_string_lossy();
    match severity {
        OrtLoggingLevel::ORT_LOGGING_LEVEL_FATAL | OrtLoggingLevel::ORT_LOGGING_LEVEL_ERROR => {
            tracing::event!(
                target: "ONNX Runtime",
                tracing::Level::ERROR,
                "{}",
                message_content
            );
        }
        OrtLoggingLevel::ORT_LOGGING_LEVEL_WARNING => {
            tracing::event!(
                target: "ONNX Runtime",
                tracing::Level::WARN,
                "{}",
                message_content
            );
        }
        OrtLoggingLevel::ORT_LOGGING_LEVEL_INFO => {
            tracing::event!(
                target: "ONNX Runtime",
                tracing::Level::INFO,
                "{}",
                message_content
            );
        }
        OrtLoggingLevel::ORT_LOGGING_LEVEL_VERBOSE => {
            tracing::event!(
                target: "ONNX Runtime",
                tracing::Level::DEBUG,
                "{}",
                message_content
            );
        }
    };
}

create_pointer_wrapper!(
    /// ONNX Runtime CUDA provider options.
    CUDAProviderOptionsV2(OrtCUDAProviderOptionsV2) new:CreateCUDAProviderOptions drop:ReleaseCUDAProviderOptions
);

impl CUDAProviderOptionsV2 {
    pub fn update_cuda_provider_options(
        &self,
        key_value_pair: &[(&str, String)],
    ) -> Result<(), Error> {
        let keys_c_str: Vec<std::ffi::CString> = helpers::keys_to_c_strings(key_value_pair);
        let values_c_str = helpers::values_to_c_strings(key_value_pair);
        let keys_c_str_ptr: Vec<*const std::os::raw::c_char> =
            helpers::c_strings_to_pointers(&keys_c_str);
        let values_c_str_ptr: Vec<*const std::os::raw::c_char> =
            helpers::c_strings_to_pointers(&values_c_str);

        call_ort_checked!(
            UpdateCUDAProviderOptions,
            self.as_mut_ptr(),
            keys_c_str_ptr.as_ptr(),
            values_c_str_ptr.as_ptr(),
            keys_c_str_ptr.len()
        );
        Ok(())
    }
}

create_pointer_wrapper!(
    /// ONNX Runtime TensorRT RTX provider options.
    NvTensorRtRtxProviderOptions(OrtNvTensorRtRtxProviderOptions)
);

create_pointer_wrapper!(
    /// ONNX Runtime session options.
    ///
    /// See <https://onnxruntime.ai/docs/api/c/struct_ort_1_1_session_options.html>
    SessionOptions(OrtSessionOptions) drop:ReleaseSessionOptions
);

impl SessionOptions {
    /// Create a session options object.
    pub fn new() -> Result<Self, Error> {
        let mut options_ptr = std::ptr::null_mut();
        call_ort_checked!(CreateSessionOptions, &mut options_ptr);
        call_ort_checked!(DisablePerSessionThreads, options_ptr);
        Ok(SessionOptions(options_ptr))
    }

    #[allow(dead_code)]
    pub fn set_log_severity_level(
        &self,
        level: onnxruntime_sys::OrtLoggingLevel,
    ) -> Result<(), Error> {
        call_ort_checked!(SetSessionLogSeverityLevel, self.as_mut_ptr(), level as i32);
        Ok(())
    }

    /// Append a execution provider to the session options.
    pub fn append_execution_provider(
        &mut self,
        provider: &str,
        key_value_pair: &[(&str, String)],
    ) -> Result<(), Error> {
        let provider_cstr = std::ffi::CString::new(provider)
            .map_err(|_| Error::OnnxRuntimeError("Invalid provider name".to_string()))?;
        let keys_c_str: Vec<std::ffi::CString> = helpers::keys_to_c_strings(key_value_pair);
        let values_c_str = helpers::values_to_c_strings(key_value_pair);
        let keys_c_str_ptr: Vec<*const std::os::raw::c_char> =
            helpers::c_strings_to_pointers(&keys_c_str);
        let values_c_str_ptr: Vec<*const std::os::raw::c_char> =
            helpers::c_strings_to_pointers(&values_c_str);

        call_ort_checked!(
            SessionOptionsAppendExecutionProvider,
            self.0,
            provider_cstr.as_ptr(),
            keys_c_str_ptr.as_ptr(),
            values_c_str_ptr.as_ptr(),
            keys_c_str_ptr.len()
        );
        Ok(())
    }

    /// Append a TensorRT execution provider to the session options.
    pub fn append_tensorrt_rtx_execution_provider(
        &mut self,
        key_value_pair: &[(&str, String)],
    ) -> Result<(), Error> {
        self.append_execution_provider("NvTensorRTRTXExecutionProvider", key_value_pair)
    }

    /// Append a CUDA execution provider to the session options.
    pub fn append_cuda_execution_provider(
        &mut self,
        key_value_pair: &[(&str, String)],
    ) -> Result<(), Error> {
        let cuda_provider_options = CUDAProviderOptionsV2::new()?;
        cuda_provider_options.update_cuda_provider_options(key_value_pair)?;
        call_ort_checked!(
            SessionOptionsAppendExecutionProvider_CUDA_V2,
            self.0,
            cuda_provider_options.as_mut_ptr()
        );
        Ok(())
    }
}

create_pointer_wrapper!(
    /// ONNX Runtime threading options.
    ///
    /// See <https://onnxruntime.ai/docs/api/c/struct_ort_1_1_threading_options.html>
    ThreadingOptions(OrtThreadingOptions) new:CreateThreadingOptions drop:ReleaseThreadingOptions
);

unsafe impl Sync for ThreadingOptions {}

impl ThreadingOptions {
    /// Set the number of threads to use.
    pub fn set_num_threads(&mut self, num_threads: i32) -> Result<(), Error> {
        call_ort_checked!(SetGlobalIntraOpNumThreads, self.0, num_threads);
        Ok(())
    }
}

create_pointer_wrapper!(
    /// ONNX Runtime environment.
    ///
    /// See <https://onnxruntime.ai/docs/api/c/struct_ort_1_1_env.html>
    pub Env(OrtEnv) drop:ReleaseEnv
);

unsafe impl Sync for Env {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionType {
    PreferTensorRT,
    PreferCUDA,
    CPU,
}

/// User-defined options for the session.
pub struct SessionUserOptions {
    /// The device ID to use for the session.
    pub device_id: Option<i32>,
    /// The type of the session.
    pub session_type: SessionType,
    /// Whether to allow CUDA graph capture.
    pub allow_cuda_graph: bool,
}

impl Env {
    /// Create a new ONNX Runtime environment.
    pub fn new(log_id: &str) -> Result<Self, Error> {
        let log_id = std::ffi::CString::new(log_id)
            .map_err(|_| Error::OnnxRuntimeError("Invalid log ID".to_string()))?;
        let mut threading_options = ThreadingOptions::new()?;
        threading_options.set_num_threads(8)?;

        let mut ort_env_ptr = std::ptr::null_mut();
        call_ort_checked!(
            CreateEnvWithCustomLoggerAndGlobalThreadPools,
            Some(onnxruntime_log),
            std::ptr::null_mut(),
            onnxruntime_sys::OrtLoggingLevel::ORT_LOGGING_LEVEL_VERBOSE,
            log_id.as_ptr(),
            threading_options.0,
            &mut ort_env_ptr
        );
        Ok(Env(ort_env_ptr))
    }

    /// Create a new ONNX Runtime session.
    pub fn create_session(
        &self,
        model_path: &dyn AsRef<std::path::Path>,
        session_user_options: SessionUserOptions,
    ) -> Result<Session, Error> {
        macro_rules! try_register_provider {
            ($name:expr, $expr:expr) => {
                if let Err(e) = $expr {
                    tracing::info!("Execution provider {} not available: {}", $name, e);
                }
            };
        }

        let model_path: &std::path::Path = model_path.as_ref();
        let model_path_cstr = std::ffi::CString::new(model_path.as_os_str().as_bytes())
            .map_err(|_| Error::OnnxRuntimeError("Invalid model path".to_string()))?;

        let mut session_options = SessionOptions::new()?;

        // On most platforms use Nvidia execution providers
        if cfg!(not(target_os = "macos")) {
            let mut common_options = vec![];
            if let Some(device_id) = session_user_options.device_id {
                common_options.push(("device_id", device_id.to_string()));
            } else {
                tracing::warn!("No device ID specified for ONNX Runtime session. Defaulting to 0.");
                common_options.push(("device_id", "0".to_string()));
            }

            if session_user_options.session_type == SessionType::PreferTensorRT {
                try_register_provider!(
                    "TensorRT RTX",
                    session_options.append_tensorrt_rtx_execution_provider(&common_options)
                );
            }

            if session_user_options.session_type != SessionType::CPU {
                common_options.push((
                    "enable_cuda_graph",
                    if session_user_options.allow_cuda_graph {
                        "1"
                    } else {
                        "0"
                    }
                    .to_string(),
                ));
                common_options.push(("use_ep_level_unified_stream", "1".to_string()));

                try_register_provider!(
                    "CUDA",
                    session_options.append_cuda_execution_provider(&common_options)
                );
            }
        }
        // For MacOS use CoreML
        if cfg!(target_os = "macos") && session_user_options.session_type != SessionType::CPU {
            try_register_provider!(
                "CoreML",
                session_options.append_execution_provider(
                    "CoreML",
                    &[
                        ("ModelFormat", "MLProgram".to_string()),
                        ("MLComputeUnits", "ALL".to_string())
                    ],
                )
            );
        }

        let mut session_ptr = std::ptr::null_mut();
        call_ort_checked!(
            CreateSession,
            self.0,
            model_path_cstr.as_ptr(),
            session_options.0,
            &mut session_ptr
        );
        let mut unsynchronized_run_options = RunOptions::new()?;
        unsynchronized_run_options
            .add_config_entry("disable_synchronize_execution_providers", "1")?;
        Ok(Session {
            session: RawSession(session_ptr),
            file_name: model_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            allocator: Allocator::new()?,
            unsynchronized_run_options,
        })
    }
}

#[repr(transparent)]
pub struct OwnedAllocator(Allocator);

impl AsRef<Allocator> for OwnedAllocator {
    fn as_ref(&self) -> &Allocator {
        &self.0
    }
}

impl AsMut<Allocator> for OwnedAllocator {
    fn as_mut(&mut self) -> &mut Allocator {
        &mut self.0
    }
}

impl Drop for OwnedAllocator {
    fn drop(&mut self) {
        if !self.0.is_null() {
            call_ort!(ReleaseAllocator, self.0.as_mut_ptr());
        }
    }
}

create_pointer_wrapper!(
    /// ONNX Runtime allocator.
    ///
    /// See <https://onnxruntime.ai/docs/api/c/struct_ort_1_1_allocator.html>
    pub Allocator(OrtAllocator)
);

unsafe impl Sync for Allocator {}

impl Allocator {
    /// Create a new ONNX Runtime allocator with default options.
    ///
    /// # Note
    ///
    /// Because this allocator is created with default options, it does not need a call to `ReleaseAllocator`.
    pub fn new() -> Result<Allocator, Error> {
        let mut allocator_ptr = std::ptr::null_mut();
        call_ort_checked!(GetAllocatorWithDefaultOptions, &mut allocator_ptr);
        Ok(Allocator(allocator_ptr))
    }

    /// Create a tensor with uninitialized memory.
    pub fn create_uninit_tensor<T: TensorDataType>(
        &self,
        shape: &[i64],
    ) -> Result<Value<'static>, Error> {
        let mut ort_value = std::ptr::null_mut();
        call_ort_checked!(
            CreateTensorAsOrtValue,
            self.0,
            shape.as_ptr(),
            shape.len(),
            T::get_onnx_tensor_element_data_type(),
            &mut ort_value
        );
        if ort_value.is_null() {
            panic!("Failed to create OrtValue from data");
        }
        Ok(Value {
            value: RawOrtValue(ort_value),
            _phantom: std::marker::PhantomData,
        })
    }

    /// Create a tensor with initialized memory.
    pub fn create_tensor<T: TensorDataType>(
        &self,
        default_value: T,
        shape: &[i64],
    ) -> Result<Value<'static>, Error> {
        let mut tensor = self.create_uninit_tensor::<T>(shape)?;
        let data = tensor.get_tensor_mutable_data::<T>()?;
        #[allow(clippy::needless_range_loop)]
        for index in 0..data.len() {
            data[index] = default_value;
        }
        Ok(tensor)
    }

    /// Create a tensor with initialized memory.
    pub fn create_tensor_with_value<T: TensorDataType>(
        &self,
        value: &[T],
        shape: &[i64],
    ) -> Result<Value<'static>, Error> {
        if value.len() != shape.iter().product::<i64>() as usize {
            return Err(Error::InvalidShapeError(
                "Value length does not match shape product",
            ));
        }
        let mut tensor = self.create_uninit_tensor::<T>(shape)?;
        let data = tensor.get_tensor_mutable_data::<T>()?;
        data.copy_from_slice(&value[..data.len()]);

        Ok(tensor)
    }
}

create_pointer_wrapper!(
    /// ONNX Runtime memory info.
    ///
    /// See <https://onnxruntime.ai/docs/api/c/struct_ort_1_1_memory_info.html>
    pub MemoryInfo(OrtMemoryInfo) drop:ReleaseMemoryInfo
);

unsafe impl Sync for MemoryInfo {}

impl MemoryInfo {
    /// Create a new memory info for CPU with default allocator type and memory type.
    pub fn cpu() -> Result<MemoryInfo, Error> {
        Self::cpu_with_options(
            onnxruntime_sys::OrtAllocatorType::OrtArenaAllocator,
            onnxruntime_sys::OrtMemType::OrtMemTypeCPU,
        )
    }

    /// Create a new memory info for CPU with specific allocator type and memory type.
    pub fn cpu_with_options(
        allocator_type: onnxruntime_sys::OrtAllocatorType,
        mem_type: onnxruntime_sys::OrtMemType,
    ) -> Result<MemoryInfo, Error> {
        let mut memory_info_ptr = std::ptr::null_mut();
        call_ort_checked!(
            CreateCpuMemoryInfo,
            allocator_type,
            mem_type,
            &mut memory_info_ptr
        );
        Ok(MemoryInfo(memory_info_ptr))
    }

    /// Create a new memory info for the device (e.g. GPU) to output to the CPU.
    pub fn cpu_output() -> Result<MemoryInfo, Error> {
        Self::cpu_with_options(
            onnxruntime_sys::OrtAllocatorType::OrtDeviceAllocator,
            onnxruntime_sys::OrtMemType::OrtMemTypeCPUOutput,
        )
    }

    /// Create a new memory info to send inputs from the CPU to the device (e.g. GPU).
    pub fn cpu_input() -> Result<MemoryInfo, Error> {
        Self::cpu_with_options(
            onnxruntime_sys::OrtAllocatorType::OrtDeviceAllocator,
            onnxruntime_sys::OrtMemType::OrtMemTypeCPUInput,
        )
    }

    /// Create a new memory info with specific name, allocator type, device ID, and memory type.
    pub fn new(
        name: &str,
        allocator_type: onnxruntime_sys::OrtAllocatorType,
        device_id: i32,
        mem_type: onnxruntime_sys::OrtMemType,
    ) -> Result<MemoryInfo, Error> {
        let name_cstr = std::ffi::CString::new(name)
            .map_err(|_| Error::OnnxRuntimeError("Invalid memory info name".to_string()))?;
        let mut memory_info_ptr = std::ptr::null_mut();
        call_ort_checked!(
            CreateMemoryInfo,
            name_cstr.as_ptr(),
            allocator_type,
            device_id,
            mem_type,
            &mut memory_info_ptr
        );
        Ok(MemoryInfo(memory_info_ptr))
    }

    /// Get the name of the memory info.
    pub fn name(&self) -> Result<String, Error> {
        let mut name_ptr = std::ptr::null();
        // MemoryInfoGetName returns void, not a status
        call_ort!(MemoryInfoGetName, self.0, &mut name_ptr);
        if name_ptr.is_null() {
            return Err(Error::UnexpectedNullPtr(
                "Memory info name pointer is null".to_string(),
            ));
        }
        let c_str = unsafe { std::ffi::CStr::from_ptr(name_ptr) };
        Ok(c_str.to_str()?.to_owned())
    }

    /// Get the device ID of the memory info.
    pub fn device_id(&self) -> Result<i32, Error> {
        let mut device_id = 0;
        // MemoryInfoGetId returns void, not a status
        call_ort!(MemoryInfoGetId, self.0, &mut device_id);
        Ok(device_id)
    }

    /// Get the memory type of the memory info.
    pub fn mem_type(&self) -> Result<onnxruntime_sys::OrtMemType, Error> {
        let mut mem_type = onnxruntime_sys::OrtMemType::OrtMemTypeDefault;
        call_ort_checked!(MemoryInfoGetMemType, self.0, &mut mem_type);
        Ok(mem_type)
    }

    /// Get the allocator type of the memory info.
    pub fn allocator_type(&self) -> Result<onnxruntime_sys::OrtAllocatorType, Error> {
        let mut allocator_type = onnxruntime_sys::OrtAllocatorType::OrtInvalidAllocator;
        call_ort_checked!(MemoryInfoGetType, self.0, &mut allocator_type);
        Ok(allocator_type)
    }

    /// Get the device type of the memory info.
    pub fn device_type(&self) -> Result<onnxruntime_sys::OrtMemoryInfoDeviceType, Error> {
        let mut device_type = onnxruntime_sys::OrtMemoryInfoDeviceType::OrtMemoryInfoDeviceType_CPU;
        // MemoryInfoGetDeviceType returns void, not a status
        call_ort!(MemoryInfoGetDeviceType, self.0, &mut device_type);
        Ok(device_type)
    }

    /// Compare this memory info with another memory info.
    ///
    /// # Returns
    ///
    /// `0` if they are equal, non-zero otherwise.
    pub fn compare(&self, other: &MemoryInfo) -> Result<i32, Error> {
        let mut result = 0;
        call_ort_checked!(CompareMemoryInfo, self.0, other.0, &mut result);
        Ok(result)
    }

    /// Check if this memory info is equal to another memory info.
    pub fn equals(&self, other: &MemoryInfo) -> Result<bool, Error> {
        Ok(self.compare(other)? == 0)
    }

    /// Creates a tensor with a user supplied buffer.
    pub fn create_tensor_with_data_as_ort_value<'a, T: TensorDataType>(
        &self,
        data: &'a [T],
        shape: &[i64],
    ) -> Result<Value<'a>, Error> {
        let mut ort_value = std::ptr::null_mut();
        call_ort_checked!(
            CreateTensorWithDataAsOrtValue,
            self.0,
            data.as_ptr() as *mut std::os::raw::c_void,
            std::mem::size_of_val(data),
            shape.as_ptr(),
            shape.len(),
            T::get_onnx_tensor_element_data_type(),
            &mut ort_value
        );
        if ort_value.is_null() {
            panic!("Failed to create OrtValue from data");
        }
        Ok(Value {
            value: RawOrtValue(ort_value),
            _phantom: std::marker::PhantomData,
        })
    }
}

/// ONNX Runtime value.
///
/// See <https://onnxruntime.ai/docs/api/c/struct_ort_1_1_value.html>.
#[repr(transparent)]
pub struct Value<'a> {
    /// Optional buffer to hold the data if it is created with `create_uninit_tensor`.
    #[allow(dead_code)]
    value: RawOrtValue,
    _phantom: std::marker::PhantomData<&'a ()>,
}

unsafe impl Send for Value<'_> {}
unsafe impl Sync for Value<'_> {}

impl std::fmt::Debug for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_info = self.type_info().map_err(|_| std::fmt::Error)?;
        let element_type = type_info
            .cast_to_tensor_type_info()
            .map_err(|_| std::fmt::Error)?;
        write!(f, "Value({element_type:?})")
    }
}

impl Value<'_> {
    /// Get the type info of the value.
    pub fn type_info(&self) -> Result<TypeInfo, Error> {
        let mut type_info_ptr = std::ptr::null_mut();
        call_ort_checked!(GetTypeInfo, self.value.as_mut_ptr(), &mut type_info_ptr);
        if type_info_ptr.is_null() {
            return Err(Error::UnexpectedNullPtr(
                "Type info pointer is null".to_string(),
            ));
        }
        Ok(TypeInfo(type_info_ptr))
    }

    /// Dimensions of the value.
    pub fn dimensions(&self) -> Result<Vec<i64>, Error> {
        let type_info = self.type_info()?;
        let tensor_type_info = type_info.cast_to_tensor_type_info()?;
        tensor_type_info.shape()
    }

    /// Get a access to the raw data inside a tensor.
    ///
    /// Used to read/write/modify the internal tensor data directly.
    pub fn get_tensor_mutable_data<T: TensorDataType>(&mut self) -> Result<&mut [T], Error> {
        let mut data_ptr = std::ptr::null_mut();
        call_ort_checked!(GetTensorMutableData, self.value.as_mut_ptr(), &mut data_ptr);
        if data_ptr.is_null() {
            return Err(Error::UnexpectedNullPtr(
                "Mutable data pointer is null".to_string(),
            ));
        }

        let data_len = self.dimensions()?.iter().product::<i64>() as usize;
        Ok(unsafe { std::slice::from_raw_parts_mut(data_ptr as *mut T, data_len) })
    }

    pub fn tensor_at<T: TensorDataType>(&self, indices: &[i64]) -> Result<T, Error> {
        let mut data_ptr: *mut T = std::ptr::null_mut();
        call_ort_checked!(
            TensorAt,
            self.value.as_mut_ptr(),
            indices.as_ptr(),
            indices.len(),
            (&mut data_ptr as *mut *mut T) as *mut *mut std::os::raw::c_void,
        );
        if data_ptr.is_null() {
            return Err(Error::UnexpectedNullPtr(
                "Tensor data pointer is null".to_string(),
            ));
        }
        Ok(unsafe { std::ptr::read(data_ptr) })
    }
}

impl Drop for Value<'_> {
    fn drop(&mut self) {
        if !self.value.is_null() {
            call_ort!(ReleaseValue, self.value.as_mut_ptr());
        }
    }
}

pub struct AllocatedString {
    ptr: *mut std::os::raw::c_char,
    allocator: *mut onnxruntime_sys::OrtAllocator,
}

impl AllocatedString {
    /// Convert the allocated string to a Rust `String`.
    pub fn to_string(self) -> Result<String, Error> {
        if self.ptr.is_null() {
            return Err(Error::UnexpectedNullPtr(
                "AllocatedString pointer is null".to_string(),
            ));
        }

        let c_str = unsafe { std::ffi::CStr::from_ptr(self.ptr) };
        Ok(c_str.to_str()?.to_owned())
    }
}

impl Drop for AllocatedString {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            call_ort!(AllocatorFree, self.allocator, self.ptr as *mut _);
        }
    }
}

create_pointer_wrapper!(
    /// ONNX Runtime type info.
    ///
    /// See <https://onnxruntime.ai/docs/api/c/struct_ort_1_1_type_info.html>
    pub TypeInfo(OrtTypeInfo) drop:ReleaseTypeInfo
);

impl TypeInfo {
    pub fn cast_to_tensor_type_info(&self) -> Result<TensorTypeAndShapeInfo<'_>, Error> {
        let mut tensor_type_info = std::ptr::null();
        call_ort_checked!(CastTypeInfoToTensorInfo, self.0, &mut tensor_type_info);
        if tensor_type_info.is_null() {
            return Err(Error::UnexpectedNullPtr(
                "Failed to cast TypeInfo to TensorTypeInfo".to_string(),
            ));
        }
        Ok(TensorTypeAndShapeInfo {
            value: tensor_type_info,
            _lifetime: std::marker::PhantomData,
        })
    }
}

/// ONNX Runtime tensor type and shape info.
///
/// This type is valid for the lifetime of the `TypeInfo` it is cast from.
pub struct TensorTypeAndShapeInfo<'a> {
    value: *const onnxruntime_sys::OrtTensorTypeAndShapeInfo,
    _lifetime: std::marker::PhantomData<&'a onnxruntime_sys::OrtTypeInfo>,
}

impl TensorTypeAndShapeInfo<'_> {
    /// Get the element type of the tensor.
    pub fn element_type(&self) -> Result<onnxruntime_sys::ONNXTensorElementDataType, Error> {
        let mut element_type =
            onnxruntime_sys::ONNXTensorElementDataType::ONNX_TENSOR_ELEMENT_DATA_TYPE_UNDEFINED;
        call_ort_checked!(GetTensorElementType, self.value, &mut element_type);
        Ok(element_type)
    }

    /// Get the shape of the tensor.
    pub fn shape(&self) -> Result<Vec<i64>, Error> {
        let mut dimension_count = 0;
        call_ort_checked!(GetDimensionsCount, self.value, &mut dimension_count);
        let mut dimensions = vec![0i64; dimension_count];
        call_ort_checked!(
            GetDimensions,
            self.value,
            dimensions.as_mut_ptr(),
            dimension_count
        );
        Ok(dimensions)
    }
}

impl std::fmt::Debug for TensorTypeAndShapeInfo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Value({:?}{:?})",
            self.element_type().map_err(|_| std::fmt::Error)?,
            self.shape().map_err(|_| std::fmt::Error)?
        )
    }
}

create_pointer_wrapper!(
    /// ONNX Runtime run options.
    ///
    /// See <https://onnxruntime.ai/docs/api/c/struct_ort_1_1_run_options.html>
    pub RunOptions(OrtRunOptions) drop:ReleaseRunOptions
);
unsafe impl Sync for RunOptions {}

impl RunOptions {
    /// Create a new run options object.
    pub fn new() -> Result<Self, Error> {
        let mut run_options_ptr = std::ptr::null_mut();
        call_ort_checked!(CreateRunOptions, &mut run_options_ptr);
        Ok(RunOptions(run_options_ptr))
    }

    pub fn add_config_entry(&mut self, key: &str, value: &str) -> Result<(), Error> {
        let key_cstr = std::ffi::CString::new(key)
            .map_err(|_| Error::OnnxRuntimeError("Invalid config key".to_string()))?;
        let value_cstr = std::ffi::CString::new(value)
            .map_err(|_| Error::OnnxRuntimeError("Invalid config value".to_string()))?;
        call_ort_checked!(
            AddRunConfigEntry,
            self.0,
            key_cstr.as_ptr(),
            value_cstr.as_ptr()
        );
        Ok(())
    }

    /// Terminates all currently executing Session::Run calls that were made using this RunOptions instance.
    pub fn set_terminate(&self) -> Result<(), Error> {
        call_ort_checked!(RunOptionsSetTerminate, self.0);
        Ok(())
    }
}

create_pointer_wrapper!(
    /// ONNX Runtime IO binding.
    ///
    /// See <https://onnxruntime.ai/docs/api/c/struct_ort_1_1detail_1_1_io_binding_impl.html>
    pub IoBinding(OrtIoBinding) drop:ReleaseIoBinding
);

impl IoBinding {
    /// Bind an input to the IO binding.
    pub fn bind_input(&self, name: &str, value: &Value<'_>) -> Result<(), Error> {
        let name_cstr = std::ffi::CString::new(name)
            .map_err(|_| Error::OnnxRuntimeError("Invalid input name".to_string()))?;
        call_ort_checked!(BindInput, self.0, name_cstr.as_ptr(), value.value.as_ptr());
        Ok(())
    }

    /// Bind an output to the IO binding.
    pub fn bind_output(&self, name: &str, value: &Value<'_>) -> Result<(), Error> {
        let name_cstr = std::ffi::CString::new(name)
            .map_err(|_| Error::OnnxRuntimeError("Invalid output name".to_string()))?;
        call_ort_checked!(BindOutput, self.0, name_cstr.as_ptr(), value.value.as_ptr());
        Ok(())
    }

    /// Bind an output to the IO binding.
    pub fn bind_output_with_memory_info(
        &self,
        name: &str,
        memory_info: &MemoryInfo,
    ) -> Result<(), Error> {
        let name_cstr = std::ffi::CString::new(name)
            .map_err(|_| Error::OnnxRuntimeError("Invalid output name".to_string()))?;
        call_ort_checked!(
            BindOutputToDevice,
            self.0,
            name_cstr.as_ptr(),
            memory_info.as_ptr()
        );
        Ok(())
    }

    /// Synchronize bound inputs.
    pub fn synchronize_bound_inputs(&self) -> Result<(), Error> {
        call_ort_checked!(SynchronizeBoundInputs, self.0);
        Ok(())
    }

    /// Synchronize bound outputs.
    pub fn synchronize_bound_outputs(&self) -> Result<(), Error> {
        call_ort_checked!(SynchronizeBoundOutputs, self.0);
        Ok(())
    }

    /// Retrieves the names of all bound output tensors.
    pub fn get_bound_output_names(&self, allocator: &Allocator) -> Result<Vec<String>, Error> {
        let mut output_names_ptr: *mut std::os::raw::c_char = std::ptr::null_mut();
        let mut output_count: usize = 0;
        let mut output_lengths: *mut usize = std::ptr::null_mut();

        call_ort!(
            GetBoundOutputNames,
            self.0,
            allocator.as_mut_ptr(),
            &mut output_names_ptr as *mut *mut _,
            &mut output_lengths,
            &mut output_count
        );

        // Nothing is allocated when 0 outputs are bound
        if output_count == 0 {
            return Ok(Vec::new());
        }

        let mut offset = 0;
        let output_lengths_slice =
            unsafe { std::slice::from_raw_parts(output_lengths, output_count) };
        let mut output_names = Vec::with_capacity(output_count);
        for length in &output_lengths_slice[0..output_count] {
            let end = offset + *length;
            let string_content =
                unsafe { std::slice::from_raw_parts(output_names_ptr.add(offset), *length) };
            output_names.push(
                string_content
                    .iter()
                    .map(|x| *x as char)
                    .collect::<String>(),
            );
            offset = end;
        }
        call_ort_checked!(
            AllocatorFree,
            allocator.as_mut_ptr(),
            output_names_ptr as *mut _
        );
        call_ort_checked!(
            AllocatorFree,
            allocator.as_mut_ptr(),
            output_lengths as *mut _
        );
        Ok(output_names)
    }

    // Retrieves the values of all bound output tensors.
    pub fn get_bound_output_values(
        &self,
        allocator: &Allocator,
    ) -> Result<Vec<Value<'static>>, Error> {
        let mut output_values_ptr: *mut *mut onnxruntime_sys::OrtValue = std::ptr::null_mut();
        let mut output_count: usize = 0;

        call_ort!(
            GetBoundOutputValues,
            self.0,
            allocator.as_mut_ptr(),
            &mut output_values_ptr as *mut *mut _,
            &mut output_count
        );

        let output_values: Vec<Value<'static>> =
            unsafe { std::slice::from_raw_parts(output_values_ptr, output_count) }
                .iter()
                .map(|x| Value {
                    value: RawOrtValue(*x),
                    _phantom: std::marker::PhantomData,
                })
                .collect();

        call_ort_checked!(
            AllocatorFree,
            allocator.as_mut_ptr(),
            output_values_ptr as *mut std::os::raw::c_void
        );

        Ok(output_values)
    }
}

/// ONNX Runtime session.
///
/// See <https://onnxruntime.ai/docs/api/c/struct_ort_1_1_session.html>
pub struct Session {
    /// The raw session pointer.
    session: RawSession,
    /// The file name of the model used to create this session.
    file_name: String,
    /// The allocator used for this session.
    pub allocator: Allocator,
    /// Run options with synchronization disabled.
    unsynchronized_run_options: RunOptions,
}
unsafe impl Send for Session {}
unsafe impl Sync for Session {}

impl Session {
    /// Get the input count for the session.
    pub fn get_input_count(&self) -> Result<usize, Error> {
        let mut count = 0;
        call_ort_checked!(SessionGetInputCount, self.session.as_mut_ptr(), &mut count);
        Ok(count)
    }

    /// Get the output count for the session.
    pub fn get_output_count(&self) -> Result<usize, Error> {
        let mut count = 0;
        call_ort_checked!(SessionGetOutputCount, self.session.as_mut_ptr(), &mut count);
        Ok(count)
    }

    /// Get an input name.
    pub fn get_input_name(&self, index: usize) -> Result<String, Error> {
        let mut result = AllocatedString {
            ptr: std::ptr::null_mut(),
            allocator: self.allocator.0,
        };
        call_ort_checked!(
            SessionGetInputName,
            self.session.as_mut_ptr(),
            index,
            self.allocator.0,
            &mut result.ptr
        );
        result.to_string()
    }

    /// Get an input type info.
    pub fn get_input_type_info(&self, index: usize) -> Result<TypeInfo, Error> {
        let mut result = std::ptr::null_mut();
        call_ort_checked!(
            SessionGetInputTypeInfo,
            self.session.as_mut_ptr(),
            index,
            &mut result
        );
        Ok(TypeInfo(result))
    }

    /// Get all input names.
    pub fn get_input_names(&self) -> Result<Vec<String>, Error> {
        let input_count = self.get_input_count()?;
        let mut inputs = Vec::with_capacity(input_count);
        for i in 0..input_count {
            let input_name = self.get_input_name(i)?;
            inputs.push(input_name);
        }
        Ok(inputs)
    }

    /// Get an output name.
    pub fn get_output_name(&self, index: usize) -> Result<String, Error> {
        let mut result = AllocatedString {
            ptr: std::ptr::null_mut(),
            allocator: self.allocator.0,
        };

        call_ort_checked!(
            SessionGetOutputName,
            self.session.as_mut_ptr(),
            index,
            self.allocator.0,
            &mut result.ptr
        );
        result.to_string()
    }

    /// Get an output type info.
    pub fn get_output_type_info(&self, index: usize) -> Result<TypeInfo, Error> {
        let mut result = std::ptr::null_mut();
        call_ort_checked!(
            SessionGetOutputTypeInfo,
            self.session.as_mut_ptr(),
            index,
            &mut result
        );
        Ok(TypeInfo(result))
    }

    /// Get an output type info.
    pub fn get_output_type_info_by_name(&self, name: &str) -> Result<TypeInfo, Error> {
        let Some(index) = self
            .get_output_names()?
            .iter()
            .position(|output_name| output_name == name)
        else {
            panic!("Output name not found: {name}");
        };
        self.get_output_type_info(index)
    }

    /// Get all output names.
    pub fn get_output_names(&self) -> Result<Vec<String>, Error> {
        let output_count = self.get_output_count()?;
        let mut outputs = Vec::with_capacity(output_count);
        for i in 0..output_count {
            let output_name = self.get_output_name(i)?;
            outputs.push(output_name);
        }
        Ok(outputs)
    }

    /// Create an IO binding for this session.
    pub fn create_io_binding(&self) -> Result<IoBinding, Error> {
        let mut io_binding_ptr = std::ptr::null_mut();
        call_ort_checked!(
            CreateIoBinding,
            self.session.as_mut_ptr(),
            &mut io_binding_ptr
        );
        if io_binding_ptr.is_null() {
            return Err(Error::UnexpectedNullPtr(
                "Failed to create IO binding".to_string(),
            ));
        }
        Ok(IoBinding(io_binding_ptr))
    }

    /// Run the model with the provided inputs.
    pub fn run(
        &self,
        inputs: &[(&str, &Value<'_>)],
        outputs: &[&str],
    ) -> Result<Vec<Value<'static>>, Error> {
        let span = tracing::info_span!("ONNX run", model = &self.file_name);
        let _guard = span.enter();

        let mut output_values: Vec<RawOrtValue> = Vec::with_capacity(outputs.len());
        output_values.resize_with(outputs.len(), || RawOrtValue(std::ptr::null_mut()));
        let input_names = helpers::keys_to_c_strings(inputs);
        let output_names = helpers::iter_to_c_strings(outputs.iter());
        let input_array: Vec<RawOrtValue> = inputs.iter().map(|(_, value)| value.value).collect();

        let input_names_ptrs = helpers::c_strings_to_pointers(&input_names);
        let output_names_ptrs = helpers::c_strings_to_pointers(&output_names);

        call_ort_checked!(
            Run,
            self.session.as_mut_ptr(),
            std::ptr::null_mut(), // Use default run options
            input_names_ptrs.as_ptr(),
            input_array.as_ptr() as *const *const onnxruntime_sys::OrtValue,
            input_array.len(),
            output_names_ptrs.as_ptr(),
            output_names_ptrs.len(),
            output_values.as_mut_ptr() as *mut *mut onnxruntime_sys::OrtValue,
        );

        let mut owned_outputs = Vec::with_capacity(output_values.len());
        let mut should_error = false;
        for (i, value) in output_values.into_iter().enumerate() {
            if value.0.is_null() {
                tracing::error!(
                    "Received a null output value for {:?} from ONNX Runtime",
                    &output_names[i]
                );
                should_error = true;
                continue;
            }
            owned_outputs.push(Value {
                value,
                _phantom: std::marker::PhantomData,
            });
        }
        if should_error {
            return Err(Error::UnexpectedNullPtr(
                "One or more output values from ONNX Runtime were not populated".to_string(),
            ));
        }

        Ok(owned_outputs)
    }

    /// Run the model with the provided inputs.
    pub fn run_with_io_binding(&self, io_binding: IoBinding) -> Result<IoBinding, Error> {
        let session = self.session;
        let run_options = RawRunOptions(self.unsynchronized_run_options.0);

        io_binding.synchronize_bound_inputs()?;
        {
            let span = tracing::info_span!("ONNX run", model = &self.file_name);
            let _guard = span.enter();
            call_ort_checked!(
                RunWithBinding,
                session.as_mut_ptr(),
                run_options.as_mut_ptr(),
                io_binding.as_mut_ptr(),
            );
        }
        io_binding.synchronize_bound_outputs()?;
        Ok(io_binding)
    }

    pub fn create_allocator(&self, memory_info: &MemoryInfo) -> Result<OwnedAllocator, Error> {
        let mut allocator = Allocator(std::ptr::null_mut());
        let allocator_ptr =
            (&mut allocator as *mut Allocator) as *mut *mut onnxruntime_sys::OrtAllocator;
        call_ort_checked!(
            CreateAllocator,
            self.session.as_ptr(),
            memory_info.as_ptr(),
            allocator_ptr
        );
        if allocator.0.is_null() {
            return Err(Error::UnexpectedNullPtr(
                "Failed to create allocator".to_string(),
            ));
        }
        Ok(OwnedAllocator(allocator))
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        if !self.session.is_null() {
            call_ort!(ReleaseSession, self.session.as_mut_ptr());
        }
    }
}

create_pointer_wrapper!(
    /// Non-owning wrapper to make `OrtValue` implement `Send`.
    #[derive(Copy, Clone)]
    RawOrtValue(OrtValue)
);

create_pointer_wrapper!(
    /// Non-owning wrapper to make `OrtSession` implement `Send`.
    #[derive(Copy, Clone)]
    RawSession(OrtSession)
);

create_pointer_wrapper!(
    /// Non-owning wrapper to make `OrtRunOptions` implement `Send`.
    RawRunOptions(OrtRunOptions)
);

/// Get the available ONNX Runtime providers.
///
/// See <https://onnxruntime.ai/docs/api/c/struct_ort_api.html#aaacd4a9540eb5044291addd2ebd9bc5f>
pub fn get_available_providers() -> Result<Vec<String>, Error> {
    let mut providers = std::ptr::null_mut();
    let mut provider_count = 0;
    call_ort_checked!(GetAvailableProviders, &mut providers, &mut provider_count);

    let mut result = Vec::with_capacity(provider_count as usize);
    let provider_slice = unsafe { std::slice::from_raw_parts(providers, provider_count as usize) };
    for &ptr in provider_slice {
        if ptr.is_null() {
            return Err(Error::UnexpectedNullPtr(
                "Provider pointer is null".to_string(),
            ));
        }
        let c_str = unsafe { std::ffi::CStr::from_ptr(ptr as *const _) };
        let provider_name = c_str.to_str()?.to_owned();
        result.push(provider_name);
    }

    call_ort_checked!(ReleaseAvailableProviders, providers, provider_count);
    Ok(result)
}
