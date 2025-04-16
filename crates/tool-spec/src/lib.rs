//! Sauropod tool specification.

use std::{future::Future, pin::Pin, sync::Arc};

use anyhow::Context;
use tracing::Instrument;

pub use sauropod_schemas::ToolDefinition;

const BUILTIN_PROVIDER: &str = "builtin";

/// A tool which can be exposed to LLMs.
pub trait Tool: Send + Sync {
    /// Get the ID of the tool.
    fn get_id(&self) -> &str;

    /// Get the name of the tool.
    fn get_name(&self) -> &str;

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

    /// Get the ID of the tool.
    fn get_id(&self) -> &str;

    /// The name of the tool. Must match the regex `^[a-zA-Z0-9_-]{1,64}$`.
    fn get_name(&self) -> &str;

    /// A detailed plaintext description of what the tool does, when it should be used, and how it behaves.
    fn get_description(&self) -> &str;

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
    fn get_id(&self) -> &str {
        <Self as ConcreteTool>::get_id(self)
    }

    fn get_name(&self) -> &str {
        <Self as ConcreteTool>::get_name(self)
    }

    fn get_definition(&self) -> ToolDefinition {
        let schema_generator = schemars::SchemaGenerator::new({
            let mut schema_settings = schemars::generate::SchemaSettings::draft2020_12();
            schema_settings.inline_subschemas = true;
            schema_settings
        });
        let input_schema = schema_generator
            .into_root_schema_for::<<Self as ConcreteTool>::Input>()
            .to_value();

        let name = <Self as ConcreteTool>::get_name(self);
        ToolDefinition {
            id: format!("{BUILTIN_PROVIDER}:{name}"),
            name: name.to_string(),
            provider: BUILTIN_PROVIDER.to_string(),
            description: self.get_description().to_string(),
            input_schema,
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send>> {
        Box::pin(async {
            let tool_name = self.get_name().to_string();
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
