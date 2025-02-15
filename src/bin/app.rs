use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logger()?;
    bootstrap().await
}

fn init_logger() -> anyhow::Result<()> {
    let log_level = match shared::env::which() {
        shared::env::Environment::Development => "debug",
        shared::env::Environment::Production => "info",
    };

    let env_filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}

async fn bootstrap() -> anyhow::Result<()> {
    let app_config = shared::config::AppConfig::new()?;
    let pool = adapter::database::connect_database_with(&app_config.database);

    let registry = registry::AppRegistry::new(pool);

    let app = axum::Router::new()
        .merge(api::route::health::build_health_check_routers())
        .merge(api::route::book::build_book_routers())
        .layer(tower_http::cors())
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(
                    tower_http::trace::DefaultMakeSpan::new().level(tracing::Level::INFO),
                )
                .on_request(tower_http::trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(
                    tower_http::trace::DefaultOnResponse::new()
                        .level(tracing::Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        )
        .with_state(registry);

    let addr = std::net::SocketAddr::new(std::net::Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in server")
        .inspect_err(
            |e| tracing::error!(error.cause_chain = ?e, error.message = %e, "Unexpected error"),
        )
}
