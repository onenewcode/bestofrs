use crate::impls::error::api_error;
use crate::impls::state::State;
use crate::types::search::{SearchRepoDto, SearchTagDto};
use app::prelude::{Page, Pagination};
use dioxus::prelude::*;

#[post("/api/search/repos", state: State)]
pub async fn search_repo_page(
    key: String,
    page: Pagination,
) -> ServerFnResult<Page<SearchRepoDto>> {
    let app_state = state.0;
    let result = app_state
        .repo
        .query
        .search_repo_page_by_key(&key, page)
        .await
        .map_err(api_error)?;
    Ok(result.map(SearchRepoDto::from))
}

#[post("/api/search/tags", state: State)]
pub async fn search_tag_page(key: String, page: Pagination) -> ServerFnResult<Page<SearchTagDto>> {
    let app_state = state.0;
    let result = app_state
        .repo
        .query
        .search_tag_page_by_key(&key, page)
        .await
        .map_err(api_error)?;
    Ok(result.map(SearchTagDto::from))
}
