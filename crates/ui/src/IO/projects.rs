use crate::types::projects::{ImportProjectsResult, ProjectDto, ProjectImportItem};
use dioxus::prelude::*;

use crate::IO::api_error::api_error;
use crate::IO::extractors::AppStateExt;

use app::prelude::{Page, Pagination};
use app::project::{ImportProjectCommand, ImportProjectsCommand, RemoveProjectCommand};
use serde::Deserialize;

#[post("/api/projects", state: AppStateExt)]
pub async fn list_projects(page: Pagination) -> ServerFnResult<Page<ProjectDto>> {
    let app_state = state.0;
    let projects_page = app_state
        .project
        .query
        .list(page)
        .await
        .map_err(api_error)?;
    Ok(projects_page.map(ProjectDto::from))
}

#[post("/api/projects/import", state: AppStateExt)]
pub async fn import_projects(
    items: Vec<ProjectImportItem>,
) -> ServerFnResult<ImportProjectsResult> {
    let app_state = state.0;

    let cmd = ImportProjectsCommand {
        items: items
            .into_iter()
            .map(|it| ImportProjectCommand {
                repo_id: it.repo_id,
                name: it.name,
                slug: it.slug,
                description: it.description,
                override_description: it.override_description,
                url: it.url,
                override_url: it.override_url,
                status: it.status,
                logo: it.logo,
                twitter: it.twitter,
                comments: it.comments,
            })
            .collect(),
    };

    let report = app_state
        .project
        .command
        .import_projects(cmd)
        .await
        .map_err(api_error)?;

    Ok(ImportProjectsResult {
        total: report.total,
        upserted: report.upserted,
        skipped_invalid: report.skipped_invalid,
        failed_upsert: report.failed_upsert,
        invalid_examples: report.invalid_examples,
        error_examples: report.error_examples,
    })
}

#[derive(Debug, Clone, Deserialize)]
struct ProjectSeedItem {
    name: String,
    full_name: String,
}

#[post("/api/projects/import_json", state: AppStateExt)]
pub async fn import_projects_json(json_text: String) -> ServerFnResult<ImportProjectsResult> {
    let app_state = state.0;

    let items: Vec<ProjectSeedItem> =
        serde_json::from_str(&json_text).map_err(|e| ServerFnError::ServerError {
            code: 400,
            message: format!("invalid json: {e}"),
            details: None,
        })?;

    let cmd = ImportProjectsCommand {
        items: items
            .into_iter()
            .map(|it| ImportProjectCommand {
                repo_id: it.full_name,
                name: it.name.clone(),
                slug: it.name,
                description: String::new(),
                override_description: false,
                url: None,
                override_url: false,
                status: None,
                logo: None,
                twitter: None,
                comments: None,
            })
            .collect(),
    };

    let report = app_state
        .project
        .command
        .import_projects(cmd)
        .await
        .map_err(api_error)?;

    Ok(ImportProjectsResult {
        total: report.total,
        upserted: report.upserted,
        skipped_invalid: report.skipped_invalid,
        failed_upsert: report.failed_upsert,
        invalid_examples: report.invalid_examples,
        error_examples: report.error_examples,
    })
}

#[post("/api/projects/remove", state: AppStateExt)]
pub async fn remove_project(repo_id: String) -> ServerFnResult<()> {
    let app_state = state.0;

    app_state
        .project
        .command
        .remove_project(RemoveProjectCommand { repo_id })
        .await
        .map_err(api_error)?;

    Ok(())
}
