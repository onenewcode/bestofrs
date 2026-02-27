use dioxus::prelude::*;

use crate::components::views::admin::{AdminJob, AdminProjects, AdminTags};

#[component]
pub fn AdminProjectsView() -> Element {
    rsx! { AdminProjects {} }
}

#[component]
pub fn AdminTagsView() -> Element {
    rsx! { AdminTags {} }
}

#[component]
pub fn AdminJobView() -> Element {
    rsx! { AdminJob {} }
}
