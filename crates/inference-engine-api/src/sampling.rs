/// Properties for the sampler used in the model.
#[derive(Clone)]
pub struct SamplerProperties {
    /// The top K tokens to sample from.
    pub top_k: Option<i64>,
    /// The minimum probability for sampling.
    pub min_p: Option<i64>,
    /// The temperature for sampling.
    pub temperature: f64,
    /// The top probability for nucleus sampling.
    pub top_p: Option<f64>,
    /// The maximum number of tokens to predict.
    pub max_predict: usize,
    /// Repetition penalty for sampling.
    pub repetition_penalty: Option<f64>,
}

impl SamplerProperties {
    pub fn new(
        response: &sauropod_openai_api::Response,
        model_config: &sauropod_config::ModelConfig,
    ) -> Self {
        Self {
            top_k: model_config.top_k,
            min_p: model_config.min_p,
            temperature: response
                .model_response_properties
                .temperature
                .unwrap_or(0.8),
            top_p: response.model_response_properties.top_p,
            max_predict: response
                .response_properties
                .max_output_tokens
                .unwrap_or(4096) as usize,
            repetition_penalty: None,
        }
    }
}
