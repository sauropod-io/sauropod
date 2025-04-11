//! UI serving code.

use axum::extract::OriginalUri;
use axum::response::IntoResponse as _;

// This file is contains the `FILES` variable.
include!(concat!(env!("OUT_DIR"), "/generated_ui_routes.rs"));

/// The prefixes to direct to UI files.
const UI_PREFIXES: &[&str] = &["/task/", "/workflow/", "/logs"];

/// Routes handled in the UI need to be served to `index.html`
fn is_ui_path(path: &str) -> bool {
    UI_PREFIXES.iter().any(|prefix| path.starts_with(prefix))
}

/// Make the UI routes for the server.
pub fn make_ui_routes() -> axum::Router<()> {
    let index_tuple = FILES
        .iter()
        .find(|(path, _, _)| path == &"/")
        .expect("/ not found in FILES");

    let router = axum::Router::new()
        .without_v07_checks()
        .fallback(async |path: OriginalUri| {
            if is_ui_path(path.path()) {
                let (_, mime_type, content) = index_tuple;
                let mut content = content.into_response();
                content
                    .headers_mut()
                    .insert(axum::http::header::CONTENT_TYPE, mime_type.parse().unwrap());
                content
            } else {
                let mut response =
                    axum::response::Json(serde_json::json!({ "error": "Not found" }))
                        .into_response();
                *response.status_mut() = axum::http::StatusCode::NOT_FOUND;
                response
            }
        });

    FILES
        .iter()
        .fold(router, |router, (path, mime_type, content)| {
            router.route(
                path,
                axum::routing::get(move || {
                    let mut content = content.into_response();
                    content
                        .headers_mut()
                        .insert(axum::http::header::CONTENT_TYPE, mime_type.parse().unwrap());
                    async move { content }
                }),
            )
        })
}
