//! Logging configuration crate.

mod in_memory;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::*;

pub use in_memory::{InMemoryLogBuffer, LogMessage};

/// Configuration for the logging system.
pub struct LoggingConfig {
    /// Whether to log verbosely.
    pub verbose: bool,
    /// Whether to use pretty printing.
    pub pretty_print: bool,
    /// An optional in-memory log buffer.
    pub in_memory_buffer: Option<std::sync::Arc<InMemoryLogBuffer>>,
}

/// A wrapper around the in-memory log buffer that implements the `tracing` layer trait.
struct InMemoryLayer {
    buffer: std::sync::Arc<InMemoryLogBuffer>,
}

impl tracing::field::Visit for LogMessage {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.fields.push((
            field.name(),
            serde_json::Value::String(format!("{:?}", value)),
        ));
    }

    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.fields.push((field.name(), serde_json::json!(value)));
    }
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.fields.push((field.name(), serde_json::json!(value)));
    }
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.fields.push((field.name(), serde_json::json!(value)));
    }
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.fields
            .push((field.name(), serde_json::Value::Bool(value)));
    }
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        self.fields
            .push((field.name(), serde_json::Value::String(value.to_string())));
    }
    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        self.fields.push((
            field.name(),
            serde_json::Value::String(format!("{:?}", value)),
        ));
    }
}

impl<S> tracing_subscriber::Layer<S> for InMemoryLayer
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut message = LogMessage {
            metadata: event.metadata(),
            timestamp: chrono::Utc::now(),
            fields: Vec::with_capacity(event.metadata().fields().len()),
        };
        event.record(&mut message);
        self.buffer.append(message);
    }
}

/// Set up the logging backend.
pub fn initialize(config: LoggingConfig) {
    let fmt_layer = if config.pretty_print {
        tracing_subscriber::fmt::Layer::default()
            .pretty()
            .with_writer(std::io::stderr)
            .boxed()
    } else {
        tracing_subscriber::fmt::Layer::default()
            .with_writer(std::io::stderr)
            .boxed()
    };

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(
            config
                .in_memory_buffer
                .map(|buffer| InMemoryLayer { buffer }),
        )
        .with(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(
                    if config.verbose {
                        LevelFilter::INFO
                    } else {
                        LevelFilter::WARN
                    }
                    .into(),
                )
                .with_env_var(format!("{}_LOG", sauropod_config::ENV_VAR_PREFIX))
                .from_env_lossy(),
        )
        .init();
}
