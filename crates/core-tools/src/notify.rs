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

    fn get_name(&self) -> String {
        "notify".to_string()
    }

    fn get_description(&self) -> String {
        "Send a notification to the user.".to_string()
    }

    async fn run(self: std::sync::Arc<Self>, _input: Self::Input) -> anyhow::Result<String> {
        tracing::error!("TODO implement notify tool");
        Ok("Notification sent".to_string())
    }
}
