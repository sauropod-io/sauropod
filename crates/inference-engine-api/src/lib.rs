//! API for loading and using LLM models.

use std::sync::Arc;

mod response_stream;
pub use response_stream::ResponseStreamCreator;
mod sampling;
pub use sampling::SamplerProperties;

/// Create an empty response from a request.
pub fn make_response(
    request: &sauropod_openai_api::CreateResponse,
) -> sauropod_openai_api::Response {
    sauropod_openai_api::Response {
        id: uuid::Uuid::new_v4().to_string(),
        object: sauropod_openai_api::ResponseObject::Response,
        created_at: chrono::Utc::now().timestamp(),
        output: vec![],
        usage: None,
        error: None,
        incomplete_details: None,
        model_response_properties: request
            .create_model_response_properties
            .model_response_properties
            .clone(),
        parallel_tool_calls: false,
        response_properties: request.response_properties.clone(),
        status: None,
        instructions: None,
    }
}

/// A local model artifact
pub enum ModelPath {
    /// GGUF model file.
    GGUF(std::path::PathBuf),
    /// TensorRT model file.
    TensorRT(std::path::PathBuf),
}

impl ModelPath {
    /// Get the underlying path.
    pub fn as_path(&self) -> &std::path::Path {
        match self {
            ModelPath::GGUF(path) => path,
            ModelPath::TensorRT(path) => path,
        }
    }

    /// Get the directory for the path.
    pub fn get_directory(&self) -> anyhow::Result<&std::path::Path> {
        match self {
            ModelPath::GGUF(path) => path.parent().ok_or_else(|| {
                anyhow::anyhow!(
                    "GGUF model path does not have a parent directory: {}",
                    path.display()
                )
            }),
            ModelPath::TensorRT(path) => Ok(path),
        }
    }
}

/// The source to load a model from.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ModelSource {
    /// A local file path.
    LocalFile(std::path::PathBuf),
    /// A Huggingface repository in the form huggingface.co/<repo>@<revision>:<quantization>.
    HuggingfaceRepo(sauropod_huggingface::HuggingfaceRepo),
}

impl std::str::FromStr for ModelSource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(huggingface_repo) = sauropod_huggingface::HuggingfaceRepo::from_str(s) {
            Ok(ModelSource::HuggingfaceRepo(huggingface_repo))
        } else {
            Ok(ModelSource::LocalFile(std::path::PathBuf::from(s)))
        }
    }
}

impl std::fmt::Display for ModelSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelSource::LocalFile(path) => write!(f, "{}", path.display()),
            ModelSource::HuggingfaceRepo(repo) => write!(f, "{repo}"),
        }
    }
}

/// A token in the model's vocabulary.
pub type Token = u32;
/// A sequence of tokens.
pub type TokenSequence = Vec<Token>;

/// An LLM model.
#[async_trait::async_trait]
pub trait LlmModel: Send + Sync {
    /// Generate text from a prompt and receive a stream of tokens.
    async fn generate_from_tokens(
        self: Arc<Self>,
        sampler_properties: SamplerProperties,
        tokens: TokenSequence,
    ) -> anyhow::Result<TokenStream>;

    /// Generate text from a prompt and receive a stream of text pieces.
    async fn generate_from_text(
        self: Arc<Self>,
        sampler_properties: SamplerProperties,
        text: String,
    ) -> anyhow::Result<PartStream>;

    /// Get the Jinja template for the model.
    fn get_model_chat_template(&self) -> &str;

    /// Get the model type.
    fn get_model_type(&self) -> sauropod_output_parser::ModelType;
}

/// Boxed stream of tokens.
pub type TokenStream = futures_core::stream::BoxStream<'static, anyhow::Result<Token>>;

/// Boxed stream of text parts.
pub type PartStream = futures_core::stream::BoxStream<'static, anyhow::Result<String>>;

/// Boxed stream of response events.
pub type ResponseStream = futures_core::stream::BoxStream<
    'static,
    anyhow::Result<sauropod_openai_api::ResponseStreamEvent>,
>;

#[cfg(test)]
mod tests {
    use sauropod_huggingface::HuggingfaceRepo;

    use super::*;

    #[test]
    fn test_model_source_from_str() {
        let local_model: ModelSource = "path/to/local/model".parse().unwrap();
        let hf_model_1: ModelSource = "huggingface.co/repo".parse().unwrap();
        let hf_model_2: ModelSource = "huggingface.co/repo@revision".parse().unwrap();
        let hf_model_3: ModelSource = "huggingface.co/repo@revision:quantization".parse().unwrap();
        let hf_model_4: ModelSource = "huggingface.co/repo:quantization".parse().unwrap();

        assert_eq!(
            local_model,
            ModelSource::LocalFile(std::path::PathBuf::from("path/to/local/model"))
        );

        assert_eq!(
            hf_model_1,
            ModelSource::HuggingfaceRepo(HuggingfaceRepo {
                repo: "repo".to_string(),
                revision: None,
                quantization: None
            })
        );
        assert_eq!(
            hf_model_2,
            ModelSource::HuggingfaceRepo(HuggingfaceRepo {
                repo: "repo".to_string(),
                revision: Some("revision".to_string()),
                quantization: None
            })
        );
        assert_eq!(
            hf_model_3,
            ModelSource::HuggingfaceRepo(HuggingfaceRepo {
                repo: "repo".to_string(),
                revision: Some("revision".to_string()),
                quantization: Some("quantization".to_string())
            })
        );
        assert_eq!(
            hf_model_4,
            ModelSource::HuggingfaceRepo(HuggingfaceRepo {
                repo: "repo".to_string(),
                revision: None,
                quantization: Some("quantization".to_string())
            })
        );
    }
}
