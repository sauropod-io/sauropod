use std::str::FromStr;

use anyhow::Context as _;
use reqwest::Url;

mod interface;
pub use interface::*;
pub mod prompt_formatter;
pub use prompt_formatter::prepare_completion_request;
pub mod engine;
pub mod openai_api;
pub use engine::Engine;
mod extra_providers_api;

/// An interface to an LLM engine using dynamic dispatch.
pub type EnginePointer = std::sync::Arc<crate::engine::Engine>;

/// The type of backend.
#[derive(Debug, Copy, Clone)]
pub(crate) enum Backend {
    /// `llama-server`.
    LlamaCpp,
    /// Ollama.
    Ollama,
    /// Anthropic.
    Anthropic,
    /// An unknown backend.
    Unknown,
}

/// Detect the backend type from the URL.
async fn detect_backend(url_str: &str) -> anyhow::Result<Backend> {
    let url = Url::from_str(url_str).with_context(|| format!("Error parsing URL {url_str}"))?;
    if matches!(url.host_str(), Some("api.anthropic.com")) {
        return Ok(Backend::Anthropic);
    }

    let client = reqwest::Client::new();
    let response = client.get(url).send().await.with_context(|| {
        format!(
            "Failed to send request to detect the LLM backend type at {}",
            url_str
        )
    })?;

    if response.status() != reqwest::StatusCode::OK {
        tracing::warn!("Could not detect backend type from {url_str}");
        return Ok(Backend::Unknown);
    }

    let headers = response.headers();

    if headers.get("Server") == Some(&"llama.cpp".parse().unwrap()) {
        return Ok(Backend::LlamaCpp);
    }

    let response_body = response.text().await?;
    if response_body.contains("Ollama") {
        return Ok(Backend::Ollama);
    }

    tracing::warn!("Could not detect backend type from {url_str}");
    Ok(Backend::Unknown)
}

/// Create an inference engine.
pub async fn create_engine(config: &sauropod_config::Config) -> anyhow::Result<EnginePointer> {
    let backend = detect_backend(&config.backend).await?;
    match backend {
        Backend::LlamaCpp => {
            tracing::info!("Backend is llama.cpp");
        }
        Backend::Ollama => {
            tracing::info!("Backend is Ollama");
        }
        Backend::Anthropic => {
            tracing::error!(
                "Backend is Anthropic - Anthropic does not support structured output in their OpenAI yet"
            );
        }
        Backend::Unknown => {
            tracing::info!("Backend is Ollama");
        }
    }
    Ok(std::sync::Arc::new(engine::Engine::new(
        config.backend.clone(),
        backend,
        config.backend_api_key.clone(),
    )?))
}
