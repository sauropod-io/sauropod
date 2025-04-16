use sauropod_tool_spec::ConcreteTool;

/// Tool to retrieve files and send HTTP requests.
pub struct NotifyTool;

/// Input to the HTTP request tool.
#[derive(serde::Deserialize, schemars::JsonSchema, Debug)]
pub struct NotifyToolInput {
    /// The message to tell the user.
    _message: String,
}

impl ConcreteTool for NotifyTool {
    type Input = NotifyToolInput;

    fn get_id(&self) -> &str {
        "builtin:notify"
    }

    fn get_name(&self) -> &str {
        "notify"
    }

    fn get_description(&self) -> &str {
        "Send a notification to the user."
    }

    async fn run(self: std::sync::Arc<Self>, _input: Self::Input) -> anyhow::Result<String> {
        tracing::error!("TODO implement notify tool");
        Ok("Notification sent".to_string())
    }
}
