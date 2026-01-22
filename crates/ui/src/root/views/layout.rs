use crate::{components::toast::ToastProvider, root::routes::Route};
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        ToastProvider {
            header { class: "border-b border-[color:var(--primary-color-6)] bg-[color:var(--primary-color-2)]",
                div { class: "mx-auto max-w-6xl px-4 py-3 flex items-center justify-between",
                    div { class: "flex items-center gap-4",
                        Link { class: "font-semibold", to: Route::Home {}, "bestofrs" }
                        nav { class: "flex items-center gap-3 text-sm",
                            Link { class: "text-[color:var(--secondary-color-5)] hover:underline", to: Route::Home {}, "Home" }
                            Link { class: "text-[color:var(--secondary-color-5)] hover:underline", to: Route::Admin {}, "Admin" }
                        }
                    }
                }
            }

            main { class: "min-h-screen",
                Outlet::<Route> {}
            }
        }
    }
}
