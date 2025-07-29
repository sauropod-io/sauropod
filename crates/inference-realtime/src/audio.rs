use std::sync::Arc;

use rubato::Resampler as _;

use sauropod_openai_api::RealtimeServerEvent;

/// Store 2 megabytes worth of audio.
///
/// This is about 30 seconds.
const AUDIO_BUFFER_MAX_SIZE: usize = 2_000_000 / std::mem::size_of::<f32>();

/// Maintain at least 1 second of audio in the buffer.
const AUDIO_BUFFER_MIN_SIZE: usize = 2 * INTERNAL_SAMPLE_RATE;

/// Internal sample rate used for VAD.
pub(crate) const INTERNAL_SAMPLE_RATE: usize = 16_000;
/// Sample rate expected from the client.
const INPUT_SAMPLE_RATE: usize = 24_000;
/// Number of input frames per resampler chunk.
const RESAMPLE_CHUNK_SIZE: usize = 480;

pub(crate) const fn milliseconds_to_samples(ms: u32) -> usize {
    ms as usize * (INTERNAL_SAMPLE_RATE / 1000)
}

pub(crate) const fn samples_to_milliseconds(samples: usize) -> u32 {
    (samples / milliseconds_to_samples(1)) as u32
}

pub(crate) struct ResampledAudioBuffer {
    /// Raw audio buffer for incoming audio data.
    pub(crate) raw_audio_buffer: std::collections::VecDeque<f32>,
    /// Resampler for converting 24kHz input to the internal 16kHz sample rate.
    resampler: rubato::SincFixedIn<f32>,
    /// Buffer for pending samples before resampling.
    resample_buffer: Vec<f32>,
}

impl ResampledAudioBuffer {
    pub(crate) fn new() -> Self {
        let mut raw_audio_buffer = std::collections::VecDeque::with_capacity(AUDIO_BUFFER_MAX_SIZE);
        // Initialize the raw audio buffer 1 second of zeros.
        for _ in 0..AUDIO_BUFFER_MIN_SIZE {
            raw_audio_buffer.push_back(0.0f32);
        }
        let resampler = rubato::SincFixedIn::<f32>::new(
            INTERNAL_SAMPLE_RATE as f64 / INPUT_SAMPLE_RATE as f64,
            1.0,
            rubato::SincInterpolationParameters {
                sinc_len: 128,
                f_cutoff: 0.95,
                interpolation: rubato::SincInterpolationType::Linear,
                oversampling_factor: 128,
                window: rubato::WindowFunction::BlackmanHarris2,
            },
            RESAMPLE_CHUNK_SIZE,
            1,
        )
        .expect("failed to create resampler");

        Self {
            raw_audio_buffer,
            resampler,
            resample_buffer: Vec::with_capacity(RESAMPLE_CHUNK_SIZE),
        }
    }

    pub(crate) fn extend(&mut self, audio_data: impl IntoIterator<Item = i16>) {
        let input: Vec<f32> = audio_data.into_iter().map(|x| x as f32 / 32768.0).collect();
        let mut input_offset = 0;
        let mut temp_chunk = Vec::with_capacity(RESAMPLE_CHUNK_SIZE);

        // If resample_buffer has leftover samples, fill it up first
        if !self.resample_buffer.is_empty() {
            let needed = RESAMPLE_CHUNK_SIZE - self.resample_buffer.len();
            let take = needed.min(input.len());
            self.resample_buffer.extend_from_slice(&input[..take]);
            input_offset += take;

            if self.resample_buffer.len() == RESAMPLE_CHUNK_SIZE {
                temp_chunk.extend_from_slice(&self.resample_buffer);
                self.resample_buffer.clear();
                let resampled = self
                    .resampler
                    .process(&[temp_chunk.clone()], None)
                    .expect("resample failed");
                // Only one channel, so index 0
                for sample in &resampled[0] {
                    self.raw_audio_buffer.push_back(*sample);
                }
                temp_chunk.clear();
            }
        }

        // Process full chunks from input
        while input.len() - input_offset >= RESAMPLE_CHUNK_SIZE {
            let chunk = &input[input_offset..input_offset + RESAMPLE_CHUNK_SIZE];
            temp_chunk.extend_from_slice(chunk);
            input_offset += RESAMPLE_CHUNK_SIZE;

            let resampled = self
                .resampler
                .process(&[temp_chunk.clone()], None)
                .expect("resample failed");
            for sample in &resampled[0] {
                self.raw_audio_buffer.push_back(*sample);
            }
            temp_chunk.clear();
        }

        // Store any remaining samples in resample_buffer
        if input_offset < input.len() {
            self.resample_buffer
                .extend_from_slice(&input[input_offset..]);
        }

        // Truncate buffer if it exceeds max size
        while self.raw_audio_buffer.len() > AUDIO_BUFFER_MAX_SIZE {
            self.raw_audio_buffer.pop_front();
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.raw_audio_buffer.len()
    }

    pub(crate) fn range(
        &self,
        range: std::ops::Range<usize>,
    ) -> std::collections::vec_deque::Iter<'_, f32> {
        self.raw_audio_buffer.range(range)
    }

