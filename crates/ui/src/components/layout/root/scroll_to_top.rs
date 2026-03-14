use dioxus::prelude::*;
use dioxus_use_js::use_js;

use_js!("src/js/scroll_to_top.js"::mount_scroll_to_top);

#[component]
pub fn ScrollToTop() -> Element {
    use_effect(move || {
        spawn(async move {
            let _ = mount_scroll_to_top::<()>().await;
        });
    });

    rsx! {
        button {
            id: "root-scroll-to-top",
            r#type: "button",
            title: "Scroll to top",
            aria_label: "Scroll to top",
            class: "scroll-to-top-btn fixed bottom-6 right-6 z-[60] inline-flex h-10 w-10 cursor-pointer items-center justify-center border border-primary-6 bg-primary-1 text-secondary-4 transition-all duration-200 hover:text-grid-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-grid-accent",
            style: "opacity: 0; transform: translateY(8px); pointer-events: none;",
            span {
                aria_hidden: "true",
                "↑"
            }
            span {
                class: "sr-only",
                "Scroll to top"
            }
        }
    }
}
