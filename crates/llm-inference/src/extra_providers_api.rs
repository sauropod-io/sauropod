//! Anthropic API interface.

/// Response from the models endpoint.
#[derive(Debug, serde::Deserialize)]
#[allow(unused)]
pub struct AnthropicModelData {
    pub id: String,
    pub display_name: String,
    pub created_at: String,
    pub r#type: String,
}

/// Response from the Anthropic models endpoint.
#[derive(Debug, serde::Deserialize)]
pub struct AnthropicModelsResponse {
    #[allow(unused)]
    pub data: Vec<AnthropicModelData>,
}

/// Get models from Anthropic API.
pub async fn get_anthropic_models(
    client: &reqwest::Client,
    url: &str,
) -> anyhow::Result<super::openai_api::ModelsResponse> {
    let response = client
        .get(format!("{}/v1/models", url))
        .send()
        .await?
        .error_for_status()?;
    let models_response: AnthropicModelsResponse = response.json().await?;

    Ok(super::openai_api::ModelsResponse {
        object: "list".to_string(),
        data: Some(
            models_response
                .data
                .into_iter()
                .map(|model_data| super::openai_api::ModelData {
                    id: model_data.id,
                    object: model_data.r#type,
                    created: 0,
                    owned_by: model_data.display_name,
                })
                .collect(),
        ),
    })
}
