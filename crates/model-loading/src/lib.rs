use std::{collections::HashMap, sync::Arc};

use anyhow::Context;
use sauropod_config::ConfigModelSource;
use tracing::Instrument as _;

/// The internal data of `LoadedModels`.
struct LoadedModelsInternal {
    /// Mapping from a model name to the loaded model.
    model_mapping: HashMap<String, Arc<sauropod_inference_engine::Model>>,
    /// Mapping from a model source to the loaded model pointer.
    tts_models: HashMap<String, Arc<sauropod_tts::ConfiguredTtsThread>>,
}

/// Loaded model state for Axum.
#[derive(Clone)]
pub struct LoadedModels {
    /// The actual internal state.
    ///
    /// The state needs to be copyable so it's stored as an `Arc`.
    internal: Arc<tokio::sync::RwLock<LoadedModelsInternal>>,
    /// The ONNX Runtime environment.
    pub onnxruntime_env: Arc<sauropod_onnxruntime::Env>,
    /// VAD model.
    pub vad_model: Arc<sauropod_vad::VadThread>,
    /// STT model.
    pub stt_model: Arc<sauropod_stt::SttThread>,
}

impl LoadedModels {
    /// Create a new `LoadedModels` instance.
    pub async fn new(config: &sauropod_config::Config) -> anyhow::Result<LoadedModels> {
        // We might have multiple models from the same source (to create aliases) so we need to track those
        let mut name_to_model =
            HashMap::<String, Arc<sauropod_inference_engine::Model>>::with_capacity(2);

        let onnxruntime_env = Arc::new(sauropod_onnxruntime::Env::new("sauropod")?);

        // Download VAD, STT, and TTS models
        let (vad_model_dir, stt_model_dir) = tokio::try_join!(
            sauropod_vad::download_from_huggingface(config.vad_model.as_ref().unwrap())
                .instrument(tracing::info_span!("download VAD model")),
            sauropod_stt::download_from_huggingface(config.stt_model.as_ref().unwrap())
                .instrument(tracing::info_span!("download STT model")),
        )?;

        // Load VAD model
        let vad_model = sauropod_vad::make_vad_thread(&onnxruntime_env, &vad_model_dir)
            .instrument(tracing::info_span!("load VAD model"))
            .await?;
        if let Err(e) = vad_model
            .enqueue(vec![
                0.0f32;
                sauropod_vad::Vad::CONTEXT_FRAMES
                    * sauropod_vad::Vad::FRAME_SIZE
            ])
            .instrument(tracing::info_span!("Warm up VAD"))
            .await
        {
            tracing::warn!("Failed to warm up VAD model: {e:?}");
        }

        // Load STT model
        let stt_model = sauropod_stt::make_stt_thread(&onnxruntime_env, &stt_model_dir)
            .instrument(tracing::info_span!("load STT model"))
            .await?;
        if let Err(e) = stt_model
            .enqueue(vec![0.0f32; 16000])
            .instrument(tracing::info_span!("Warm up STT"))
            .await
        {
            tracing::warn!("Failed to warm up STT model: {e:?}");
        }

        // Load LLM models
        let mut source_to_model_pointer =
            HashMap::<ConfigModelSource, sauropod_inference_engine::ModelPointer>::with_capacity(2);
        if config.models.is_empty() {
            tracing::warn!(
                "No models configured - you may be missing the models section in your config file."
            );
        }
        for (alias, model_config) in &config.models {
            let pointer = get_or_create(&mut source_to_model_pointer, &model_config.model, {
                let model_source = model_config.model.clone();
                let alias = alias.clone();
                let model_config = model_config.clone();
                async move || {
                    let model_path = sauropod_inference_engine::get_model_path(&model_source)
                        .await
                        .context(format!("Failed to get model path for {alias}"))?;

                    let llm_model = sauropod_inference_engine::load_model(
                        alias.to_string(),
                        &model_path,
                        model_config.multimodal_projector.as_ref(),
                    )
                    .await
                    .context(format!("Failed to load model for {alias}"))?;

                    let temporary_model = Arc::new(sauropod_inference_engine::Model::new(
                        llm_model.clone(),
                        model_config.clone(),
                    )?);

                    // Warm up the model by generating a response
                    let request = sauropod_openai_api::CreateResponse {
                        input: Some(sauropod_openai_api::CreateResponseInput::Variant0(
                            "Hello".to_string(),
                        )),
                        ..sauropod_openai_api::CreateResponse::default()
                    };
                    let render_context =
                        sauropod_prompt_templates::RenderContext::from_create_response(
                            &request, None,
                        )?;
                    match temporary_model
                        .generate(request, render_context)
                        .instrument(tracing::info_span!("Warm up model", alias = alias))
                        .await
                    {
                        Ok(response) => {
                            tracing::debug!("Warm up response was {response:#?}");
                        }
                        Err(e) => tracing::warn!("Failed to warm up model {alias}: {e:?}"),
                    }

                    Ok(llm_model)
                }
            })
            .await?;

            name_to_model.insert(
                alias.clone(),
                Arc::new(sauropod_inference_engine::Model::new(
                    pointer,
                    model_config.clone(),
                )?),
            );
        }

        let mut source_to_tts_pointer: HashMap<ConfigModelSource, Arc<sauropod_tts::TtsThread>> =
            HashMap::new();
        let mut tts_models: HashMap<String, Arc<sauropod_tts::ConfiguredTtsThread>> =
            HashMap::new();
        for (alias, voice_config) in &config.voices {
            let model = match &voice_config {
                sauropod_config::VoiceConfig::Kokoro {
                    voice,
                    model: model_source,
                    ..
                } => {
                    get_or_create(&mut source_to_tts_pointer, model_source, async || {
                        let model_dir = match &model_source {
                            ConfigModelSource::HuggingFace(repo) => {
                                sauropod_tts::kokoro::download_from_huggingface(repo).await?
                            }
                            ConfigModelSource::LocalPath(dir) => std::path::PathBuf::from(dir),
                        };
                        let model = Arc::new(
                            sauropod_tts::kokoro::make_tts_thread(&onnxruntime_env, &model_dir)
                                .await?,
                        );
                        model
                            .enqueue(sauropod_tts::TtsRequest {
                                text: "Hi.".to_string(),
                                voice: Some(voice.clone()),
                            })
                            .instrument(tracing::info_span!("Warm up Kokoro TTS"))
                            .await
                            .context("Failed to warm up Kokoro TTS")?;
                        Ok(model)
                    })
                    .await?
                }
                sauropod_config::VoiceConfig::Orpheus {
                    model: model_source,
                    ..
                } => {
                    get_or_create(&mut source_to_tts_pointer, model_source, async || {
                        let model = Arc::new(
                            sauropod_tts::orpheus::make_tts_thread(&onnxruntime_env, model_source)
                                .await?,
                        );
                        model
                            .enqueue(sauropod_tts::TtsRequest {
                                text: "Hi.".to_string(),
                                voice: None,
                            })
                            .instrument(tracing::info_span!("Warm up Orpheus TTS"))
                            .await
                            .context("Failed to warm up Orpheus TTS")?;
                        Ok(model)
                    })
                    .await?
                }
            };
            tts_models.insert(
                alias.clone(),
                sauropod_tts::ConfiguredTtsThread::new(
                    model,
                    voice_config.get_voice().map(|x| x.to_string()),
                ),
            );
        }

        let internal = Arc::new(tokio::sync::RwLock::new(LoadedModelsInternal {
            model_mapping: name_to_model,
            tts_models,
        }));
        Ok(LoadedModels {
            internal,
            onnxruntime_env,
            vad_model: Arc::new(vad_model),
            stt_model: Arc::new(stt_model),
        })
    }

