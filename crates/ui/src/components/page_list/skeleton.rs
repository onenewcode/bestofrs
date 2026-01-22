use crate::components::skeleton::Skeleton;
use dioxus::prelude::*;

#[component]
pub fn SubSkeleton() -> Element {
    rsx! {
        Skeleton {}
    }
}
