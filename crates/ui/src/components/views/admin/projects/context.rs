use crate::types::projects::ProjectDto;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub(super) struct ProjectTablePaginationState {
    pub(super) current_page: u32,
    pub(super) page_size: u32,
    pub(super) total_pages: u32,
    pub(super) total_items: u64,
}

impl Default for ProjectTablePaginationState {
    fn default() -> Self {
        Self {
            current_page: 1,
            page_size: 20,
            total_pages: 0,
            total_items: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct ProjectsContext {
    pub(super) refresh: Signal<u32>,
    pub(super) search_key: Signal<String>,
    pub(super) edit_panel_tab: Signal<Option<String>>,
    pub(super) table_pagination: Signal<ProjectTablePaginationState>,
}

#[derive(Clone, PartialEq)]
pub(super) enum ProjectPanelMode {
    Add,
    Edit(Box<ProjectDto>),
}
