use {anyhow::Context as _, argh::FromArgs, std::fmt::Debug};

use sauropod_task::Task;
use sauropod_workflows::Workflow;

/// Schema runner tool.
#[derive(FromArgs, PartialEq, Debug)]
struct Command {
    #[argh(
        option,
        short = 'b',
        default = "String::from(\"http://localhost:8000\")"
    )]
    /// the backend to use
    backend: String,
    #[argh(positional)]
    file: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Command = argh::from_env();
    sauropod_logging::initialize(sauropod_logging::LoggingConfig {
        verbose: true,
        in_memory_buffer: None,
    });

    let workflow_schema: sauropod_schemas::workflow::Workflow = serde_json::from_str(
        &std::fs::read_to_string(&args.file).with_context(|| format!("Reading {}", &args.file))?,
    )?;
    let config = sauropod_config::Config::load(None)?;
    let llm_engine = sauropod_llm_inference::create_engine(&config).await?;
    let task = Workflow::from_schema(workflow_schema).await?;

    let models = config.models.to_map();
    if models.is_empty() {
        anyhow::bail!("No models configured in the configuration file");
    }

    let context = sauropod_task_context::make_default_task_context(llm_engine, models);
    let response = task
        .execute(
            serde_json::json!({"url": "https://hnrss.org/newest" }),
            context,
        )
        .await?;

    if let Some(response_str) = response.as_str() {
        println!("{}", response_str);
    } else {
        println!("Response: {:#?}", response);
    }

    Ok(())
}
