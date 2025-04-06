//! HTTP.

use axum::response::IntoResponse;
use axum::response::Json;

mod generated;
pub use generated::{API_PREFIX, ServerInterface, register_routes};

mod ui_routes;
pub use ui_routes::make_ui_routes;

/// HTTP response.
pub enum HttpResponse<T> {
    /// HTTP 200
    Ok(T),
    /// HTTP 404
    NotFound(Option<String>),
    /// HTTP 400
    BadRequest(String),
    /// HTTP 500
    InternalServerError(String),
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
            HttpResponse::InternalServerError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match self {
            HttpResponse::Ok(_) => "OK".to_string(),
            HttpResponse::NotFound(Some(message)) => message,
            HttpResponse::NotFound(None) => "Not found".to_string(),
            HttpResponse::BadRequest(message) => message,
            HttpResponse::InternalServerError(message) => message,
        };

        let mut response: axum::http::Response<axum::body::Body> =
            axum::response::Json(sauropod_schemas::Error { error: message }).into_response();
        *response.status_mut() = status_code;
        response
    }
}

pub(crate) fn create_response_from_result<T>(
    data: anyhow::Result<HttpResponse<T>>,
) -> impl axum::response::IntoResponse
where
    HttpResponse<T>: axum::response::IntoResponse,
{
    match data {
        Ok(data) => data.into_response(),
        Err(_) => {
            HttpResponse::InternalServerError("Internal server error".to_string()).into_response()
        }
    }
}
