//! Multi-Scale Neural Audio Codec (SNAC).

use std::path::Path;

/// SNAC decoder
pub struct SnacDecoder {
    ort_session: sauropod_onnxruntime::Session,
    /// Memory info for input allocation.
    input_memory_info: sauropod_onnxruntime::MemoryInfo,
    /// Memory info for output allocation.
    output_memory_allocator: sauropod_onnxruntime::OwnedAllocator,
}

impl SnacDecoder {
    /// Load the preprocessor
    pub async fn new(
        ort_env: &sauropod_onnxruntime::Env,
        model_path: &Path,
        session_options: sauropod_onnxruntime::SessionUserOptions,
    ) -> anyhow::Result<Self> {
        let session = ort_env.create_session(&model_path, session_options)?;
        let input_memory_info = sauropod_onnxruntime::MemoryInfo::cpu_input()?;
        let output_memory_info = sauropod_onnxruntime::MemoryInfo::cpu_output()?;
        let output_memory_allocator = session.create_allocator(&output_memory_info)?;
        Ok(Self {
            ort_session: session,
            input_memory_info,
            output_memory_allocator,
        })
    }

    /// Process input to an audio element.
    ///
    /// # Returns
    ///
    /// The prepared data.
    #[tracing::instrument(skip(self, tokens))]
    pub fn decode(&self, tokens: &[i64]) -> anyhow::Result<Vec<f32>> {
        anyhow::ensure!(
            tokens.len() % 7 == 0,
            "Expected 7 tokens per batch for audio generation"
        );
        let batches = tokens.len() / 7;
        let mut codes_0: Vec<i64> = Vec::with_capacity(batches);
        let mut codes_1: Vec<i64> = Vec::with_capacity(batches * 2);
        let mut codes_2: Vec<i64> = Vec::with_capacity(batches * 4);
        for batch in tokens.chunks_exact(7) {
            codes_0.push(batch[0]);

            codes_1.push(batch[1]);
            codes_1.push(batch[4]);

            codes_2.push(batch[2]);
            codes_2.push(batch[3]);
            codes_2.push(batch[5]);
            codes_2.push(batch[6]);
        }

        let io_binding = self.ort_session.create_io_binding()?;

        let mut audio = self
            .output_memory_allocator
            .as_ref()
            .create_uninit_tensor::<f32>(&[1, 1, 2048 * batches as i64])?;
        io_binding.bind_output("audio_values", &audio)?;

        let audio_codes_0 = self
            .input_memory_info
            .create_tensor_with_data_as_ort_value(&codes_0, &[1, (batches as i64)])?;
        let audio_codes_1 = self
            .input_memory_info
            .create_tensor_with_data_as_ort_value(&codes_1, &[1, batches as i64 * 2])?;
        let audio_codes_2 = self
            .input_memory_info
            .create_tensor_with_data_as_ort_value(&codes_2, &[1, batches as i64 * 4])?;
        io_binding.bind_input("audio_codes.0", &audio_codes_0)?;
        io_binding.bind_input("audio_codes.1", &audio_codes_1)?;
        io_binding.bind_input("audio_codes.2", &audio_codes_2)?;
        let _ = self.ort_session.run_with_io_binding(io_binding)?;
        Ok(audio.get_tensor_mutable_data()?[2048..4096].to_vec())
    }
}
