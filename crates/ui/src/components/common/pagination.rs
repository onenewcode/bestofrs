use dioxus::prelude::*;

use crate::components::pagination::{
    Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
    PaginationLinkSize, PaginationNext, PaginationPrevious,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum PagePart {
    Page(u32),
    Ellipsis,
}

fn build_page_parts(current_page: u32, total_pages: u32) -> Vec<PagePart> {
    if total_pages <= 1 {
        return Vec::new();
    }

    let current_page = current_page.clamp(1, total_pages);

    if total_pages <= 7 {
        return (1..=total_pages).map(PagePart::Page).collect();
    }

    let mut parts = Vec::new();
    parts.push(PagePart::Page(1));

    if current_page <= 4 {
        for p in 2..=5 {
            parts.push(PagePart::Page(p));
        }
        parts.push(PagePart::Ellipsis);
    } else if current_page >= total_pages - 3 {
        parts.push(PagePart::Ellipsis);
        for p in (total_pages - 4)..=total_pages - 1 {
            parts.push(PagePart::Page(p));
        }
    } else {
        parts.push(PagePart::Ellipsis);
        for p in (current_page - 1)..=current_page + 1 {
            parts.push(PagePart::Page(p));
        }
        parts.push(PagePart::Ellipsis);
    }

    parts.push(PagePart::Page(total_pages));
    parts
}

#[component]
pub fn CommonPagination(
    current_page: u32,
    total_pages: u32,
    on_page_change: Callback<u32>,
) -> Element {
    let current_page = current_page.clamp(1, total_pages);
    let can_prev = current_page > 1;
    let can_next = current_page < total_pages;
    let parts = build_page_parts(current_page, total_pages);

    rsx! {
        div { class: "pt-4",
            Pagination {
                PaginationContent {
                    PaginationItem {
                        PaginationPrevious {
                            href: "#",
                            aria_disabled: if can_prev { "false" } else { "true" },
                            style: if can_prev {
                                "opacity: 1; pointer-events: auto;"
                            } else {
                                "opacity: 0.5; pointer-events: none;"
                            },
                            onclick: move |e: MouseEvent| {
                                e.prevent_default();
                                if can_prev {
                                    on_page_change.call(current_page - 1);
                                }
                            },
                        }
                    }

                    for (idx, part) in parts.into_iter().enumerate() {
                        match part {
                            PagePart::Page(p) => rsx! {
                                PaginationItem { key: "page-{p}",
                                    PaginationLink {
                                        size: PaginationLinkSize::Icon,
                                        is_active: p == current_page,
                                        href: "#",
                                        onclick: move |e: MouseEvent| {
                                            e.prevent_default();
                                            on_page_change.call(p);
                                        },
                                        "{p}"
                                    }
                                }
                            },
                            PagePart::Ellipsis => rsx! {
                                PaginationItem { key: "ellipsis-{idx}",
                                    PaginationEllipsis {}
                                }
                            },
                        }
                    }

                    PaginationItem {
                        if can_next {
                            PaginationNext {
                                href: "#",
                                onclick: move |e: MouseEvent| {
                                    e.prevent_default();
                                    on_page_change.call(current_page + 1);
                                },
                            }
                        } else {
                            PaginationNext {
                                href: "#",
                                aria_disabled: "true",
                                style: "opacity: 0.5; pointer-events: none;",
                            }
                        }
                    }
                }
            }
        }
    }
}
