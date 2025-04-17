use axum::{extract::Request, middleware::Next, response::Response};

/// The ID of a user derived from their authentication credentials.
#[derive(Debug, Clone, Copy)]
pub struct UserId(pub sauropod_database::DatabaseId);

/// Extension type for the user ID.
pub type UserIdExtension = axum::Extension<UserId>;

/// Middleware to extract and validate a user ID from the request.
///
/// For now, this always sets the user ID to 0, regardless of the authorization header.
pub async fn auth_middleware(mut request: Request, next: Next) -> Response {
    // For now, always use user ID 0, ignoring any authentication headers
    let user_id = UserId(0);

    // Add the user ID to the request extensions
    request.extensions_mut().insert(user_id);

    // Continue processing the request
    next.run(request).await
}
