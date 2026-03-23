use crate::components::common::{
    CommonBreadcrumb, GradientDirection, GridBackground, GridPadding, GridPattern, GridType,
    GridWrapper,
};
use crate::components::icons::BORSFerrisIcon;
use dioxus::prelude::*;

#[component]
pub fn NotFound() -> Element {
    rsx! {
        GridWrapper { is_dot_on: true, padding: GridPadding::Sm, CommonBreadcrumb {} }

        GridWrapper {
            grid_type: GridType::Default,
            padding: GridPadding::Lg,
            is_dot_on: true,
            background: GridBackground {
                pattern: GridPattern::Slash,
                gradient: GradientDirection::ToBottom,
            },
            div { class: "flex min-h-[calc(100svh-4rem-340px)] w-full items-center justify-center",
                div { class: "w-full max-w-[34rem] px-2 md:max-w-3xl md:px-6",
                    div { class: "flex flex-col items-center justify-center gap-5 md:gap-6",
                        div { class: "relative flex items-center justify-center w-full py-2 md:py-6",
                            div { style: "color: var(--primary-error-color);",
                                BORSFerrisIcon { width: 260.0 }
                            }
                        }
                        div { class: "w-full max-w-[30rem] text-center text-sm md:text-base",
                            h1 { class: "text-primary-error", "Page Not Found." }
                        }
                    }
                }
            }
        }
    }
}
