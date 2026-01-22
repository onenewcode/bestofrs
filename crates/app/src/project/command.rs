use std::sync::Arc;

use domain::{Project, RepoId};
use serde::{Deserialize, Serialize};

use crate::app_error::AppResult;
use crate::project::ProjectRepo;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportProjectCommand {
    pub repo_id: String,
    pub name: String,
    pub slug: String,
    pub description: String,

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportProjectsCommand {
    pub items: Vec<ImportProjectCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportProjectsReport {
    pub total: usize,
    pub upserted: usize,
    pub skipped_invalid: usize,
    pub failed_upsert: usize,

    #[serde(default)]
    pub invalid_examples: Vec<String>,

    #[serde(default)]
    pub error_examples: Vec<String>,
}

impl ImportProjectsReport {
    fn new(total: usize) -> Self {
        Self {
            total,
            upserted: 0,
            skipped_invalid: 0,
            failed_upsert: 0,
            invalid_examples: Vec::new(),
            error_examples: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemoveProjectCommand {
    pub repo_id: String,
}

#[derive(Clone)]
pub struct ProjectCommandHandler {
    projects: Arc<dyn ProjectRepo>,
}

impl ProjectCommandHandler {
    pub fn new(projects: Arc<dyn ProjectRepo>) -> Self {
        Self { projects }
    }

    pub async fn import_projects(
        &self,
        cmd: ImportProjectsCommand,
    ) -> AppResult<ImportProjectsReport> {
        const MAX_INVALID_EXAMPLES: usize = 20;

        let mut report = ImportProjectsReport::new(cmd.items.len());

        let mut projects = Vec::new();

        for item in cmd.items {
            let repo_id = match RepoId::parse(&item.repo_id) {
                Ok(v) => v,
                Err(_) => {
                    report.skipped_invalid += 1;
                    if report.invalid_examples.len() < MAX_INVALID_EXAMPLES {
                        report.invalid_examples.push(item.repo_id);
                    }
                    continue;
                }
            };
            projects.push(Project {
                id: repo_id,
                name: item.name,
                slug: item.slug,
                description: item.description,
                override_description: item.override_description,
                url: item.url,
                override_url: item.override_url,
                status: item.status,
                logo: item.logo,
                twitter: item.twitter,
                comments: item.comments,
            });
        }

        if !projects.is_empty() {
            self.projects.upsert_many(&projects).await?;
            report.upserted = projects.len();
        }
        Ok(report)
    }

    pub async fn remove_project(&self, cmd: RemoveProjectCommand) -> AppResult<()> {
        let repo_id = RepoId::parse(&cmd.repo_id)?;
        self.projects.remove(repo_id.to_string()).await
    }
}
