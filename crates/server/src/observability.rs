//! Observability-related server code.

use std::collections::HashMap;

use sauropod_schemas::observability::{LogLevel, LogMessage, LogResponse};

pub(crate) struct Observability {
    pub(crate) log_buffer: std::sync::Arc<sauropod_logging::InMemoryLogBuffer>,
}

/// Convert a `tracing` log level to a `sauropod_schemas` log level.
fn tracing_log_level_to_log_level(level: &tracing::Level) -> LogLevel {
    match *level {
        tracing::Level::ERROR => LogLevel::Error,
        tracing::Level::WARN => LogLevel::Warning,
        tracing::Level::INFO => LogLevel::Info,
        tracing::Level::DEBUG => LogLevel::Debug,
        tracing::Level::TRACE => LogLevel::Debug,
    }
}

impl Observability {
    pub(crate) fn get_observability_logs(&self) -> LogResponse {
        let mut logs: Vec<LogMessage> = Vec::with_capacity(self.log_buffer.capacity);
        self.log_buffer.for_each(|log_message| {
            logs.push(LogMessage {
                level: tracing_log_level_to_log_level(log_message.metadata.level()),
                module: log_message.metadata.module_path().unwrap_or("").to_string(),
                line: log_message.metadata.line(),
                timestamp_ms: log_message.timestamp.timestamp_millis(),
                fields: HashMap::from_iter(
                    log_message
                        .fields
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.clone())),
                ),
            });
        });
        LogResponse(logs)
    }
}
