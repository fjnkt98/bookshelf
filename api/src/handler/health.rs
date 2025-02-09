pub async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

pub async fn health_check_db(
    axum::extract::State(registry): axum::extract::State<registry::AppRegistry>,
) -> axum::http::StatusCode {
    if registry.health_check_repository().check_db().await {
        axum::http::StatusCode::OK
    } else {
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}
