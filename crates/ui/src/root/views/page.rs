use crate::components::{IOCell, PaginatedList};
use dioxus::prelude::*;

#[component]
pub fn Page() -> Element {
    rsx! {
        IOCell { PaginatedList {} }
    }
}
