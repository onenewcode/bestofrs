use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::IO::repos::get_repo_readme;
use crate::components::common::CommonMarkdown;
use super::RepoDetailContext;

pub(super) mod skeleton;

#[component]
pub(crate) fn ReadmeSection() -> Element {
    let ctx = use_context::<RepoDetailContext>();
    let owner = ctx.owner;
    let name = ctx.name;
    let readme_fut = use_server_future(move || get_repo_readme(owner(), name()))?;

    rsx! {
        section { class: "space-y-4",
            div { class: "space-y-1",
                h2 { class: "text-5xl leading-[0.8] font-black tracking-tighter text-secondary-2 uppercase md:text-7xl",
                    "Read"
                    span { class: "text-transparent [-webkit-text-stroke:2px_var(--primary-color-6)]", "me" }
                }
                p { class: "text-sm text-secondary-5", {t!("view_repo_detail_readme_rendered_from_github")} }
            }

            match readme_fut() {
                Some(Ok(Some(readme))) => rsx! {
                    div { class: "bg-primary-1 md:rounded-md md:border md:border-primary-6 md:p-4",
                        CommonMarkdown {
                            src: readme.content,
                            link_base_url: readme.html_url,
                            image_base_url: readme.download_url,
                        }
                    }
                },
                Some(Ok(None)) => rsx! {
                    div { class: "text-sm text-secondary-5",
                        {t!("view_repo_detail_readme_not_found")}
                    }
                },
                Some(Err(e)) => Err(e)?,
                None => rsx! { skeleton::ReadmeSectionSkeleton {} },
            }
        }
    }
}
