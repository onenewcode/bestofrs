use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut started = use_signal(|| false);

    use_effect(move || {
        if started() {
            return;
        }
        started.set(true);
        _ = document::eval("window.location.assign('/api/auth/login/github');");
    });

    rsx! {
        div { class: "mx-auto max-w-6xl px-4 py-8",
            div { class: "rounded-xl border border-primary-6 bg-primary-2 p-6 space-y-2",
                h1 { class: "text-2xl font-semibold tracking-tight", "Login" }
                div { class: "text-sm text-secondary-5", "Redirecting to GitHub..." }
            }
        }
    }
}
