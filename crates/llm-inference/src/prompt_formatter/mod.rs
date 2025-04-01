use crate::openai_api::CompletionRequest;

pub mod default;
pub mod gemma;

/// Prepare a completion request for the given `model_type`.
///
/// This function mainly exists to ensure tool calling and system prompt compatibility.
pub fn prepare_completion_request(
    model: sauropod_config::ModelConfig,
    context: crate::LlmContext,
) -> anyhow::Result<CompletionRequest> {
    match model.model_type {
        // Gemma 3 and Phi 4 seem to work well with the same function calling format.
        sauropod_config::ModelType::Gemma3 | sauropod_config::ModelType::Phi4 => {
            gemma::prepare_completion_request(model, context)
        }
        sauropod_config::ModelType::Default => default::prepare_completion_request(model, context),
    }
}
