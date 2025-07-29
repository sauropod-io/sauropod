use std::sync::Arc;

use axum::{Router, http, serve::ListenerExt as _};
use clap::Parser;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;
use utoipa_redoc::{Redoc, Servable};

fn create_api_router(
    global_state: Arc<sauropod_global_state::GlobalState>,
) -> (
    Router<Arc<sauropod_global_state::GlobalState>>,
    utoipa::openapi::OpenApi,
) {
    // Create an API router with OpenAPI documentation
    let (router, mut spec) =
        utoipa_axum::router::OpenApiRouter::<Arc<sauropod_global_state::GlobalState>>::new()
            .routes(utoipa_axum::routes!(
                sauropod_inference_realtime::get_v1_realtime,
                sauropod_inference_realtime::realtime_webrtc::post_v1_realtime,
            ))
            .routes(utoipa_axum::routes!(
                sauropod_inference_realtime::realtime_webrtc::v1_realtime_sessions
            ))
            .routes(utoipa_axum::routes!(
                sauropod_inference_responses::create_response,
                sauropod_inference_responses::get_response,
                sauropod_inference_responses::delete_response
            ))
            .routes(utoipa_axum::routes!(
                sauropod_inference_responses::get_models
            ))
            .split_for_parts();

    spec.info.title = env!("CARGO_PKG_NAME").to_string();
    spec.info.description = Some(env!("CARGO_PKG_DESCRIPTION").to_string());
    spec.info.contact = None;
    spec.info.license = None;
    spec.info.version = env!("CARGO_PKG_VERSION").to_string();

    (
        router
            .layer(axum::middleware::from_fn_with_state(
                global_state.clone(),
                sauropod_inference_http::auth_middleware,
            ))
            .layer(
                tower_http::cors::CorsLayer::new()
                    .allow_origin(tower_http::cors::Any)
                    .allow_methods(tower_http::cors::Any)
                    .allow_headers(tower_http::cors::Any),
            ),
        spec,
    )
}

pub fn initialize_tracing(pretty_print: bool, verbose: bool, trace_output: Option<&str>) {
    let fmt_layer = if pretty_print {
        tracing_subscriber::fmt::Layer::default()
            .pretty()
            .with_writer(std::io::stderr)
            .boxed()
    } else {
        tracing_subscriber::fmt::Layer::default()
            .with_writer(std::io::stderr)
            .boxed()
    };

    let trace_layer = trace_output.map(|output| {
        tracing_perfetto::PerfettoLayer::new(std::fs::File::create(output).unwrap()).boxed()
    });

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(
                    if verbose {
                        tracing_subscriber::filter::LevelFilter::INFO
                    } else {
                        tracing_subscriber::filter::LevelFilter::WARN
                    }
                    .into(),
                )
                .with_env_var("SAUROPOD_LOG")
                .from_env_lossy(),
        )
        .with(trace_layer)
        .with(sauropod_profiling::get_profiling_layer())
        .init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config: sauropod_config::Config = {
        let args = sauropod_inference_server::Cli::parse();
        let config_source = sauropod_inference_server::make_config_source(&args)?;

        if let Some(config_file) = args.config_file {
            sauropod_config::Config::load_from_file(config_file, config_source)?
        } else {
            sauropod_config::Config::load(config_source)?
        }
    };
    initialize_tracing(
        config.verbose,
        config.verbose,
        config.trace_output.as_deref(),
    );

    if cfg!(not(feature = "cuda")) && sauropod_device_discovery::has_cuda_device()? {
        tracing::warn!("A CUDA-capable GPU was detected, but CUDA is not enabled in the build.");
    }

    let global_state = Arc::new(sauropod_global_state::GlobalState::new(&config).await?);

    let (api_app, spec) = create_api_router(global_state.clone());
    let app = api_app
        .without_v07_checks()
        .with_state(global_state)
        .merge(Redoc::with_url("/docs", spec.clone()))
        .route(
            "/openapi.json",
            axum::routing::get(axum::response::Json(spec.clone())),
        )
        .layer(TraceLayer::new_for_http())
        .layer(tower_http::request_id::PropagateRequestIdLayer::x_request_id())
        .layer(
            tower_http::sensitive_headers::SetSensitiveHeadersLayer::new(std::iter::once(
                http::header::AUTHORIZATION,
            )),
        )
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            http::header::SERVER,
            http::HeaderValue::from_static(concat!("Sauropod v", env!("CARGO_PKG_VERSION"))),
        ));

    println!("Starting server at {}:{}", &config.host, &config.port);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &config.host, &config.port))
        .await?
        .tap_io(|tcp_stream| {
            if let Err(err) = tcp_stream.set_nodelay(true) {
                tracing::warn!("failed to set TCP_NODELAY on incoming connection: {err:#}");
            }
        });
    axum::serve(listener, app).await?;

    Ok(())
}
