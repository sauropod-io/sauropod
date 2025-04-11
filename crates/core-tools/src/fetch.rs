use anyhow::Context;

use sauropod_tool_spec::ConcreteTool;

/// HTTP method.
#[derive(serde::Deserialize, schemars::JsonSchema, Debug, Default)]
pub enum HttpMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
}

/// Tool to retrieve files and send HTTP requests.
pub struct FetchTool;

/// Input to the HTTP request tool.
#[derive(serde::Deserialize, schemars::JsonSchema, Debug)]
pub struct FetchRequestToolInput {
    /// The URL to request.
    url: String,
    /// The method to use. In most cases this should be `GET`.
    #[serde(default)]
    method: HttpMethod,
}

impl ConcreteTool for FetchTool {
    type Input = FetchRequestToolInput;

    fn get_name(&self) -> &str {
        "fetch"
    }

    fn get_description(&self) -> &str {
        "Send an HTTP request to the provided URL to upload or download data. This tool will return the response code and the body."
    }

    async fn run(self: std::sync::Arc<Self>, input: Self::Input) -> anyhow::Result<String> {
        tracing::debug!("Sending request to {}", &input.url);

        let response = match input.method {
            HttpMethod::GET => reqwest::get(&input.url).await,
            HttpMethod::POST => reqwest::Client::new().post(&input.url).send().await,
            HttpMethod::PUT => reqwest::Client::new().put(&input.url).send().await,
            HttpMethod::DELETE => reqwest::Client::new().delete(&input.url).send().await,
        }?;

        response
            .text()
            .await
            .context("Failed to read the response body")
    }
}
