//! Audio utilities.

use std::path::Path;

mod snac;
pub use snac::SnacDecoder;

/// Preprocessor for audio data
pub struct Preprocessor {
    ort_session: sauropod_onnxruntime::Session,
    /// Memory info for input allocation.
    input_memory_info: sauropod_onnxruntime::MemoryInfo,
    /// Memory info for output allocation.
    output_memory_allocator: sauropod_onnxruntime::OwnedAllocator,
    /// The feature count output from the preprocessor.
    feature_count: i64,
}

pub struct Features {
    pub features: sauropod_onnxruntime::Value<'static>,
    pub features_length: sauropod_onnxruntime::Value<'static>,
}

const FEATURES_NAME: &str = "features";
const FEATURES_LENS_NAME: &str = "features_lens";
const HOP_LENGTH: i64 = 160;

impl Preprocessor {
    /// Load the preprocessor
    pub async fn new(
        ort_env: &sauropod_onnxruntime::Env,
        model_path: &Path,
        session_options: sauropod_onnxruntime::SessionUserOptions,
    ) -> anyhow::Result<Self> {
        let session = ort_env.create_session(&model_path, session_options)?;
        let output_shape = session
            .get_output_type_info_by_name(FEATURES_NAME)?
            .cast_to_tensor_type_info()?
            .shape()?;
        if output_shape.len() != 3 {
            return Err(anyhow::anyhow!(
                "Expected output shape of 3 dimensions, got {output_shape:?}",
            ));
        }

        let input_memory_info = sauropod_onnxruntime::MemoryInfo::cpu_input()?;
        let output_memory_info = sauropod_onnxruntime::MemoryInfo::cpu_output()?;
        let output_memory_allocator = session.create_allocator(&output_memory_info)?;
        let feature_count = output_shape[1];
        if feature_count == -1 {
            return Err(anyhow::anyhow!(
                "Feature count in output shape is -1, which is not allowed"
            ));
        }
        Ok(Self {
            ort_session: session,
            input_memory_info,
            output_memory_allocator,
            feature_count,
        })
    }

    /// Get the number of features produced by the preprocessor.
    pub fn get_output_length(&self, audio_length: usize) -> i64 {
        audio_length as i64 / HOP_LENGTH + 1
    }

    /// Process a slice of f32 PCM samples.
    ///
    /// # Returns
    ///
    /// The prepared data.
    #[tracing::instrument(skip(self, audio))]
    pub fn preprocess(&self, audio: &[f32]) -> anyhow::Result<Features> {
        let io_binding = self.ort_session.create_io_binding()?;

        let last_dim = self.get_output_length(audio.len());
        let features = self
            .output_memory_allocator
            .as_ref()
            .create_uninit_tensor::<f32>(&[1, self.feature_count, last_dim])?;
        let features_length = self
            .output_memory_allocator
            .as_ref()
            .create_uninit_tensor::<i64>(&[1])?;
        io_binding.bind_output(FEATURES_NAME, &features)?;
        io_binding.bind_output(FEATURES_LENS_NAME, &features_length)?;

        let pcm = self
            .input_memory_info
            .create_tensor_with_data_as_ort_value(audio, &[1, audio.len() as i64])?;
        io_binding.bind_input("audio", &pcm)?;
        let pcm_length = [audio.len() as i64];
        let pcm_length = self
            .input_memory_info
            .create_tensor_with_data_as_ort_value(&pcm_length, &[1])?;
        io_binding.bind_input("audio_length", &pcm_length)?;

        let _ = self.ort_session.run_with_io_binding(io_binding)?;
        Ok(Features {
            features,
            features_length,
        })
    }

    /// Process a slice of f32 PCM samples.
    ///
    /// # Returns
    ///
    /// The prepared data.
    #[tracing::instrument(skip(self, audio_batch))]
    pub fn preprocess_batch(
        &self,
        audio_batch: &[f32],
        element_length: usize,
    ) -> anyhow::Result<Features> {
        let io_binding = self.ort_session.create_io_binding()?;

        if audio_batch.len() % element_length != 0 {
            return Err(anyhow::anyhow!(
                "Audio batch length {} must be a multiple of the element length {}",
                audio_batch.len(),
                element_length
            ));
        }

        let batch_size = (audio_batch.len() / element_length) as i64;
        let last_dim = self.get_output_length(element_length);
        let features = self
            .output_memory_allocator
            .as_ref()
            .create_uninit_tensor::<f32>(&[batch_size, self.feature_count, last_dim])?;
        let features_length = self
            .output_memory_allocator
            .as_ref()
            .create_uninit_tensor::<i64>(&[batch_size])?;
        io_binding.bind_output(FEATURES_NAME, &features)?;
        io_binding.bind_output(FEATURES_LENS_NAME, &features_length)?;

        let pcm = self
            .input_memory_info
            .create_tensor_with_data_as_ort_value(
                audio_batch,
                &[batch_size, element_length as i64],
            )?;
        io_binding.bind_input("audio", &pcm)?;
        let pcm_length = vec![element_length as i64; batch_size as usize];
        let pcm_length = self
            .input_memory_info
            .create_tensor_with_data_as_ort_value(&pcm_length, &[batch_size])?;
        io_binding.bind_input("audio_length", &pcm_length)?;

        let _ = self.ort_session.run_with_io_binding(io_binding)?;

        Ok(Features {
            features,
            features_length,
        })
    }
}
