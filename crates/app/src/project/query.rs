use std::sync::Arc;

use domain::Project;

use crate::app_error::AppResult;
use crate::common::pagination::{Page, Pagination};
use crate::project::ProjectRepo;

#[derive(Clone)]
pub struct ProjectQueryHandler {
    projects: Arc<dyn ProjectRepo>,
}

impl ProjectQueryHandler {
    pub fn new(projects: Arc<dyn ProjectRepo>) -> Self {
        Self { projects }
    }

    pub async fn list(&self, page: Pagination) -> AppResult<Page<Project>> {
        self.projects.list(page).await
    }
}
