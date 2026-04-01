use dioxus::prelude::*;
use dioxus_markdown::{LinkDescription, Markdown};
use dioxus_use_js::use_js;

use super::rewrite::{next_markdown_root_id, resolve_href, resolve_src};

use_js!("src/js/dom_bridge.js"::rewrite_markdown_links);

#[component]
pub fn CommonMarkdown(
    src: String,
    class: Option<String>,
    link_base_url: Option<String>,
    image_base_url: Option<String>,
) -> Element {
    let class = class.unwrap_or_default();
    let root_id = use_hook(next_markdown_root_id);
    let effect_root_id = root_id.clone();
    let data_link_base_url = link_base_url.unwrap_or_default();
    let data_image_base_url = image_base_url.unwrap_or_default();

    let render_links = {
        let link_base_url = data_link_base_url.clone();
        let image_base_url = data_image_base_url.clone();
        move |link: LinkDescription<Element>| {
            let LinkDescription {
                url,
                content,
                title,
                image,
                ..
            } = link;

            if image {
                let resolved = resolve_src(&url, &image_base_url, &link_base_url);
                rsx! {
                    img {
                        src: resolved,
                        alt: title,
                    }
                }
            } else {
                let resolved = resolve_href(&url, &link_base_url);
                rsx! {
                    a { href: resolved, {content} }
                }
            }
        }
    };

    use_effect(move || {
        let root_id = effect_root_id.clone();
        spawn(async move {
            let _ = rewrite_markdown_links::<()>(root_id).await;
        });
    });

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            id: root_id,
            class: "common-markdown {class}",
            "data-md-link-base-url": data_link_base_url,
            "data-md-image-base-url": data_image_base_url,
            Markdown {
                src,
                render_links,
            }
        }
    }
}
