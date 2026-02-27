mod ingest_daily_snapshots_control;
mod project_management;
mod tags_management;
use dioxus::prelude::*;
use crate::components::common::IOCell;
pub use ingest_daily_snapshots_control::IngestDailySnapshotsControl;
pub use project_management::ProjectManagement;
pub use tags_management::TagsManagement;

#[component]
pub fn AdminProjects() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        IOCell { ProjectManagement {} }
    }
}

#[component]
pub fn AdminTags() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        IOCell { TagsManagement {} }
    }
}

#[component]
pub fn AdminJob() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        IOCell { IngestDailySnapshotsControl {} }
    }
}
