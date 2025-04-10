//! Model Context Protocol interface.
use rmcp::model::Tool;
use rmcp::{
    ServiceExt,
    service::{DynService, RoleClient, RunningService},
    transport::{SseTransport, TokioChildProcess},
};
use tracing::Instrument;

/// Manages multiple MCP connections.
pub struct ModelContextProtocol {
    /// List of MCP servers.
    mcp_servers: Vec<RunningService<RoleClient, Box<dyn DynService<RoleClient>>>>,
}

impl ModelContextProtocol {
    /// Creates a new `ModelContextProtocol` instance.
    pub async fn new(config: &sauropod_config::Config) -> anyhow::Result<Self> {
        let mut mcp_servers = Vec::with_capacity(config.mcp_servers.len());

        for server in &config.mcp_servers {
            match server {
                sauropod_config::McpServer::Http { url } => {
                    let transport = SseTransport::start(url.as_str()).await?;
                    let service = ().into_dyn().serve(transport).await?;
                    mcp_servers.push(service);
                }
                sauropod_config::McpServer::Process { command } => {
                    let command_executable = command.first().ok_or_else(|| {
                        anyhow::anyhow!("MCP server command must have at least one argument")
                    })?;

                    let child = TokioChildProcess::new(
                        tokio::process::Command::new(command_executable).args(&command[1..]),
                    )?;
                    let service = ().into_dyn().serve(child).await?;
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

            dbg!(server.list_all_tools().await?);
        }

        Ok(Self { mcp_servers })
    }

    pub async fn list_all_tools(&self) -> anyhow::Result<Vec<Tool>> {
        let futures: Vec<_> = self
            .mcp_servers
            .iter()
            .map(|server| server.list_all_tools())
            .collect();

        let results = futures::future::join_all(futures)
            .instrument(tracing::info_span!("MCP list_all_tools"))
            .await;

        let mut tools = Vec::new();
        for result in results {
            match result {
                Ok(tool) => tools.extend(tool),
                Err(err) => return Err(anyhow::anyhow!("Error fetching tools: {}", err)),
            }
        }

        Ok(tools)
    }
}
