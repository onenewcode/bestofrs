use dioxus::prelude::*;
use dioxus_use_js::use_js;

use_js!("src/js/scroll_progress.js"::mount_scroll_progress);

#[component]
pub fn ScrollProgress() -> Element {
    use_effect(move || {
        spawn(async move {
            let _ = mount_scroll_progress::<()>().await;
        });
    });

    rsx! {
        progress {
            id: "root-scroll-progress",
            class: "pointer-events-none fixed inset-x-0 top-0 z-[70] m-0 block h-0.5 w-full appearance-none border-0 bg-primary-6/50 align-top",
            aria_label: "Page scroll progress",
            max: "100",
            value: "0",
        }
    }
}