    pub(crate) fn clear(&mut self) {
        let buffer_len = self.len();
        self.raw_audio_buffer.rotate_left(buffer_len);
        self.raw_audio_buffer.truncate(0);
        self.resample_buffer.clear();
    }
}

pub(crate) struct AudioBuffer {
    /// The accumulated offset to the head of the `raw_audio_buffer` for the current session.
    pub(crate) accumulated_offset: usize,
    /// The offset in the `raw_audio_buffer` which has had VAD applied to it.
    pub(crate) vad_offset: usize,
    /// VAD classifications for the current session.
    pub(crate) vad_classifications: std::collections::VecDeque<VadClassification>,
    /// The VAD model.
    vad_model: Arc<sauropod_vad::VadThread>,
    /// The resampled audio buffer.
    resampled_buffer: ResampledAudioBuffer,
}

impl AudioBuffer {
    pub(crate) fn new(loaded_models: &sauropod_global_state::GlobalState) -> Self {
        let resampling = ResampledAudioBuffer::new();
        let vad_offset = AUDIO_BUFFER_MIN_SIZE;

        Self {
            accumulated_offset: 0,
            vad_offset,
            vad_classifications: std::collections::VecDeque::new(),
            vad_model: loaded_models.get_loaded_models().vad_model.clone(),
            resampled_buffer: resampling,
        }
    }

    /// Consume the audio buffer by removing the specified range of samples.
    pub(crate) fn consume_from_buffer(&mut self, consumed_range: std::ops::Range<usize>) {
        // Remove most of the consumed audio, but leave AUDIO_BUFFER_MIN_SIZE samples.
        let new_end = consumed_range.end.saturating_sub(AUDIO_BUFFER_MIN_SIZE);

        // Remove the consumed audio from the buffer
        self.resampled_buffer.raw_audio_buffer.rotate_left(new_end);
        self.resampled_buffer
            .raw_audio_buffer
            .truncate(self.resampled_buffer.raw_audio_buffer.len() - new_end);

        // Update the offsets
        self.vad_offset = self.vad_offset.saturating_sub(new_end);
        self.accumulated_offset += new_end;
    }

    /// Extend the audio buffer with new audio data.
    pub(crate) fn extend(&mut self, audio_data: impl IntoIterator<Item = i16>) {
        self.resampled_buffer.extend(audio_data);
    }

    /// Clear the audio buffer.
    pub(crate) fn clear(&mut self) {
        self.vad_classifications.clear();
        self.resampled_buffer.clear();
        self.vad_offset = AUDIO_BUFFER_MIN_SIZE;
        self.accumulated_offset = 0;
    }

    /// Get the current length of the audio buffer.
    pub(crate) fn len(&self) -> usize {
        self.resampled_buffer.len()
    }

