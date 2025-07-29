use axum::extract::State;
use axum::response::IntoResponse;

use sauropod_openai_api::{CreateResponse, Response};

#[utoipa::path(
    post,
    path = "/v1/responses",
    tag = "Responses",
    request_body = CreateResponse,
    responses(
        (status = 200, description = "Response created", body = Response),
        (status = 500, description = "Error occured", body = sauropod_inference_http::Error)
    )
)]
pub async fn create_response(
    State(loaded_models): sauropod_global_state::AxumGlobalState,
    axum::Json(request): axum::Json<serde_json::Value>,
) -> axum::response::Response {
    let request = match serde_json::from_value::<CreateResponse>(request.clone()) {
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
    match crate::create_response_impl(loaded_models, request).await {
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

#[utoipa::path(
    get,
    path = "/v1/responses/{response_id}",
    description = "Retrieves a model response with the given ID",
    tag = "Responses",
    params(
        ("response_id" = String, Path, description = "The ID of the response to retrieve")
    ),
    responses(
        (status = 200, description = "Response found", body = Response),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Error occured", body = sauropod_inference_http::Error)
    )
)]
pub async fn get_response(
    response_id: axum::extract::Path<String>,
    State(_loaded_models): sauropod_global_state::AxumGlobalState,
) -> axum::response::Response {
    let responses = crate::in_memory_responses();
    let responses_guard = responses.read().await;

    match responses_guard.get(&response_id.0) {
        Some(response_data) => axum::Json(response_data.response.clone()).into_response(),
        None => (
            axum::http::StatusCode::NOT_FOUND,
            axum::Json(sauropod_inference_http::Error {
                error: format!("Response with ID '{}' not found", response_id.0),
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/responses/{response_id}",
    description = "Deletes a model response with the given ID",
    tag = "Responses",
    params(
        ("response_id" = String, Path, description = "The ID of the response to delete")
    ),
    responses(
        (status = 200, description = "OK"),
        (status = 404, description = "Not Found"),
        (status = 500, description = "Error occured", body = sauropod_inference_http::Error)
    )
)]
pub async fn delete_response(
    response_id: axum::extract::Path<String>,
    State(_loaded_models): sauropod_global_state::AxumGlobalState,
) -> axum::response::Response {
    let responses = crate::in_memory_responses();
    let mut responses_guard = responses.write().await;

    match responses_guard.remove(&response_id.0) {
        Some(_) => axum::http::StatusCode::OK.into_response(),
        None => (
            axum::http::StatusCode::NOT_FOUND,
            axum::Json(sauropod_inference_http::Error {
                error: format!("Response with ID '{}' not found", response_id.0),
            }),
        )
            .into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/v1/models",
    description = "Lists the currently available models, and provides basic information about each one such as the owner and availability.",
    tag = "Models",
    responses(
        (status = 200, description = "OK", body = sauropod_openai_api::ListModelsResponse),
    )
)]
pub async fn get_models(
    State(loaded_models): sauropod_global_state::AxumGlobalState,
) -> axum::response::Response {
    let models = loaded_models.get_all_models().await;
    let response = sauropod_openai_api::ListModelsResponse {
        data: models
            .keys()
            .map(|name| sauropod_openai_api::Model {
                created: 0,
                id: name.to_string(),
                object: sauropod_openai_api::ModelObject::Model,
                owned_by: "".to_string(),
            })
            .collect(),
        object: sauropod_openai_api::ListModelsResponseObject::List,
    };
    axum::Json(response).into_response()
}
