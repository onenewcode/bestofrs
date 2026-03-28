use crate::{Project, Repo};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectCreated {
    pub repo: Repo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectUpdated {
    pub project: Project,
}