    /// Get a loaded model by name.
    pub async fn get_model(
        &self,
        model_name: &str,
    ) -> Option<Arc<sauropod_inference_engine::Model>> {
        let internal = self.internal.read().await;
        internal.model_mapping.get(model_name).cloned()
    }

    /// Get a loaded voice model by name.
    pub async fn get_voice_model(
        &self,
        model_name: &str,
    ) -> Option<Arc<sauropod_tts::ConfiguredTtsThread>> {
        let internal = self.internal.read().await;
        internal.tts_models.get(model_name).cloned()
    }

    /// Get all the loaded models.
    pub async fn get_all_models(
        &self,
    ) -> tokio::sync::RwLockReadGuard<'_, HashMap<String, Arc<sauropod_inference_engine::Model>>>
    {
        tokio::sync::RwLockReadGuard::map(self.internal.read().await, |internal| {
            &internal.model_mapping
        })
    }
}

async fn get_or_create<T>(
    collection: &mut HashMap<ConfigModelSource, T>,
    key: &ConfigModelSource,
    create: impl AsyncFnOnce() -> anyhow::Result<T>,
) -> anyhow::Result<T>
where
    T: Clone + Send + 'static,
{
    if let Some(existing) = collection.get(key) {
        Ok(existing.clone())
    } else {
        let value = create().await?;
        collection.insert(key.clone(), value.clone());
        Ok(value)
    }
}
