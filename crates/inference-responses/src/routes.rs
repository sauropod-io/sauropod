use axum::extract::State;
use axum::response::IntoResponse;

use sauropod_inference_http::UserAuthenticationExtension;
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
    axum::Extension(authentication): UserAuthenticationExtension,
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
    match crate::create_response_impl(loaded_models, request, authentication).await {
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
    axum::Extension(authentication): UserAuthenticationExtension,
    State(global_state): sauropod_global_state::AxumGlobalState,
) -> axum::response::Response {
    let user_id = authentication.get_user_id();
    let result = sqlx::query!(
        "SELECT response_output FROM response WHERE response_id = ?1 AND user_id = ?2",
        response_id.0,
        user_id
    )
    .fetch_one(global_state.database())
    .await;

    match result {
        Ok(row) => {
            let mut response = row.response_output.into_response();
            response.headers_mut().insert(
                axum::http::header::CONTENT_TYPE,
                axum::http::HeaderValue::from_static("application/json"),
            );
            response
        }
        Err(sqlx::Error::RowNotFound) => {
            sauropod_inference_http::HttpResponse::<()>::NotFound(None).into_response()
        }
        Err(e) => {
            tracing::error!("Error fetching response: {e}");
            sauropod_inference_http::HttpResponse::<()>::InternalServerError(
                "Error occured querying database".to_string(),
            )
            .into_response()
        }
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
    axum::Extension(authentication): UserAuthenticationExtension,
    State(global_state): sauropod_global_state::AxumGlobalState,
) -> axum::response::Response {
    let user_id = authentication.get_user_id();
    let result = sqlx::query!(
        "DELETE FROM response WHERE response_id = ?1 AND user_id = ?2",
        response_id.0,
        user_id
    )
    .execute(global_state.database())
    .await;

    let response = match result {
        Ok(user) if user.rows_affected() > 0 => ().into_response(),
        Err(sqlx::Error::RowNotFound) | Ok(_) => {
            sauropod_inference_http::HttpResponse::<()>::NotFound(None).into_response()
        }
        Err(e) => {
            tracing::error!("Error deleting response: {e}");
            sauropod_inference_http::HttpResponse::<()>::InternalServerError(
                "Error occured querying database".to_string(),
            )
            .into_response()
        }
    };
    response.into_response()
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
