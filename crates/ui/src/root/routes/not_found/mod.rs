use crate::components::views::not_found::NotFound;
use dioxus::prelude::*;

#[component]
pub fn NotFoundView(segments: Vec<String>) -> Element {
    let _segments = segments;
    rsx! { NotFound {} }
}
