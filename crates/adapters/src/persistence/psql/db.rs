use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::PgPool;
use std::str::FromStr;

use app::app_error::{AppError, AppResult};

pub async fn connect_and_migrate(database_url: &str) -> AppResult<PgPool> {
    let opts = PgConnectOptions::from_str(database_url).map_err(AppError::internal)?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(opts)
        .await
        .map_err(AppError::database)?;

    sqlx::migrate!("./migrations_postgres")
        .run(&pool)
        .await
        .map_err(AppError::database)?;

    Ok(pool)
}
