use domain::Project;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectDto {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub repo_id: String,
}

impl From<Project> for ProjectDto {
    fn from(value: Project) -> Self {
        let repo_id = value.id.to_string();
        Self {
            // Keep `id` for UI keys/backward-compat; it matches repo_id.
            id: repo_id.clone(),
            name: value.name,
            slug: value.slug,
            description: value.description,
            repo_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectImportItem {
    pub id: Option<String>,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub repo_id: String,

    #[serde(default)]
    pub override_description: bool,

    pub url: Option<String>,

    #[serde(default)]
    pub override_url: bool,

    pub status: Option<String>,
    pub logo: Option<String>,
    pub twitter: Option<String>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportProjectsResult {
    pub total: usize,
    pub upserted: usize,
    pub skipped_invalid: usize,
    pub failed_upsert: usize,

    #[serde(default)]
    pub invalid_examples: Vec<String>,

    #[serde(default)]
    pub error_examples: Vec<String>,
}
