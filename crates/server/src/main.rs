use std::{collections::HashMap, path::PathBuf};

use tracing::Instrument;

const CONFIG_FLAG: &str = "config";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = sauropod_server::add_config_flags(
        clap::Command::new("sauropod")
            .version(env!("CARGO_PKG_VERSION"))
            .about(concat!(
                env!("CARGO_PKG_DESCRIPTION"),
                "\n\nSee ",
                env!("CARGO_PKG_HOMEPAGE"),
                "/docs for more information."
            ))
            .arg(
                clap::Arg::new(CONFIG_FLAG)
                    .long(CONFIG_FLAG)
                    .short('c')
                    .help("Path to the configuration file.")
                    .value_parser(clap::value_parser!(PathBuf)),
            )
            .arg(
                clap::Arg::new("mcp-servers")
                    .long("mcp-servers")
                    .short('m')
                    .value_name("MCP_SERVERS")
                    .env("SAUROPOD_MCP_SERVERS")
                    .help("Comma seperated list of MCP server URLs."),
            ),
    );
    let cli_matches = command.get_matches();
    let mut cli_config_overrides = sauropod_server::clap_to_config_source(&cli_matches);

    // Add some custom handling for the MCP servers flag
    if let Some(mcp_servers) = cli_matches.get_one::<String>("mcp-servers") {
        let mcp_servers = mcp_servers
            .split(',')
            .map(|x| {
                let mut map = HashMap::with_capacity(1);
                map.insert("url".to_string(), x.to_string());
                map
            })
            .collect::<Vec<_>>();
        cli_config_overrides.add_value(
            "mcp_servers".to_string(),
            config::Value::new(None, mcp_servers),
        )?;
    }

    let config = if let Some(config_path) = cli_matches.get_one::<PathBuf>(CONFIG_FLAG).cloned() {
        sauropod_config::Config::load_from_file(config_path, cli_config_overrides)?
    } else {
        sauropod_config::Config::load(cli_config_overrides)?
    };

    let log_buffer = sauropod_logging::InMemoryLogBuffer::new(50, 20);
    sauropod_logging::initialize(sauropod_logging::LoggingConfig {
        verbose: config.verbose,
        pretty_print: true,
        in_memory_buffer: Some(log_buffer.clone()),
    });

    let server = match sauropod_server::Server::new(&config, log_buffer.clone())
        .instrument(tracing::info_span!("Server initialization"))
        .await
    {
        Ok(server) => server,
        Err(e) => {
            tracing::error!("error: {:?}", e);
            std::process::exit(1);
        }
    };
    dbg!(&config);

    if let Err(err) = sauropod_server::run_server(&config, server).await {
        tracing::error!("error: {:?}", err);
        std::process::exit(1);
    }
    Ok(())
}
