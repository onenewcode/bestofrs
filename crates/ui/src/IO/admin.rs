use dioxus::prelude::*;

use crate::IO::api_error::api_error;
use crate::IO::extractors::AppStateExt;

use app::prelude::IngestDailySnapshotsResult;

#[post("/api/admin/ingest_daily_snapshots/run_once", state: AppStateExt)]
pub async fn run_ingest_daily_snapshots() -> ServerFnResult<IngestDailySnapshotsResult> {
    if std::env::var("APP_ENV").ok().as_deref() == Some("production") {
        return Err(ServerFnError::ServerError {
            message: "forbidden".to_string(),
            code: 403,
            details: None,
        });
    }

    let app_state = state.0;

    let res = app_state
        .ingest_daily_snapshots
        .execute()
        .await
        .map_err(api_error)?;

    Ok(res)
}
