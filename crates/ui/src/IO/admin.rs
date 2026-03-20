use dioxus::prelude::*;

use crate::impls::auth::AdminAuth;
use crate::impls::error::api_error;
use crate::impls::state::State;

use app::prelude::IngestDailySnapshotsResult;

#[post(
    "/api/admin/ingest_daily_snapshots/run_once",
    state: State,
    _auth: AdminAuth
)]
pub async fn run_ingest_daily_snapshots() -> ServerFnResult<IngestDailySnapshotsResult> {
    let app_state = state.0;

    let res = app_state
        .ingest_daily_snapshots
        .execute()
        .await
        .map_err(api_error)?;

    Ok(res)
}
