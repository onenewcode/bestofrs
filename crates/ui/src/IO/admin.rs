use dioxus::prelude::*;

use crate::impls::error::api_error;
use crate::impls::session::auth::AdminAuth;
use crate::impls::state::State;

use app::backup::BackupEntry;
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

#[post("/api/admin/backup/list", state: State, _auth: AdminAuth)]
pub async fn list_backups() -> ServerFnResult<Vec<BackupEntry>> {
    let app_state = state.0;
    let backups = app_state
        .backup
        .query
        .list_backups()
        .await
        .map_err(api_error)?;
    Ok(backups)
}

#[post("/api/admin/backup/create", state: State, _auth: AdminAuth)]
pub async fn create_backup(label: Option<String>) -> ServerFnResult<BackupEntry> {
    let app_state = state.0;
    let created = app_state
        .backup
        .command
        .create_backup(label)
        .await
        .map_err(api_error)?;
    Ok(created)
}

#[post("/api/admin/backup/delete", state: State, _auth: AdminAuth)]
pub async fn delete_backup(name: String) -> ServerFnResult<()> {
    let app_state = state.0;
    app_state
        .backup
        .command
        .delete_backup(name)
        .await
        .map_err(api_error)?;
    Ok(())
}

#[post("/api/admin/backup/restore", state: State, _auth: AdminAuth)]
pub async fn restore_backup(name: String) -> ServerFnResult<()> {
    let app_state = state.0;
    app_state
        .backup
        .command
        .restore_backup(name)
        .await
        .map_err(api_error)?;
    Ok(())
}
