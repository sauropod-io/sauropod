//! Model Context Protocol interface.
use std::sync::Arc;
use std::{borrow::Cow, pin::Pin};

use anyhow::Context;
use rmcp::{
    ServiceExt,
    model::CallToolRequestParam,
    service::{DynService, RoleClient, RunningService},
    transport::{SseTransport, TokioChildProcess},
};
use tracing::Instrument;

/// A tool provided via Model Context Protocol.
pub struct McpTool {
    /// The MCP server that provides this tool.
    provider_name: String,
    /// The MCP provider.
    mcp: Arc<ModelContextProtocol>,
    /// The tool description.
    description: sauropod_tool_spec::ToolDefinition,
}

impl McpTool {
    pub fn new(
        mcp: Arc<ModelContextProtocol>,
        peer_info: &rmcp::model::InitializeResult,
        tool: rmcp::model::Tool,
    ) -> Self {
        let provider = format!("mcp:{}", peer_info.server_info.name);
        Self {
            provider_name: peer_info.server_info.name.clone(),
            mcp,
            description: sauropod_tool_spec::ToolDefinition {
                id: format!("{}:{}", provider, tool.name),
                name: tool.name.to_string(),
                provider,
                description: tool.description.to_string(),
                input_schema: tool.schema_as_json_value(),
            },
        }
    }
}

impl sauropod_tool_spec::Tool for McpTool {
    fn get_id(&self) -> &str {
        self.description.id.as_str()
    }

    fn get_name(&self) -> &str {
        self.description.name.as_str()
    }

    fn get_definition(&self) -> sauropod_tool_spec::ToolDefinition {
        self.description.clone()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<String>> + Send>> {
        let mcp = self.mcp.clone();
        let provider_name = self.provider_name.clone();
        let tool_name = self.description.name.clone();
        Box::pin(async { mcp.call_tool(provider_name, tool_name, input).await })
    }
}

/// Manages multiple MCP connections.
pub struct ModelContextProtocol {
    /// List of MCP servers.
    mcp_servers: Vec<RunningService<RoleClient, Box<dyn DynService<RoleClient>>>>,
}

impl ModelContextProtocol {
    /// Creates a new `ModelContextProtocol` instance.
    pub async fn new(config: &sauropod_config::Config) -> anyhow::Result<Arc<Self>> {
        let mut mcp_servers = Vec::with_capacity(config.mcp_servers.len());

        for server in &config.mcp_servers {
            match server {
                sauropod_config::McpServer::Http { url } => {
                    let transport = SseTransport::start(url.as_str()).await?;
                    let service = ()
                        .into_dyn()
                        .serve(transport)
                        .await
                        .with_context(|| format!("When connecting to MCP server at {url}"))?;
                    mcp_servers.push(service);
                }
                sauropod_config::McpServer::Process { command } => {
                    let command_executable = command.first().ok_or_else(|| {
                        anyhow::anyhow!("MCP server command must have at least one argument")
                    })?;

                    let child = TokioChildProcess::new(
                        tokio::process::Command::new(command_executable).args(&command[1..]),
                    )?;
                    let service = ().into_dyn().serve(child).await.with_context(|| {
                        format!("When spawning MCP server with via command: {command:?}")
                    })?;
                    mcp_servers.push(service);
                }
            }
        }

        for server in &mcp_servers {
            let peer_info = server.peer_info();
            if peer_info.capabilities.tools.is_none() {
                tracing::warn!(
                    "MCP server {} does not support tools",
                    peer_info.server_info.name
                );
            }
        }

        Ok(Arc::new(Self { mcp_servers }))
    }

    /// Internal function to call a tool on the MCP server.
    pub(crate) async fn call_tool(
        self: Arc<Self>,
        peer_name: String,
        tool_name: String,
        input: serde_json::Value,
    ) -> anyhow::Result<String> {
        let Some(mcp_server) = self
            .mcp_servers
            .iter()
            .find(|x| x.peer_info().server_info.name == peer_name)
        else {
            anyhow::bail!("MCP server with name {peer_name} not found")
        };
        let Some(input) = input.as_object() else {
            anyhow::bail!("Input must be a JSON object")
        };

        let result = mcp_server
            .call_tool(CallToolRequestParam {
                name: Cow::Owned(tool_name.to_string()),
                arguments: Some(input.clone()),
            })
            .await?;

        if result.is_error.unwrap_or(false) {
            tracing::error!(
                "Error calling tool {} on MCP server {}: {:#?}",
                tool_name,
                peer_name,
                &result.content
            );
        }

        if result.content.is_empty() {
            Ok("".to_string())
        } else if result.content.len() == 1 {
            match result.content.into_iter().next().unwrap().raw {
                rmcp::model::RawContent::Image(_) => {
                    anyhow::bail!(
                        "Right now Sauropod only handles text from MCP tools, but received image"
                    );
                }
                rmcp::model::RawContent::Text(text) => {
                    return Ok(text.text);
                }
                rmcp::model::RawContent::Resource(_) => {
                    anyhow::bail!(
                        "Right now Sauropod only handles text from MCP tools, but received resource"
                    );
                }
            }
        } else {
            anyhow::bail!(
                "Right now Sauropod only handles single text results from MCP tools, but received multiple results: {:#?}",
                result.content
            );
        }
    }

    /// Lists all tools available on the MCP servers.
    pub async fn list_all_tools(
        self: Arc<Self>,
    ) -> anyhow::Result<Vec<Arc<dyn sauropod_tool_spec::Tool>>> {
        let futures: Vec<_> = self
            .mcp_servers
            .iter()
            .map(|server| async {
                let peer_info = server.peer_info();
                let tools = server.list_all_tools().await;

                // Convert the MCP library tool representation to our internal representation
                tools.map(|tools_list| {
                    tools_list
                        .into_iter()
                        .map(|x| {
                            Arc::new(McpTool::new(Arc::clone(&self), peer_info, x))
                                as Arc<dyn sauropod_tool_spec::Tool>
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        let results = futures::future::join_all(futures)
            .instrument(tracing::info_span!("MCP list_all_tools"))
            .await;

        let mut tools: Vec<Arc<dyn sauropod_tool_spec::Tool>> = Vec::new();
        for result in results {
            match result {
                Ok(tool) => tools.extend(tool),
                Err(err) => return Err(anyhow::anyhow!("Error fetching tools: {}", err)),
            }
        }

        Ok(tools)
    }
}
