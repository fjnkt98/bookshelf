async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new().route("/hello", axum::routing::get(hello_world));
    let addr = std::net::SocketAddr::new(std::net::Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();
}
