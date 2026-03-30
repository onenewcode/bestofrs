use app::prelude::{GithubLatestPushedRepoInfo, LatestPushedRepoCandidatesResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LatestPushedRepoDto {
    pub id: i64,
    pub full_name: String,
    pub stargazers_count: i64,
    pub created_at: String,
    pub pushed_at: String,
}

impl From<GithubLatestPushedRepoInfo> for LatestPushedRepoDto {
    fn from(value: GithubLatestPushedRepoInfo) -> Self {
        Self {
            id: value.id,
            full_name: value.full_name,
            stargazers_count: value.stargazers_count,
            created_at: value.created_at,
            pushed_at: value.pushed_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LatestPushedRepoQueryResultDto {
    pub requested_limit: usize,
    pub upstream_total_count: Option<u64>,
    pub fetched_raw_count: usize,
    pub unique_count: usize,
    pub filtered_existing_count: usize,
    pub returned_count: usize,
    pub items: Vec<LatestPushedRepoDto>,
}

impl From<LatestPushedRepoCandidatesResult> for LatestPushedRepoQueryResultDto {
    fn from(value: LatestPushedRepoCandidatesResult) -> Self {
        Self {
            requested_limit: value.requested_limit,
            upstream_total_count: value.upstream_total_count,
            fetched_raw_count: value.fetched_raw_count,
            unique_count: value.unique_count,
            filtered_existing_count: value.filtered_existing_count,
            returned_count: value.returned_count,
            items: value.items.into_iter().map(LatestPushedRepoDto::from).collect(),
        }
    }
}
