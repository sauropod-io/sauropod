use clap::Parser as _;

#[tokio::main]
async fn main() {
    let opt = sqlx_cli::Opt::parse();

    // no special handling here
    if let Err(error) = sqlx_cli::run(opt).await {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
