use crate::app_error::AppResult;
use crate::repo::{GithubGateway, RepoCommandHandler, RepoGithubLookupExt, RepoProjectOverrideExt, RepoRepo};
use domain::{ProjectCreated, ProjectUpdated, Repo};
use futures::{stream, StreamExt, TryStreamExt};
use std::sync::Arc;

#[derive(Clone)]
pub struct ProjectEventHandler {
    repo_command: RepoCommandHandler,
    repos: Arc<dyn RepoRepo>,
    github: Arc<dyn GithubGateway>,
}

impl ProjectEventHandler {
    pub fn new(
        repo_command: RepoCommandHandler,
        repos: Arc<dyn RepoRepo>,
        github: Arc<dyn GithubGateway>,
    ) -> Self {
        Self {
            repo_command,
            repos,
            github,
        }
    }

    pub async fn handle_project_created(&self, event: &ProjectCreated) -> AppResult<()> {
        self.handle_projects_created(std::slice::from_ref(event))
            .await
    }

    pub async fn handle_projects_created(&self, events: &[ProjectCreated]) -> AppResult<()> {
        if events.is_empty() {
            return Ok(());
        }
        let repos = events
            .iter()
            .map(|event| event.repo.clone())
            .collect::<Vec<Repo>>();
        self.repo_command.upsert_many(&repos).await
    }

    pub async fn handle_project_updated(&self, event: &ProjectUpdated) -> AppResult<()> {
        self.handle_projects_updated(std::slice::from_ref(event))
            .await
    }

    pub async fn handle_projects_updated(&self, events: &[ProjectUpdated]) -> AppResult<()> {
        if events.is_empty() {
            return Ok(());
        }

        const FETCH_CONCURRENCY: usize = 16;
        let projects = events
            .iter()
            .map(|event| event.project.clone())
            .collect::<Vec<_>>();
        let project_ids = projects
            .iter()
            .map(|project| project.id.clone())
            .collect::<Vec<_>>();
        let existing_repos = self.repos.list_by_ids(&project_ids).await?;
        if existing_repos.is_empty() {
            return Ok(());
        }

        let fetch_items = projects
            .into_iter()
            .filter_map(|project| {
                let repo = existing_repos
                    .iter()
                    .find(|repo| repo.id == project.id)
                    .cloned()?;
                Some((project, repo))
            })
            .collect::<Vec<_>>();

        let github = self.github.clone();
        let synced_repos: Vec<Repo> = stream::iter(fetch_items.into_iter())
            .map(|(project, repo)| {
                let github = github.clone();
                async move {
                    let github_repo = github.fetch_repo_by_lookup_key(&repo.github_lookup_key()).await?;
                    let synced_repo = repo.with_project_overrides(
                        &project,
                        github_repo.homepage.as_deref(),
                        github_repo.owner_avatar_url.as_deref(),
                    );
                    Ok::<Repo, crate::app_error::AppError>(synced_repo)
                }
            })
            .buffer_unordered(FETCH_CONCURRENCY)
            .try_collect()
            .await?;

        if synced_repos.is_empty() {
            return Ok(());
        }
        self.repo_command.upsert_many(&synced_repos).await
    }
}
