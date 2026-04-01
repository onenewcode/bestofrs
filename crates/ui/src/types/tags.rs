use app::repo::{RepoSearchTagItem, RepoTagFacet, RepoTagListItem, RepoTagTopRepo};
use domain::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagDto {
    pub label: String,
    pub value: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub repos_total: Option<u64>,
}

impl From<Tag> for TagDto {
    fn from(value: Tag) -> Self {
        Self {
            label: value.label.as_str().to_string(),
            value: value.value.as_str().to_string(),
            description: value.description,
            repos_total: None,
        }
    }
}

impl From<RepoSearchTagItem> for TagDto {
    fn from(value: RepoSearchTagItem) -> Self {
        Self {
            label: value.label,
            value: value.value,
            description: value.description,
            repos_total: Some(value.repos_total),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportTagsResult {
    pub total: usize,
    pub upserted: usize,
    pub skipped_invalid: usize,
    pub failed_upsert: usize,
    #[serde(default)]
    pub invalid_examples: Vec<String>,
    #[serde(default)]
    pub error_examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagImportItem {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagTopRepoDto {
    pub repo_id: String,
    pub avatar_urls: Vec<String>,
}

impl From<RepoTagTopRepo> for TagTopRepoDto {
    fn from(value: RepoTagTopRepo) -> Self {
        Self {
            repo_id: value.repo_id,
            avatar_urls: value.avatar_urls,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagListItemDto {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub repos_total: u64,
    pub top_repos: Vec<TagTopRepoDto>,
}

impl From<RepoTagListItem> for TagListItemDto {
    fn from(value: RepoTagListItem) -> Self {
        Self {
            label: value.label,
            value: value.value,
            description: value.description,
            repos_total: value.repos_total,
            top_repos: value
                .top_repos
                .into_iter()
                .map(TagTopRepoDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagFacetDto {
    pub value: String,
    pub count: u64,
}

impl From<RepoTagFacet> for TagFacetDto {
    fn from(value: RepoTagFacet) -> Self {
        Self {
            value: value.value,
            count: value.count,
        }
    }
}
