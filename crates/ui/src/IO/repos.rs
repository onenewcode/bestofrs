use dioxus::prelude::*;

use crate::types::repos::RepoDto;
use crate::types::snapshot_deltas::SnapshotDeltaDto;
use crate::types::snapshots::SnapshotDto;
use crate::IO::api_error::api_error;
use crate::IO::extractors::AppStateExt;

use app::prelude::{Page, Pagination};

#[post("/api/repos", state: AppStateExt)]
pub async fn list_repos(page: Pagination) -> ServerFnResult<Page<RepoDto>> {
    let app_state = state.0;

    let repos_page = app_state
        .repo
        .query
        .list(page)
        .await
        .map_err(api_error)?;
    Ok(repos_page.map(RepoDto::from))
}

#[post("/api/repos/:owner/:name", state: AppStateExt)]
pub async fn get_repo(owner: String, name: String) -> ServerFnResult<Option<RepoDto>> {
    let app_state = state.0;
    let repo = app_state
        .repo
        .query
        .get_by_owner_name(&owner, &name)
        .await
        .map_err(api_error)?;
    Ok(repo.map(RepoDto::from))
}

#[post("/api/repos/:owner/:name/snapshots", state: AppStateExt)]
pub async fn list_repo_snapshots(
    owner: String,
    name: String,
    page: Pagination,
) -> ServerFnResult<Page<SnapshotDto>> {
    let app_state = state.0;

    let items_page = app_state
        .snapshot
        .query
        .list_by_owner_name(&owner, &name, page)
        .await
        .map_err(api_error)?;
    Ok(items_page.map(SnapshotDto::from))
}

#[post("/api/repos/:owner/:name/deltas", state: AppStateExt)]
pub async fn list_repo_deltas(
    owner: String,
    name: String,
    page: Pagination,
) -> ServerFnResult<Page<SnapshotDeltaDto>> {
    let app_state = state.0;

    let items_page = app_state
        .snapshot
        .query
        .list_deltas_by_owner_name(&owner, &name, page)
        .await
        .map_err(api_error)?;
    Ok(items_page.map(SnapshotDeltaDto::from))
}
