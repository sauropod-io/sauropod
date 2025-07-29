/// Sauropod inference engine
#[derive(Debug, clap::Parser)]
#[command(version, name = "sauropod", about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
pub struct Cli {
    /// The path to the TOML config file to load.
    #[arg(short, long, env = "SAUROPOD_CONFIG_FILE")]
    pub config_file: Option<std::path::PathBuf>,
    /// The port to listen on.
    #[arg(short, long, default_value = "3000", env = "SAUROPOD_PORT")]
    pub port: u16,
    /// The host to listen on.
    #[arg(long, default_value = "127.0.0.1", env = "SAUROPOD_HOST")]
    pub host: String,
    #[arg(long, default_value = "false", env = "SAUROPOD_VERBOSE")]
    /// Whether to log verbosely.
    ///
    /// You can also control the log level using the`SAUROPOD_LOG` environment variable, e.g. `SAUROPOD_LOG=debug`.
    pub verbose: bool,
    /// The path to output a Perfetto trace file to.
    #[arg(long, env = "SAUROPOD_TRACE_OUTPUT")]
    pub trace_output: Option<String>,
}

pub fn make_config_source(cli: &Cli) -> anyhow::Result<sauropod_config::ClapConfigSource> {
    let mut source = sauropod_config::ClapConfigSource::default();
    source.add_value("port".to_string(), cli.port)?;
    source.add_value("host".to_string(), cli.host.clone())?;
    source.add_value("verbose".to_string(), cli.verbose)?;
    source.add_value("trace_output".to_string(), cli.trace_output.clone())?;
    Ok(source)
}
