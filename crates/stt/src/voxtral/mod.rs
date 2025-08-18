//! Voxtral-based speech-to-text (STT) model implementation.

use tokio_stream::StreamExt;

pub struct Voxtral {
    model: sauropod_inference_engine::ModelPointer,
}

impl Voxtral {
    pub async fn new(
        model_source: &sauropod_config::ConfigModelSource,
        projector_model_source: &sauropod_config::ConfigModelSource,
    ) -> anyhow::Result<Self> {
        let model_path = sauropod_inference_engine::get_model_path(model_source).await?;
        let model = sauropod_inference_engine::load_model(
            "voxtral".to_string(),
            &model_path,
            Some(projector_model_source),
        )
        .await?;

        Ok(Voxtral { model })
    }

    async fn transcribe(&self, audio_input: Vec<f32>) -> anyhow::Result<String> {
        let sampler_properties = sauropod_inference_engine_api::SamplerProperties {
            temperature: 0.0, // Recommended by Mistral when transcribing,
            max_predict: (audio_input.len() / 400).clamp(1024, 32_000), // Allow 40 tokens per second of audio
            min_p: None,
            top_k: None,
            top_p: None,
            repetition_penalty: None,
        };
        let sauropod_inference_engine_api::GenerateFromTextResponse { mut stream, .. } = self
            .model
            .clone()
            .generate_from_text(
                sampler_properties,
                "<s>[INST]<__media__>[/INST]lang:en[TRANSCRIBE]".to_string(),
                vec![sauropod_prompt_templates::MultimodalData::Audio(
                    audio_input.clone(),
                )],
            )
            .await?;
        let mut text = String::with_capacity(128);
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            text.push_str(&chunk);
        }
        Ok(text)
    }
}

impl sauropod_inference_thread::InferenceProvider for Voxtral {
    type Input = Vec<f32>;
    type Output = String;

    fn process(
        &self,
        input: &[Self::Input],
        output: &mut Vec<anyhow::Result<Self::Output>>,
    ) -> anyhow::Result<()> {
        anyhow::ensure!(input.len() == 1, "STT model expects a single audio input");
        let audio_input = &input[0];
        let text = futures::executor::block_on(self.transcribe(audio_input.clone()));
        output.push(text);
        Ok(())
    }
}
