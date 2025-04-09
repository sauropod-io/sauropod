use std::path::PathBuf;

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
            ),
    );
    let cli_matches = command.get_matches();
    let config_path = cli_matches.get_one::<PathBuf>(CONFIG_FLAG).cloned();
    let cli_config_overrides = sauropod_server::clap_to_config_source(cli_matches);

    let config = if let Some(config_path) = config_path {
        sauropod_config::Config::load_from_file(config_path, cli_config_overrides)?
    } else {
        sauropod_config::Config::load(cli_config_overrides)?
    };

    let log_buffer = sauropod_logging::InMemoryLogBuffer::new(50, 20);
    sauropod_logging::initialize(sauropod_logging::LoggingConfig {
        verbose: true,
        in_memory_buffer: Some(log_buffer.clone()),
    });

    let server = match sauropod_server::Server::new(&config, log_buffer.clone()).await {
        Ok(server) => server,
        Err(e) => {
            tracing::error!("error: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(err) = sauropod_server::run_server(&config, server).await {
        tracing::error!("error: {}", err);
        std::process::exit(1);
    }
    Ok(())
}
