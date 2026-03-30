use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum FinderSortBy {
    CreatedAtDesc,
    StarsDesc,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct FinderTablePaginationState {
    pub(super) current_page: u32,
    pub(super) page_size: u32,
    pub(super) total_pages: u32,
    pub(super) total_items: u64,
}

impl Default for FinderTablePaginationState {
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
pub(super) struct FinderContext {
    pub(super) refresh_nonce: Signal<u32>,
    pub(super) committed_limit: Signal<usize>,
    pub(super) sort_by: Signal<FinderSortBy>,
    pub(super) table_pagination: Signal<FinderTablePaginationState>,
}
