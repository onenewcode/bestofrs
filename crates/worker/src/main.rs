use app::app_error::{AppError, AppResult};
use infra::setup::init_app_container;

#[tokio::main]
async fn main() -> AppResult<()> {
    let container = init_app_container().await?;

    let res = container.ingest_daily_snapshots.execute().await?;
    println!(
        "{}",
        serde_json::to_string(&res).map_err(AppError::internal)?
    );

    Ok(())
}
