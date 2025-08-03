use sauropod_inference_thread::InferenceProvider as _;

use crate::Vad;

const TEST_FILE: &str = "weather.wav";

#[tokio::test]
async fn test_vad() -> anyhow::Result<()> {
    sauropod_tracing_test_helpers::init_tracing();

    let ort_env = sauropod_onnxruntime::Env::new("unit-test")?;
    let model_dir = crate::download_from_huggingface(
        sauropod_config::Config::default()
            .vad_model
            .as_ref()
            .unwrap(),
    )
    .await?;
    let vad = Vad::new(&ort_env, &model_dir).await?;

    let (audio_data, _) = sauropod_hf_test_helpers::get_audio_file_content(TEST_FILE)?;
    let chunks: Vec<Vec<f32>> = audio_data
        .chunks_exact(Vad::CONTEXT_SAMPLES)
        .map(|x| x.to_vec())
        .collect();

    let mut output = Vec::with_capacity(1);
    vad.process(&chunks, &mut output)?;

    assert_eq!(
        output.len(),
        3,
        "Output should contain one classification vector"
    );

    let mut output_iter = output.into_iter();
    let classifications_1 = output_iter.next().unwrap()?;
    let classifications_2 = output_iter.next().unwrap()?;
    let classifications_3 = output_iter.next().unwrap()?;

    // Check that we have some classifications
    for classification_set in [&classifications_1, &classifications_2, &classifications_3] {
        assert!(
            !classification_set.is_empty(),
            "Classification set should not be empty"
        );

        assert!(
            classification_set.iter().all(|c| !c.score.is_nan()),
            "None of the classification scores should be NaN"
        );
    }

    assert_eq!(
        classifications_1[0].range,
        0..Vad::FRAME_SIZE,
        "First classification range should be 0..FRAME_SIZE"
    );
    assert_eq!(
        classifications_1[classifications_1.len() - 1].range,
        Vad::CONTEXT_SAMPLES - Vad::FRAME_SIZE..Vad::CONTEXT_SAMPLES
    );

    for i in 0..10 {
        // First set
        assert!(
            classifications_1[i].score < 0.5,
            "Chunk 1 - First classification score should be less than 0.5"
        );

        assert!(
            classifications_1[classifications_1.len() / 2 - i].score < 0.5,
            "Chunk 1 - Middle classification score should be less than 0.5"
        );

        assert!(
            classifications_1[classifications_1.len() - i - 1].score > 0.5,
            "Chunk 1 - Last classification score should be greater than 0.5"
        );

        // Second set
        assert!(
            classifications_2[i].score > 0.5,
            "Chunk 2 - First classification score should be greater than 0.5"
        );

        assert!(
            classifications_2[classifications_2.len() / 2 - i].score > 0.5,
            "Chunk 2 - Middle classification score should be greater than 0.5"
        );
        assert!(
            classifications_2[classifications_2.len() - i - 1].score > 0.5,
            "Chunk 2 - Last classification score should be greater than 0.5"
        );

        // Third set
        assert!(
            classifications_3[i].score > 0.5,
            "Chunk 3 - First classification score should be greater than 0.5"
        );

        assert!(
            classifications_3[classifications_3.len() / 2 - i + 5].score < 0.5,
            "Chunk 3 - Middle classification score should be less than 0.5"
        );
        assert!(
            classifications_3[classifications_3.len() - i - 1].score < 0.5,
            "Chunk 3 - Last classification score should be less than 0.5"
        );
    }

    Ok(())
}
