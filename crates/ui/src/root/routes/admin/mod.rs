use dioxus::prelude::*;

use crate::components::views::admin::{Backup, Finder, Job, Projects, Tags};

#[component]
pub fn AdminProjectsView() -> Element {
    rsx! { Projects {} }
}

#[component]
pub fn AdminTagsView() -> Element {
    rsx! { Tags {} }
}

#[component]
pub fn AdminJobView() -> Element {
    rsx! { Job {} }
}

#[component]
pub fn AdminBackupView() -> Element {
    rsx! { Backup {} }
}

#[component]
pub fn AdminFinderView() -> Element {
    rsx! { Finder {} }
}
