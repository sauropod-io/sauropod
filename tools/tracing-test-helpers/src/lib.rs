use tracing_subscriber::prelude::*;

/// Initialize tracing for unit tests.
pub fn init_tracing() {
    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::Layer::default()
                .pretty()
                .with_writer(std::io::stderr),
        )
        .with(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::DEBUG.into())
                .with_env_var("RUST_LOG")
                .from_env_lossy(),
        )
        .try_init();
}
