//! Global state.

/// A user ID.
pub type UserId = i64;

/// A user ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserInfo {
    /// The user ID.
    pub user_id: i64,
}

/// Get a user by their API key.
pub async fn get_user_info_by_api_key(
    database: &sauropod_database::Database,
    api_key: &str,
) -> Option<UserInfo> {
    sqlx::query_as!(
        UserInfo,
        r#"SELECT user_id FROM api_keys WHERE key = $1"#,
        api_key
    )
    .fetch_optional(database)
    .await
    .ok()
    .flatten()
}
