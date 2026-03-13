use dioxus::prelude::*;

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
        section { class: "space-y-4 border border-primary-6 bg-primary p-5 shadow-comic-sm",
            div { class: "space-y-1",
                h2 { class: "text-lg font-semibold", "README" }
                p { class: "text-sm text-secondary-5", "Rendered from GitHub README" }
            }

            match readme_fut() {
                Some(Ok(Some(readme))) => rsx! {
                    div { class: "rounded-md border border-primary-6 bg-primary-1 p-4",
                        CommonMarkdown {
                            src: readme.content,
                            link_base_url: readme.html_url,
                            image_base_url: readme.download_url,
                        }
                    }
                },
                Some(Ok(None)) => rsx! { div { class: "text-sm text-secondary-5", "README not found" } },
                Some(Err(e)) => Err(e)?,
                None => rsx! { skeleton::ReadmeSectionSkeleton {} },
            }
        }
    }
}
