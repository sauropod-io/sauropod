//! Link generation utilities.

/// The URL to the Sauropod documentation site.
pub const DOCS_URL: &str = "https://sauropod.io/docs/";

/// Make a link to the Sauropod documentation site.
pub fn doc_link(path: &str) -> String {
    format!("{DOCS_URL}{path}")
}
