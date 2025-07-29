use axum::response::IntoResponse;
use axum::{extract::Request, middleware::Next, response::Response};

/// Authentication information.
#[derive(Debug, Clone)]
pub struct Authentication {
    pub user_info: sauropod_users::UserInfo,
}

impl Authentication {
    /// Get the user ID.
    ///
    /// # Returns
    /// The user ID if the client is authenticated, otherwise `None`.
    pub fn get_user_id(&self) -> sauropod_users::UserId {
        self.user_info.user_id
    }
}

/// Extension type for the user ID.
pub type UserIdExtension = axum::Extension<Authentication>;

/// Middleware to extract and validate a user ID from the request.
pub async fn auth_middleware(
    axum::extract::State(state): sauropod_global_state::AxumGlobalState,
    mut request: Request,
    next: Next,
) -> Response {
    let user_id = if let Some(auth_header) =
        request.headers().get(axum::http::header::AUTHORIZATION)
    {
        match auth_header.to_str() {
            Ok(auth_str) => {
                let Some(token) = auth_str
                    .strip_prefix("Bearer ")
                    .map(|s| s.trim().to_string())
                else {
                    tracing::info!(
                        "Client's Authorization header is not a Bearer token {auth_str}"
                    );
                    return HttpResponse::<()>::Unauthorized(
                        "Invalid token format - expected a bearer token".to_string(),
                    )
                    .into_response();
                };

                match &state.config.authentication {
                    sauropod_config::AuthenticationConfig::Database => {
                        // Check the database for the user ID associated with the token
                        if let Some(user_id) =
                            sauropod_users::get_user_info_by_api_key(state.database(), &token).await
                        {
                            Some(user_id)
                        } else {
                            tracing::info!("Invalid API key");
                            return HttpResponse::<()>::Unauthorized("Invalid API key".to_string())
                                .into_response();
                        }
                    }
                    sauropod_config::AuthenticationConfig::ApiKey { api_key }
                        if api_key == &token =>
                    {
                        // If the API key matches the hardcoded one, allow access
                        Some(sauropod_users::UserInfo {
                            user_id: 0, // The anonymous user ID is 0
                        })
                    }
                    _ => {
                        return HttpResponse::<()>::Unauthorized("Invalid API key".to_string())
                            .into_response();
                    }
                }
            }
            Err(e) => {
                tracing::info!("Invalid authorization header: {e}");
                return HttpResponse::<()>::Unauthorized(
                    "Invalid token format - expected a bearer token".to_string(),
                )
                .into_response();
            }
        }
    } else {
        None
    };

    if user_id.is_none()
        && !matches!(
            &state.config.authentication,
            sauropod_config::AuthenticationConfig::None,
        )
    {
        tracing::info!("Client is not authenticated");
        return HttpResponse::<()>::Unauthorized("Unauthorized".to_string()).into_response();
    }

    request.extensions_mut().insert(Authentication {
        user_info: user_id.unwrap_or(sauropod_users::UserInfo {
            user_id: 0, // The anonymous user ID is 0
        }),
    });

    // Continue processing the request
    next.run(request).await
}

/// An error message.
#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Error {
    /// The error message.
    pub error: String,
}

/// HTTP response.
pub enum HttpResponse<T> {
    /// HTTP 200
    Ok(T),
    /// HTTP 404
    NotFound(Option<String>),
    /// HTTP 400
    BadRequest(String),
    /// HTTP 401
    Unauthorized(String),
    /// HTTP 500
    InternalServerError(String),
}

impl<T> From<T> for HttpResponse<T> {
    fn from(x: T) -> Self {
        HttpResponse::Ok(x)
    }
}

impl<T> IntoResponse for HttpResponse<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        if let HttpResponse::Ok(x) = self {
            return axum::Json(x).into_response();
        }

        let status_code = match self {
            HttpResponse::Ok(_) => axum::http::StatusCode::OK,
            HttpResponse::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            HttpResponse::BadRequest(_) => axum::http::StatusCode::BAD_REQUEST,
            HttpResponse::Unauthorized(_) => axum::http::StatusCode::UNAUTHORIZED,
            HttpResponse::InternalServerError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = match self {
            HttpResponse::Ok(_) => "OK".to_string(),
            HttpResponse::NotFound(Some(message)) => message,
            HttpResponse::NotFound(None) => "Not found".to_string(),
            HttpResponse::BadRequest(message) => message,
            HttpResponse::Unauthorized(message) => message,
            HttpResponse::InternalServerError(message) => message,
        };

        let mut response: axum::http::Response<axum::body::Body> =
            axum::response::Json(Error { error: message }).into_response();
        *response.status_mut() = status_code;
        response
    }
}

impl<T> From<anyhow::Result<T>> for HttpResponse<T> {
    fn from(val: anyhow::Result<T>) -> Self {
        match val {
            Ok(data) => HttpResponse::Ok(data),
            Err(err) => {
                let message = format!("Internal server error: {err}");
                HttpResponse::InternalServerError(message)
            }
        }
    }
}
