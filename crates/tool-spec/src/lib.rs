//! Sauropod tool specification.

use std::{future::Future, pin::Pin, sync::Arc};

use anyhow::Context;
use tracing::Instrument;

pub use sauropod_schemas::ToolDefinition;

/// A tool which can be exposed to LLMs.
pub trait Tool: Send + Sync {
    /// Get the definition of the tool.
    fn get_definition(&self) -> ToolDefinition;

    /// Run the tool.
    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send>>;
}

/// A tool trait for static dispatch - use `Tool` instead for invocations.
pub trait ConcreteTool {
    /// The input to the tool.
    type Input;

    /// The name of the tool. Must match the regex `^[a-zA-Z0-9_-]{1,64}$`.
    fn get_name(&self) -> String;

    /// A detailed plaintext description of what the tool does, when it should be used, and how it behaves.
    fn get_description(&self) -> String;

    /// Run the tool.
    fn run(
        self: Arc<Self>,
        input: Self::Input,
    ) -> impl std::future::Future<Output = anyhow::Result<String>> + Send;
}

// Blanket implementation for Rust tools.
impl<T: ConcreteTool + Send + Sync + 'static> Tool for T
where
    <Self as ConcreteTool>::Input: for<'a> serde::Deserialize<'a> + schemars::JsonSchema,
{
    fn get_definition(&self) -> ToolDefinition {
        let schema_generator = schemars::SchemaGenerator::new({
            let mut schema_settings = schemars::generate::SchemaSettings::draft2020_12();
            schema_settings.inline_subschemas = true;
            schema_settings
        });
        let input_schema = schema_generator
            .into_root_schema_for::<<Self as ConcreteTool>::Input>()
            .to_value();

        ToolDefinition {
            name: self.get_name(),
            description: self.get_description(),
            input_schema,
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send>> {
        Box::pin(async {
            let tool_name = self.get_name();
            let input: <Self as ConcreteTool>::Input =
                serde_json::from_value(input).with_context(|| {
                    format!("Converting the input JSON to {tool_name}'s input struct")
                })?;

            let result = self
                .run(input)
                .instrument(tracing::info_span!("Running tool", tool_name = &tool_name))
                .await;
            if let Err(err) = &result {
                tracing::error!("Error running the {tool_name} tool: {err}");
            }
            result
        })
    }
}
