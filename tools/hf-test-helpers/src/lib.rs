//! Unit test helper functions for getting data from Hugging Face Hub.

use symphonia::core::audio::Signal as _;
use symphonia::core::conv::FromSample;

fn sample_to_f32<T>(
    samples: &mut Vec<f32>,
    data: std::borrow::Cow<symphonia::core::audio::AudioBuffer<T>>,
) where
    T: symphonia::core::sample::Sample,
    f32: FromSample<T>,
{
    data.chan(0)
        .iter()
        .copied()
        .for_each(|v| samples.push(f32::from_sample(v)))
}

/// Load an audio file and return the 16-bit PCM data and sample rate.
pub fn get_audio_file_content(file_path: &str) -> anyhow::Result<(Vec<f32>, u32)> {
    use symphonia::core::audio::AudioBufferRef;

    let audio_file = std::fs::File::open(get_file(file_path))?;
    let media_source_stream =
        symphonia::core::io::MediaSourceStream::new(Box::new(audio_file), Default::default());
    let hint = symphonia::core::probe::Hint::new();

    // Use the default options for metadata and format readers.
    let meta_opts = symphonia::core::meta::MetadataOptions::default();
    let fmt_opts = symphonia::core::formats::FormatOptions::default();

    // Probe the media source.
    let probed = symphonia::default::get_probe().format(
        &hint,
        media_source_stream,
        &fmt_opts,
        &meta_opts,
    )?;
    // Get the instantiated format reader.
    let mut format = probed.format;

    // Find the first audio track with a known (decodeable) codec.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
        .expect("no supported audio tracks");

    // Use the default options for the decoder.
    let dec_opts = symphonia::core::codecs::DecoderOptions::default();
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");
    let track_id = track.id;
    let sample_rate = track.codec_params.sample_rate.unwrap_or(0);
    let mut pcm_data = Vec::with_capacity(4096);

    // The decode loop.
    while let Ok(packet) = format.next_packet() {
        // Consume any new metadata that has been read since the last packet.
        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        // If the packet does not belong to the selected track, skip over it.
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet)? {
            AudioBufferRef::F32(data) => sample_to_f32(&mut pcm_data, data),
            AudioBufferRef::U8(data) => sample_to_f32(&mut pcm_data, data),
            AudioBufferRef::U16(data) => sample_to_f32(&mut pcm_data, data),
            AudioBufferRef::U24(data) => sample_to_f32(&mut pcm_data, data),
            AudioBufferRef::U32(data) => sample_to_f32(&mut pcm_data, data),
            AudioBufferRef::S8(data) => sample_to_f32(&mut pcm_data, data),
            AudioBufferRef::S16(data) => sample_to_f32(&mut pcm_data, data),
            AudioBufferRef::S24(data) => sample_to_f32(&mut pcm_data, data),
            AudioBufferRef::S32(data) => sample_to_f32(&mut pcm_data, data),
            AudioBufferRef::F64(data) => sample_to_f32(&mut pcm_data, data),
        }
    }

    Ok((pcm_data, sample_rate))
}

/// Get a file from the unit test dataset on Hugging Face.
pub fn get_file(file_name: &str) -> std::path::PathBuf {
    let client = match hf_hub::api::sync::ApiBuilder::from_env().build() {
        Ok(client) => client,
        Err(e) => panic!("Failed to create Hugging Face client: {e}"),
    };

    let dataset = client.repo(hf_hub::Repo::with_revision(
        "sauropod/unit-test-data".to_string(),
        hf_hub::RepoType::Dataset,
        "ce3de1720443e49ee9bb18247a6fc35637158b42".to_string(),
    ));

    match dataset.get(file_name) {
        Ok(file) => file,
        Err(e) => panic!("Failed to get test data file: {e}"),
    }
}
