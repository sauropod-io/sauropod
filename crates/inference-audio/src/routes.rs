use axum::extract::State;
use axum::response::IntoResponse;

use sauropod_openai_api::{CreateSpeechRequest, Response};

#[utoipa::path(
    post,
    path = "/v1/audio/speech",
    tag = "Audio",
    request_body = CreateSpeechRequest,
    responses(
        (status = 200, description = "Response created", body = Response),
        (status = 500, description = "Error occured", body = sauropod_inference_http::Error)
    )
)]
pub async fn create_speech(
    State(loaded_models): sauropod_global_state::AxumGlobalState,
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::response::Response {
    let request = match serde_json::from_value::<CreateSpeechRequest>(request.clone()) {
        Ok(request) => request,
        Err(e) => {
            tracing::error!(
                "Failed to parse request: {e}\n{}",
                serde_json::to_string_pretty(&request).unwrap()
            );
            return (
                axum::http::StatusCode::BAD_REQUEST,
                axum::Json(sauropod_inference_http::Error {
                    error: format!("Failed to parse request: {e}"),
                }),
            )
                .into_response();
        }
    };
    match crate::create_speech_impl(loaded_models, request).await {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Failed to create response: {e:#?}");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(sauropod_inference_http::Error {
                    error: format!("Internal server error: {e}"),
                }),
            )
                .into_response()
        }
    }
}
