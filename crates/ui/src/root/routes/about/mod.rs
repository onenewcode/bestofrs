use crate::components::views::about::About;
use dioxus::prelude::*;

#[component]
pub fn AboutView() -> Element {
    rsx! { About {} }
}
