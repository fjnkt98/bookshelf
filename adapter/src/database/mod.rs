pub mod model;

fn make_pg_connect_options(
    config: &shared::config::DatabaseConfig,
) -> sqlx::postgres::PgConnectOptions {
    sqlx::postgres::PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.username)
        .password(&config.password)
        .database(&config.database)
}

#[derive(Clone)]
pub struct ConnectionPool(sqlx::pool::Pool<sqlx::Postgres>);

impl ConnectionPool {
    pub fn inner_ref(&self) -> &sqlx::pool::Pool<sqlx::Postgres> {
        &self.0
    }

    pub fn new(pool: sqlx::pool::Pool<sqlx::Postgres>) -> Self {
        Self(pool)
    }
}

pub fn connect_database_with(config: &shared::config::DatabaseConfig) -> ConnectionPool {
    ConnectionPool(sqlx::pool::Pool::connect_lazy_with(
        make_pg_connect_options(config),
    ))
}
