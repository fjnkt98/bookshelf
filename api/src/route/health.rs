pub fn build_health_check_routers() -> axum::Router<registry::AppRegistry> {
    let routers = axum::Router::new()
        .route(
            "/",
            axum::routing::get(crate::handler::health::health_check),
        )
        .route(
            "/db",
            axum::routing::get(crate::handler::health::health_check_db),
        );
    axum::Router::new().nest("/health", routers)
}
