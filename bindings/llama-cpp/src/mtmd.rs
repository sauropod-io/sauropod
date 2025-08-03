use std::os::unix::ffi::OsStrExt as _;

use crate::Error;

#[repr(transparent)]
pub struct MtmdContext(pub *mut llama_cpp_sys::mtmd_context);
unsafe impl Send for MtmdContext {}
unsafe impl Sync for MtmdContext {}

impl MtmdContext {
    pub fn new(
        mmproj_file: &std::path::Path,
        llama_model: *mut llama_cpp_sys::llama_model,
        context_params: llama_cpp_sys::mtmd_context_params,
    ) -> Result<Self, Error> {
        let mmproj_path = std::ffi::CString::new(mmproj_file.as_os_str().as_bytes())?;
        let mtmd_context = unsafe {
            llama_cpp_sys::mtmd_init_from_file(mmproj_path.as_ptr(), llama_model, context_params)
        };
        if mtmd_context.is_null() {
            return Err(Error::FailedToCreateMtmdContext);
        }
        Ok(Self(mtmd_context))
    }

    pub fn tokenize(
        &self,
        text: &str,
        bitmaps: &mut [MtmdBitmap],
    ) -> Result<MtmdInputChunks, Error> {
        let text_cstr = std::ffi::CString::new(text)?;
        let input_text = llama_cpp_sys::mtmd_input_text {
            text: text_cstr.as_ptr(),
            add_special: true,
            parse_special: true,
        };
        let mtmd_chunks = MtmdInputChunks::new()?;
        let result = unsafe {
            llama_cpp_sys::mtmd_tokenize(
                self.0,
                mtmd_chunks.0,
                &input_text,
                std::mem::transmute::<*mut MtmdBitmap, *mut *const llama_cpp_sys::mtmd_bitmap>(
                    bitmaps.as_mut_ptr(),
                ),
                bitmaps.len(),
            )
        };
        match result {
            0 => Ok(mtmd_chunks),
            1 => Err(Error::MtmdNumberOfBitsmapsDidNotMatchMarkers(bitmaps.len())),
            2 => Err(Error::MtmdImagePreprocessingError),
            x => Err(Error::MtmdTokenizationError(x)),
        }
    }
}

impl Drop for MtmdContext {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { llama_cpp_sys::mtmd_free(self.0) };
        }
    }
}

#[repr(transparent)]
pub struct MtmdInputChunks(pub *mut llama_cpp_sys::mtmd_input_chunks);

impl MtmdInputChunks {
    pub fn new() -> Result<Self, Error> {
        let input_chunks = unsafe { llama_cpp_sys::mtmd_input_chunks_init() };
        if input_chunks.is_null() {
            return Err(Error::FailedToCreateMtmdInputChunks);
        }
        Ok(Self(input_chunks))
    }

    /// Get the number of input chunks.
    pub fn len(&self) -> usize {
        unsafe { llama_cpp_sys::mtmd_input_chunks_size(self.0) }
    }

    /// Get a chunk by its index.
    pub fn get(&self, index: usize) -> *const llama_cpp_sys::mtmd_input_chunk {
        unsafe { llama_cpp_sys::mtmd_input_chunks_get(self.0, index) }
    }
}

impl Drop for MtmdInputChunks {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { llama_cpp_sys::mtmd_input_chunks_free(self.0) };
        }
    }
}

#[repr(transparent)]
pub struct MtmdBitmap(pub *mut llama_cpp_sys::mtmd_bitmap);

impl MtmdBitmap {
    /// Create a new `MtmdBitmap` from RGB data.
    pub fn new_rgb(width: u32, height: u32, rgb: &[u8]) -> Result<Self, Error> {
        let bitmap = unsafe { llama_cpp_sys::mtmd_bitmap_init(width, height, rgb.as_ptr()) };
        if bitmap.is_null() {
            return Err(Error::FailedToCreateMtmdBitmap);
        }
        Ok(Self(bitmap))
    }

    /// Create a new `MtmdBitmap` from audio data.
    pub fn new_audio(audio: &[f32]) -> Result<Self, Error> {
        let bitmap =
            unsafe { llama_cpp_sys::mtmd_bitmap_init_from_audio(audio.len(), audio.as_ptr()) };
        if bitmap.is_null() {
            return Err(Error::FailedToCreateMtmdBitmap);
        }
        Ok(Self(bitmap))
    }
}

impl Drop for MtmdBitmap {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { llama_cpp_sys::mtmd_bitmap_free(self.0) };
        }
    }
}
