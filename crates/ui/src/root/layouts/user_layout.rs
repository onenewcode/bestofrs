use dioxus::prelude::*;

use crate::components::{
    button::{Button, ButtonVariant},
    common::{GridPadding, GridType, GridWrapper},
    icons::{BestOfRSIcon, MenuIcon},
    sheet::{Sheet, SheetContent, SheetDescription, SheetHeader, SheetSide, SheetTitle},
    skeleton::Skeleton,
    ColorSwitcher, Footer, FuzzySearch, HeaderNav, ThemeSwitcher, UserProfile,
};
use crate::root::Route;

#[component]
pub fn UserLayout() -> Element {
    let mut mobile_menu_open = use_signal(|| false);

    rsx! {
        div { class: "flex min-h-screen w-full flex-col overflow-x-clip",
            header { class: "sticky top-0 z-50 w-full shrink-0 bg-primary px-3 md:px-8",
                GridWrapper {
                    class: "mx-auto max-w-7xl".to_string(),
                    padding: GridPadding::None,
                    div { class: "md:hidden",
                        div { class: "flex h-14 items-center justify-between gap-2 px-3",
                            Link {
                                class: "flex items-center gap-2 border border-transparent px-1 text-secondary-4 transition-all hover:border-primary-6 hover:bg-primary-1",
                                to: Route::HomeView {},
                                BestOfRSIcon { height: 36.0 }
                            }
                            div { class: "flex items-center gap-2",
                                FuzzySearch {}
                                Button {
                                    variant: ButtonVariant::Outline,
                                    class: "button inline-flex h-9 items-center justify-center",
                                    style: "border-radius: 0.5rem;",
                                    aria_label: "Open mobile menu",
                                    onclick: move |_| mobile_menu_open.set(true),
                                    MenuIcon { width: 18, height: 18 }
                                }
                            }
                        }
                        Sheet {
                            open: mobile_menu_open(),
                            on_open_change: move |v| mobile_menu_open.set(v),
                            SheetContent {
                                side: SheetSide::Left,
                                class: "w-[18rem] max-w-[90vw] p-0".to_string(),
                                SheetHeader {
                                    class: "hidden",
                                    SheetTitle { "Navigation menu" }
                                    SheetDescription { "Mobile navigation menu" }
                                }
                                div { class: "flex h-[100dvh] min-h-0 flex-col px-4 py-4",
                                    div { class: "shrink-0",
                                        Link {
                                            class: "flex items-center gap-2 border border-transparent px-1 text-secondary-4 transition-all hover:border-primary-6 hover:bg-primary-1",
                                            to: Route::HomeView {},
                                            BestOfRSIcon { height: 34.0 }
                                        }
                                    }
                                    div { class: "min-h-0 flex-1 overflow-y-auto py-3",
                                        HeaderNav { vertical: true }
                                    }
                                    div { class: "shrink-0",
                                        div { class: "h-px w-full bg-primary-6" }
                                        div { class: "flex items-center justify-between gap-3 pt-3",
                                            UserProfile {}
                                            div { class: "flex items-center gap-3",
                                                ColorSwitcher {}
                                                ThemeSwitcher {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "hidden h-16 items-center justify-between gap-3 px-4 md:flex",
                        div { class: "flex items-center gap-2",
                            Link { class: "flex items-center gap-2 border border-transparent px-1 text-secondary-4 transition-all hover:border-primary-6 hover:bg-primary-1", to: Route::HomeView {},
                                BestOfRSIcon { height: 40.0 }
                            }
                            HeaderNav {}
                        }
                        div { class: "flex items-center gap-3",
                            FuzzySearch {}
                            ColorSwitcher {}
                            ThemeSwitcher {}
                            UserProfile {}
                        }
                    }
                }
            }
            main { class: "relative z-10 flex-1 w-full px-3 md:px-8",
                div { class: "mx-auto w-full max-w-7xl",
                    SuspenseBoundary {
                        fallback: move |_: SuspenseContext| {
                            rsx! {
                                Skeleton { class: "skeleton h-full w-full" }
                            }
                        },
                        Outlet::<Route> {}
                    }
                }
            }
            footer { class: "relative z-50 w-full px-3 md:px-8",
                GridWrapper {
                    class: Some("mx-auto max-w-7xl".to_string()),
                    padding: GridPadding::None,
                    is_dot_on: false,
                    grid_type: GridType::End,
                    Footer {}
                }
            }
        }
    }
}
