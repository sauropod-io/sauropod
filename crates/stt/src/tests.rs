use sauropod_inference_thread::InferenceProvider as _;

use crate::Stt;

const TEST_FILE: &str = "weather.wav";

#[tokio::test]
async fn test_stt() -> anyhow::Result<()> {
    sauropod_tracing_test_helpers::init_tracing();

    let ort_env = sauropod_onnxruntime::Env::new("unit-test")?;
    let model_dir = crate::download_from_huggingface(
        sauropod_config::Config::default()
            .stt_model
            .as_ref()
            .unwrap(),
    )
    .await?;
    let stt = Stt::new(&ort_env, &model_dir).await?;

    let (audio_data, _) = sauropod_hf_test_helpers::get_audio_file_content(TEST_FILE)?;

    let mut output = Vec::new();
    stt
        // Jump forward 0.25 seconds because this isn't using VAD
        .process(
            &[audio_data.iter().as_slice()[16_000usize / 4usize..].to_vec()],
            &mut output,
        )?;

    let text = output.into_iter().next().unwrap()?;
    assert_eq!(&text, "Hey Dino, what's the weather today?");

    Ok(())
}
