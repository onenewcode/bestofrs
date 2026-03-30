use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::root::Route;

#[derive(Clone, PartialEq)]
struct Crumb {
    label: String,
    to: Option<Route>,
}

fn home_crumb() -> Crumb {
    Crumb {
        label: t!("common_breadcrumb_home").to_string(),
        to: Some(Route::HomeView {}),
    }
}

fn repo_crumb() -> Crumb {
    Crumb {
        label: t!("common_breadcrumb_repo").to_string(),
        to: Some(Route::RepoListView {
            tags: None,
            metric: None,
            range: None,
            page: None,
            size: None,
        }),
    }
}

fn admin_crumb() -> Crumb {
    Crumb {
        label: t!("common_breadcrumb_admin").to_string(),
        to: Some(Route::AdminProjectsView {}),
    }
}

fn build_crumbs(route: &Route) -> Vec<Crumb> {
    match route {
        Route::HomeView {} => vec![home_crumb()],
        Route::RepoListView { .. } => vec![home_crumb(), repo_crumb()],
        Route::RepoDetailView { owner, name, .. } => vec![
            home_crumb(),
            repo_crumb(),
            Crumb {
                label: format!("{owner}/{name}"),
                to: None,
            },
        ],
        Route::TagListView {} => vec![
            home_crumb(),
            Crumb {
                label: t!("common_breadcrumb_tags").to_string(),
                to: None,
            },
        ],
        Route::AboutView {} => vec![
            home_crumb(),
            Crumb {
                label: t!("common_breadcrumb_about").to_string(),
                to: None,
            },
        ],
        Route::LoginView {} => vec![
            home_crumb(),
            Crumb {
                label: t!("common_breadcrumb_login").to_string(),
                to: None,
            },
        ],
        Route::AdminProjectsView {} => vec![
            admin_crumb(),
            Crumb {
                label: t!("common_breadcrumb_projects").to_string(),
                to: None,
            },
        ],
        Route::AdminTagsView {} => vec![
            admin_crumb(),
            Crumb {
                label: t!("common_breadcrumb_tags").to_string(),
                to: None,
            },
        ],
        Route::AdminJobView {} => vec![
            admin_crumb(),
            Crumb {
                label: t!("common_breadcrumb_job").to_string(),
                to: None,
            },
        ],
        Route::AdminBackupView {} => vec![
            admin_crumb(),
            Crumb {
                label: t!("common_breadcrumb_backup").to_string(),
                to: None,
            },
        ],
        Route::AdminFinderView {} => vec![
            admin_crumb(),
            Crumb {
                label: t!("common_breadcrumb_finder").to_string(),
                to: None,
            },
        ],
        Route::NotFoundView { .. } => vec![
            home_crumb(),
            Crumb {
                label: t!("common_breadcrumb_404").to_string(),
                to: None,
            },
        ],
    }
}

#[component]
pub fn CommonBreadcrumb(
    #[props(default = false)] compact: bool,
    #[props(default = "/".to_string())] separator: String,
    #[props(default = String::new())] class: String,
) -> Element {
    let route = use_route::<Route>();
    let crumbs = build_crumbs(&route);
    let total = crumbs.len();

    rsx! {
        nav {
            aria_label: t!("common_breadcrumb_aria_label"),
            class: "flex items-center",
            class: if compact { "text-xs" } else { "text-sm" },
            class: "{class}",
            ol { class: "flex flex-wrap items-center gap-1.5 font-mono tracking-wide text-secondary-5",
                for (idx, crumb) in crumbs.into_iter().enumerate() {
                    li { key: "{crumb.label}-{idx}", class: "inline-flex items-center gap-1.5",
                        if idx > 0 {
                            span { class: "select-none text-secondary-6", "{separator}" }
                        }
                        if idx + 1 == total {
                            span {
                                aria_current: "page",
                                class: "text-secondary-2",
                                "{crumb.label}"
                            }
                        } else if let Some(to) = crumb.to {
                            Link {
                                to: to,
                                class: "transition-colors hover:text-grid-accent",
                                "{crumb.label}"
                            }
                        } else {
                            span { class: "text-secondary-4", "{crumb.label}" }
                        }
                    }
                }
            }
        }
    }
}
