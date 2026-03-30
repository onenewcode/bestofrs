mod result;
mod tip;

use dioxus::prelude::*;

use crate::components::common::IOCell;
use result::{skeleton::ResultSkeleton, Result};
use tip::Tip;

#[component]
pub fn Job() -> Element {
    let mut run_nonce = use_signal(|| 0u32);
    let run_nonce_read: ReadSignal<u32> = run_nonce.into();

    rsx! {
        section { class: "h-full min-h-0 w-full overflow-x-hidden overflow-y-auto space-y-4 border border-secondary-2 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                div { class: "font-mono text-xs font-semibold tracking-widest text-secondary-5", "JOBS / SNAPSHOT INGEST" }
                h2 { class: "text-lg font-semibold tracking-tight text-secondary-3", "Ingest Daily Snapshots" }
                Tip {  }
            }

            button {
                class: "inline-flex items-center justify-center border border-secondary-2 bg-secondary-2 px-4 py-2 text-sm font-medium text-primary transition-all hover:-translate-y-0.5 hover:shadow-comic-sm",
                onclick: move |_| run_nonce.with_mut(|v| *v += 1),
                "Run once"
            }
            div { class: "pt-2 border-t border-dashed border-primary-6",
                IOCell {
                    loading_fallback: rsx! { ResultSkeleton {} },
                    Result { run_nonce: run_nonce_read }
                }
            }
        }
    }
}
