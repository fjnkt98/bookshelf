struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl From<DatabaseConfig> for sqlx::postgres::PgConnectOptions {
    fn from(config: DatabaseConfig) -> Self {
        Self::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.username)
            .password(&config.password)
            .database(&config.database)
    }
}

fn connect_database_with(cfg: DatabaseConfig) -> sqlx::Pool<sqlx::Postgres> {
    sqlx::Pool::connect_lazy_with(cfg.into())
}

pub async fn health_check() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

pub async fn health_check_db(
    axum::extract::State(db): axum::extract::State<sqlx::pool::Pool<sqlx::postgres::Postgres>>,
) -> axum::http::StatusCode {
    let result = sqlx::query("SELECT 1").fetch_one(&db).await;
    match result {
        Ok(_) => axum::http::StatusCode::OK,
        Err(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = DatabaseConfig {
        host: "localhost".into(),
        port: 5432,
        username: "app".into(),
        password: "app".into(),
        database: "app".into(),
    };
    let pool = connect_database_with(config);

    let app = axum::Router::new()
        .route("/hello", axum::routing::get(hello_world))
        .route("/health", axum::routing::get(health_check))
        .route("/health/db", axum::routing::get(health_check_db))
        .with_state(pool);
    let addr = std::net::SocketAddr::new(std::net::Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn health_check_works() {
        let status_code = health_check().await;
        assert_eq!(status_code, axum::http::StatusCode::OK);
    }

    #[sqlx::test]
    async fn health_check_db_works(pool: sqlx::pool::Pool<sqlx::postgres::Postgres>) {
        let status_code = health_check_db(axum::extract::State(pool)).await;
        assert_eq!(status_code, axum::http::StatusCode::OK);
    }
}
