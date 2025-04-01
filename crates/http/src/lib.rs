//! HTTP.

use axum::response::IntoResponse;
use axum::response::Json;

mod generated;
pub use generated::{API_PREFIX, ServerInterface, register_routes};

/// HTTP response.
pub enum HttpResponse<T> {
    /// HTTP 200
    Ok(T),
    /// HTTP 404
    NotFound(Option<String>),
    /// HTTP 400
    BadRequest(String),
}

impl<T> From<T> for HttpResponse<T> {
    fn from(x: T) -> Self {
        HttpResponse::Ok(x)
    }
}

impl<T> axum::response::IntoResponse for HttpResponse<T>
where
    Json<T>: axum::response::IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        if let HttpResponse::Ok(x) = self {
            return Json(x).into_response();
        }

        let status_code = match self {
            HttpResponse::Ok(_) => axum::http::StatusCode::OK,
            HttpResponse::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            HttpResponse::BadRequest(_) => axum::http::StatusCode::BAD_REQUEST,
        };
        let message = match self {
            HttpResponse::Ok(_) => "OK".to_string(),
            HttpResponse::NotFound(Some(message)) => message,
            HttpResponse::NotFound(None) => "Not found".to_string(),
            HttpResponse::BadRequest(message) => message,
        };

        let mut response =
            axum::response::Json(serde_json::json!({ "error": message })).into_response();
        *response.status_mut() = status_code;
        response
    }
}

/// 404 Not Found response.
pub async fn not_found() -> impl IntoResponse {
    let mut response =
        axum::response::Json(serde_json::json!({ "error": "Not found" })).into_response();
    *response.status_mut() = axum::http::StatusCode::NOT_FOUND;
    response
}

pub(crate) fn create_response_from_result<T>(
    data: anyhow::Result<HttpResponse<T>>,
) -> impl axum::response::IntoResponse
where
    HttpResponse<T>: axum::response::IntoResponse,
{
    match data {
        Ok(data) => data.into_response(),
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
