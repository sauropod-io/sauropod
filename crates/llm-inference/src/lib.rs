use anyhow::Context;

mod interface;
pub use interface::*;

pub mod prompt_formatter;
pub use prompt_formatter::prepare_completion_request;

pub mod engine;
pub mod openai_api;
pub use engine::Engine;

/// An interface to an LLM engine using dynamic dispatch.
pub type EnginePointer = std::sync::Arc<crate::engine::Engine>;

/// The type of backend.
#[derive(Debug, Copy, Clone)]
pub(crate) enum Backend {
    /// `llama-server`.
    LlamaCpp,
    /// Ollama.
    Ollama,
}

/// Detect the backend type from the URL.
async fn detect_backend(url: &str) -> anyhow::Result<Backend> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| {
            format!(
                "Failed to send request to detect the LLM backend type at {}",
                url
            )
        })?
        .error_for_status()?;

    let headers = response.headers();

    if headers.get("Server") == Some(&"llama.cpp".parse().unwrap()) {
        return Ok(Backend::LlamaCpp);
    }

    let response_body = response.text().await?;
    if response_body.contains("Ollama") {
        return Ok(Backend::Ollama);
    }

    anyhow::bail!("Could not detect backend from {url}");
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
    }
    Ok(std::sync::Arc::new(engine::Engine::new(
        config.backend.clone(),
        backend,
    )))
}
