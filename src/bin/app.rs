#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap().await
}

async fn bootstrap() -> anyhow::Result<()> {
    let app_config = shared::config::AppConfig::new()?;
    let pool = adapter::database::connect_database_with(&app_config.database);

    let registry = registry::AppRegistry::new(pool);

    let app = axum::Router::new()
        .merge(api::route::health::build_health_check_routers())
        .with_state(registry);

    let addr = std::net::SocketAddr::new(std::net::Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
