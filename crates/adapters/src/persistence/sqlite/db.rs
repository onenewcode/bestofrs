use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use app::app_error::{AppError, AppResult};

pub async fn connect_and_migrate(database_url: &str) -> AppResult<SqlitePool> {
    let mut opts = SqliteConnectOptions::from_str(database_url).map_err(AppError::internal)?;

    let filename = opts.get_filename().to_path_buf();
    if filename != Path::new(":memory:") {
        opts = opts.create_if_missing(true);

        if filename.is_relative() {
            let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            let abs = workspace_root.join(&filename);
            if let Some(parent) = abs.parent() {
                std::fs::create_dir_all(parent).map_err(AppError::internal)?;
            }
            opts = opts.filename(abs);
        }
    }
    let pool = SqlitePool::connect_with(opts)
        .await
        .map_err(AppError::database)?;
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(AppError::database)?;

    Ok(pool)
}
