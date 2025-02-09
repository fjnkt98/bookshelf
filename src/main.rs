pub async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = axum::Router::new()
        .route("/hello", axum::routing::get(hello_world))
        .route("/health", axum::routing::get(health_check));
    let addr = std::net::SocketAddr::new(std::net::Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
