use shared::{
    config::DatabaseConfig,
    error::{AppError, AppResult},
};
use sqlx::{PgPool, postgres::PgConnectOptions};

pub mod model;

fn make_pg_connect_options(config: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&config.host)
        .port(config.port)
        .username(&config.username)
        .password(&config.password)
        .database(&config.database)
}

#[derive(Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    // sqlx::PgPool への参照を取得する
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }

    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub async fn begin(&self) -> AppResult<sqlx::Transaction<'_, sqlx::Postgres>> {
        self.0.begin().await.map_err(AppError::TransactionError)
    }
}

pub fn connect_database_with(config: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pg_connect_options(config)))
}
