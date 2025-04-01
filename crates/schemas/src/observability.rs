//! Observability-related schemas.

/// The log level.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum LogLevel {
    /// Debug log level.
    Debug,
    /// Info log level.
    Info,
    /// Warning log level.
    Warning,
    /// Error log level.
    Error,
}

/// A logged message.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LogMessage {
    /// The module where the log message was emitted.
    pub module: String,
    /// The number of seconds since epoch.
    pub timestamp_s: u64,
    /// Structured logging fields.
    pub fields: std::collections::HashMap<String, serde_json::Value>,
    /// The line number where the log message was emitted.
    pub line: Option<u32>,
    /// The log level.
    pub level: LogLevel,
}

/// A list of logged messages.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LogResponse(pub Vec<LogMessage>);
