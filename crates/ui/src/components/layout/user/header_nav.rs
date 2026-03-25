use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::providers::PreferenceContext;
use crate::root::Route;

#[component]
pub fn HeaderNav(#[props(default = false)] vertical: bool) -> Element {
    let preference = use_context::<PreferenceContext>();
    let show_admin = matches!(preference().user.as_ref(), Some(me) if me.role == "Admin");
    let nav_class = if vertical {
        "flex flex-col items-stretch gap-1 text-sm"
    } else {
        "flex items-center gap-2 text-sm"
    };
    let nav_link_class = if vertical {
        "rounded-md px-3 py-2 font-mono text-xs tracking-wide text-secondary-5 transition-colors hover:bg-primary-1 hover:text-secondary-3"
    } else {
        "px-3 py-1 font-mono text-xs tracking-wide text-secondary-5 transition-colors hover:text-secondary-3"
    };

    rsx! {
        nav { class: "{nav_class}",
            Link { class: "{nav_link_class}", to: Route::HomeView {}, {t!("layout_user_header_nav_home")} }
            Link {
                class: "{nav_link_class}",
                to: Route::RepoListView {
                    tags: None,
                    metric: None,
                    range: None,
                    page: None,
                    size: None,
                },
                {t!("layout_user_header_nav_repos")}
            }
            Link { class: "{nav_link_class}", to: Route::TagListView {}, {t!("layout_user_header_nav_tags")} }
            Link { class: "{nav_link_class}", to: Route::AboutView {}, {t!("layout_user_header_nav_about")} }
            if show_admin {
                Link { class: "{nav_link_class}", to: Route::AdminProjectsView {}, {t!("layout_user_header_nav_admin")} }
            }
        }
    }
}
