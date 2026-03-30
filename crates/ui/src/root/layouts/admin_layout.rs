use dioxus::prelude::*;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::components::common::CommonBreadcrumb;
use crate::components::icons::{
    ArrowLeftIcon, BestOfRSIcon, SaveIcon, ScrollTextIcon, SearchIcon, TagsIcon,
};
use crate::components::providers::PreferenceContext;
use crate::components::separator::Separator;
use crate::components::sidebar::{
    Sidebar, SidebarCollapsible, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel,
    SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton, SidebarMenuButtonSize,
    SidebarMenuItem, SidebarProvider, SidebarRail, SidebarTrigger, SidebarVariant,
};
use crate::components::skeleton::Skeleton;
use crate::root::Route;

#[component]
pub fn AdminLayout() -> Element {
    let navigator = use_navigator();
    let route = use_route::<Route>();
    let preference = use_context::<PreferenceContext>();
    let user_info = preference().user.filter(|user| user.role == "Admin");

    let Some(user_info) = user_info else {
        navigator.replace(Route::HomeView {});
        return rsx! { Skeleton { class: "skeleton w-screen h-screen" } };
    };

    let is_projects = matches!(route, Route::AdminProjectsView {});
    let is_tags = matches!(route, Route::AdminTagsView {});
    let is_job = matches!(route, Route::AdminJobView {});
    let is_backup = matches!(route, Route::AdminBackupView {});
    let is_finder = matches!(route, Route::AdminFinderView {});

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("../../components/layout/admin/style.css"),
        }
        SidebarProvider {
            Sidebar {
                variant: SidebarVariant::Sidebar,
                collapsible: SidebarCollapsible::Icon,
                SidebarHeader {
                    SidebarMenu {
                        SidebarMenuItem {
                            SidebarMenuButton {
                                size: SidebarMenuButtonSize::Lg,
                                r#as: move |attributes: Vec<Attribute>| rsx! {
                                    div { ..attributes,
                                        Avatar { size: AvatarImageSize::Small,
                                            AvatarImage {
                                                src: user_info.avatar_url.clone().unwrap_or_default(),
                                                alt: user_info.login.clone(),
                                            }
                                            AvatarFallback { "{user_info.login.chars().next().unwrap_or('?')}" }
                                        }
                                        div { class: "grid flex-1 text-left text-sm leading-tight",
                                            span { class: "truncate font-semibold", "{user_info.login}" }
                                            span { class: "truncate text-xs", "Admin" }
                                        }
                                    }
                                },
                            }
                        }
                    }
                }
                SidebarContent {
                    SidebarGroup {
                        SidebarGroupLabel { "Management" }
                        SidebarMenu {
                            SidebarMenuItem {
                                SidebarMenuButton {
                                    is_active: is_projects,
                                    tooltip: rsx! { "Project" },
                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                        button {
                                            onclick: move |_| {
                                                let _ = navigator.push(Route::AdminProjectsView {});
                                            },
                                            ..attributes,
                                            ScrollTextIcon {}
                                            span { "Project" }
                                        }
                                    },
                                }
                            }
                            SidebarMenuItem {
                                SidebarMenuButton {
                                    is_active: is_tags,
                                    tooltip: rsx! { "Tags" },
                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                        button {
                                            onclick: move |_| {
                                                let _ = navigator.push(Route::AdminTagsView {});
                                            },
                                            ..attributes,
                                            TagsIcon {}
                                            span { "Tags" }
                                        }
                                    },
                                }
                            }
                            SidebarMenuItem {
                                SidebarMenuButton {
                                    is_active: is_job,
                                    tooltip: rsx! { "Job" },
                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                        button {
                                            onclick: move |_| {
                                                let _ = navigator.push(Route::AdminJobView {});
                                            },
                                            ..attributes,
                                            ScrollTextIcon {}
                                            span { "Job" }
                                        }
                                    },
                                }
                            }
                            SidebarMenuItem {
                                SidebarMenuButton {
                                    is_active: is_finder,
                                    tooltip: rsx! { "Finder" },
                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                        button {
                                            onclick: move |_| {
                                                let _ = navigator.push(Route::AdminFinderView {});
                                            },
                                            ..attributes,
                                            SearchIcon {}
                                            span { "Finder" }
                                        }
                                    },
                                }
                            }
                            SidebarMenuItem {
                                SidebarMenuButton {
                                    is_active: is_backup,
                                    tooltip: rsx! { "Backup" },
                                    r#as: move |attributes: Vec<Attribute>| rsx! {
                                        button {
                                            onclick: move |_| {
                                                let _ = navigator.push(Route::AdminBackupView {});
                                            },
                                            ..attributes,
                                            SaveIcon {}
                                            span { "Backup" }
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
                SidebarFooter {
                    SidebarMenu {
                        SidebarMenuItem {
                            Link {
                                to: Route::HomeView {},
                                class: "sidebar-menu-button",
                                "data-slot": "sidebar-menu-button",
                                "data-sidebar": "menu-button",
                                "data-size": "default",
                                "data-variant": "default",
                                "data-active": "false",
                                ArrowLeftIcon {}
                                span { "Back Home" }
                            }
                        }
                    }
                }
                SidebarRail {}
            }
            SidebarInset {
                header { class: "flex h-14 shrink-0 items-center gap-3 border-b border-primary-6 bg-primary-1 px-4",
                    SidebarTrigger {}
                    Separator { height: "1rem", horizontal: false }
                    BestOfRSIcon {}
                    div { class: "min-w-0",
                        div { class: "font-mono text-[11px] tracking-widest text-secondary-5",
                            "ADMIN PANEL"
                        }
                        h1 { class: "truncate text-sm font-semibold text-secondary-3 md:text-base",
                            "Data Management"
                        }
                    }
                }

                CommonBreadcrumb { class: "py-2 px-4 md:px-6" }
                div { class: "min-h-0 flex-1 overflow-hidden p-4 pt-0 md:p-6 md:pt-0",
                    SuspenseBoundary {
                        fallback: move |_: SuspenseContext| {
                            rsx! {
                                Skeleton { class: "skeleton w-full h-full" }
                            }
                        },
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}
