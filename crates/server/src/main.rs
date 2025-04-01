use {argh::FromArgs, std::fmt::Debug};

#[derive(FromArgs, PartialEq, Debug)]
/// Sauropod Scales server entrypoint.
struct Command {
    /// the host address to listen on - e.g. 127.0.0.1
    #[argh(option, short = 'h')]
    host: Option<String>,
    /// the port to listen on - e.g. 3140
    #[argh(option, short = 'p')]
    port: Option<u16>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let toplevel: Command = argh::from_env();
    let config = sauropod_config::Config::load(Some(sauropod_config::Config {
        host: toplevel.host,
        port: toplevel.port,
        ..sauropod_config::Config::default()
    }))?;

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