    /// Get a range of samples from the audio buffer.
    pub(crate) fn range(
        &self,
        range: std::ops::Range<usize>,
    ) -> std::collections::vec_deque::Iter<'_, f32> {
        self.resampled_buffer.range(range)
    }

    /// Get the latest VAD classification from the audio buffer.
    ///
    /// If the buffer is empty, returns a default classification with a range of 0..0,
    pub(crate) fn get_latest_vad_classification(&self) -> VadClassification {
        match self.vad_classifications.back().cloned() {
            Some(x) => x,
            None => VadClassification {
                range: 0..0,
                voice_detected: false,
                item_id: crate::make_id(),
            },
        }
    }

    /// Run voice activity detection on the provided audio buffer.
    pub(crate) async fn run_vad(
        &mut self,
        socket: &crate::socket::SocketWrapper,
        silence_duration_ms: u32,
        prefix_padding_ms: u32,
        vad_threshold: f32,
    ) -> anyhow::Result<Vec<VadClassification>> {
        let vad_offset = self.vad_offset;
        let unchecked_samples = self.len() - vad_offset;
        let unchecked_frames = unchecked_samples / sauropod_vad::Vad::FRAME_SIZE;
        if unchecked_frames < 10 {
            return Ok(vec![]);
        }

        let context_frames = sauropod_vad::Vad::CONTEXT_FRAMES.saturating_sub(unchecked_frames);
        let context_samples = context_frames * sauropod_vad::Vad::FRAME_SIZE;

        let samples_to_run_vad_on = sauropod_vad::Vad::FRAME_SIZE * unchecked_frames;
        let audio_start_range = vad_offset.saturating_sub(context_samples);

        let audio_sample: Vec<_> = self
            .range(audio_start_range..vad_offset + samples_to_run_vad_on)
            .copied()
            .collect();

        self.vad_offset += samples_to_run_vad_on;

        let mut last_classification = self.get_latest_vad_classification();
        for vad_classification in self
            .vad_model
            .enqueue(audio_sample)
            .await
            .map_err(|e| anyhow::anyhow!("VAD processing failed: {}", e))?
            .into_iter()
            .skip(context_frames)
        {
            let start_offset = vad_offset - context_samples + self.accumulated_offset;
            let voice_detected = vad_classification.score > vad_threshold;
            let mut classification_to_add = None;

            if voice_detected == last_classification.voice_detected {
                match self.vad_classifications.back_mut() {
                    Some(last) => {
                        last.range.end = start_offset + vad_classification.range.end;
                    }
                    _ => {
                        classification_to_add = Some(VadClassification {
                            range: start_offset + vad_classification.range.start
                                ..start_offset + vad_classification.range.end,
                            item_id: crate::make_id(),
                            voice_detected,
                        });
                    }
                }
            } else {
                let new_classification = VadClassification {
                    range: start_offset + vad_classification.range.start
                        ..start_offset + vad_classification.range.end,
                    item_id: crate::make_id(),
                    voice_detected,
                };
                last_classification = new_classification.clone();
                classification_to_add = Some(new_classification);
            }

            if let Some(classification_to_add) = classification_to_add {
                if classification_to_add.voice_detected {
                    let should_send_start = self
                        .vad_classifications
                        .back()
                        .map(|x| {
                            x.range.end - x.range.start
                                >= milliseconds_to_samples(silence_duration_ms)
                        })
                        .unwrap_or(true);
                    if should_send_start {
                        socket
                            .send_event(RealtimeServerEvent::InputAudioBufferSpeechStarted {
                                event_id: crate::make_id(),
                                item_id: classification_to_add.item_id.clone(),
                                audio_start_ms: samples_to_milliseconds(
                                    classification_to_add.range.start,
                                ) as i64,
                            })
                            .await?;
                    }
                }
                self.vad_classifications.push_back(classification_to_add);
            }
        }

        let mut result = Vec::with_capacity(1);
        loop {
            let Some(silence_index) =
                self.vad_classifications
                    .iter()
                    .enumerate()
                    .find_map(|(i, x)| {
                        if i > 0
                            && !x.voice_detected
                            && x.range.end - x.range.start
                                >= milliseconds_to_samples(silence_duration_ms)
                        {
                            Some(i)
                        } else {
                            None
                        }
                    })
            else {
                break;
            };

            let (start_item, end_time) = {
                let mut time_chunks = self.vad_classifications.drain(..=silence_index);
                let start_item = time_chunks
                    .find(|x| x.voice_detected)
                    .expect("silence index always more than 0");
                let end_time = time_chunks.next_back().map(|x| x.range.end).unwrap_or(0);
                (start_item, end_time)
            };

            let audio_buffer_start_index = (start_item.range.start - self.accumulated_offset)
                .saturating_sub(milliseconds_to_samples(prefix_padding_ms));
            let audio_buffer_end_index = end_time - self.accumulated_offset;

            socket
                .send_event(RealtimeServerEvent::InputAudioBufferSpeechStopped {
                    event_id: crate::make_id(),
                    item_id: start_item.item_id.clone(),
                    audio_end_ms: samples_to_milliseconds(end_time) as i64,
                })
                .await?;
            socket
                .send_event(RealtimeServerEvent::InputAudioBufferCommitted {
                    event_id: crate::make_id(),
                    item_id: start_item.item_id.clone(),
                    previous_item_id: None, // TODO
                })
                .await?;

            result.push(VadClassification {
                range: audio_buffer_start_index..audio_buffer_end_index,
                voice_detected: true,
                item_id: start_item.item_id.clone(),
            });
        }
        Ok(result)
    }
}

/// A voice activity detection (VAD) classification for a chunk of audio.
#[derive(Debug, Clone)]
pub(crate) struct VadClassification {
    /// The range of samples that were classified.
    pub(crate) range: std::ops::Range<usize>,
    /// Whether a voice was detected.
    pub(crate) voice_detected: bool,
    /// The item ID associated with this detection.
    pub(crate) item_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resampling_audio_buffer_extend_and_len() {
        const IN_SAMPLES: usize = RESAMPLE_CHUNK_SIZE * 2;
        // const IN_SAMPLES: usize = RESAMPLE_CHUNK_SIZE * 2;
        const EXPECTED_OUT_SAMPLES: usize = (IN_SAMPLES as f64 / 1.5) as usize;

        let mut buffer = ResampledAudioBuffer::new();
        assert_eq!(buffer.len(), AUDIO_BUFFER_MIN_SIZE);

        // 1000 i16 samples at 24kHz should produce 640 samples at 16kHz after resampling
        let input_samples: Vec<i16> = (0..1000).map(|i| (i % 32768) as i16).collect();
        buffer.extend(input_samples.clone());

        assert!(buffer.len() - AUDIO_BUFFER_MIN_SIZE <= EXPECTED_OUT_SAMPLES);
        assert!(buffer.len() - AUDIO_BUFFER_MIN_SIZE >= EXPECTED_OUT_SAMPLES - 50);
    }
}
