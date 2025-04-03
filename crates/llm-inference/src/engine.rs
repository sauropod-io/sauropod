use anyhow::Context;
use tracing::Instrument;

use crate::openai_api;

/// Client for OpenAI compatible backends.
pub struct Engine {
    /// The backend.
    backend: crate::Backend,
    /// The backend URL.
    backend_url: String,
    /// Client used for backend.
    client: reqwest::Client,
    /// Interface to the OpenAI-compatible API that Ollama exposes.
    openai: crate::openai_api::OpenAiInterface,
}

impl Engine {
    /// Create a new engine.
    pub(crate) fn new(url: String, backend: crate::Backend) -> Self {
        let openai = format!("{}/v1", url.trim_end_matches('/'));
        Self {
            backend,
            backend_url: url,
            client: reqwest::Client::new(),
            openai: crate::openai_api::OpenAiInterface::new(openai),
        }
    }
}

impl Engine {
    /// List the available models.
    pub async fn list_models(&self) -> anyhow::Result<Vec<sauropod_schemas::ModelDefinition>> {
        let models_response = self
            .openai
            .models()
            .await
            .with_context(|| "Error fetching model list from inference service".to_string())?;
        let models = models_response
            .data
            .unwrap_or_default()
            .into_iter()
            .map(|model_data| sauropod_schemas::ModelDefinition {
                name: model_data.id.clone(),
                uri: String::new(),
            })
            .collect();
        Ok(models)
    }

    /// Check if the engine can pull models.
    pub fn can_pull_model(&self) -> bool {
        matches!(self.backend, crate::Backend::Ollama)
    }

    /// Pull a model.
    pub async fn pull_model(&self, model: &str) -> anyhow::Result<()> {
        match self.backend {
            crate::Backend::Ollama => {
                tracing::info!("Pulling model {}", model);
                self.client
                    .post(format!("{}/api/pull", self.backend_url))
                    .header("Accept", "application/json")
                    .json(&serde_json::json!({
                        "model": model,
                        "stream": false,
                    }))
                    .send()
                    .instrument(tracing::info_span!("pull_model", model = model))
                    .await?;
                Ok(())
            }
            _ => {
                anyhow::bail!(
                    "Pulling models is not supported by the {:?} backend",
                    self.backend
                );
            }
        }
    }

    /// Run a prompt through the engine.
    pub async fn invoke(
        &self,
        completion_request: &openai_api::CompletionRequest,
    ) -> anyhow::Result<openai_api::CompletionResponse> {
        return self.openai.completions(completion_request).await;
    }
}
