mod routes;
mod views;

use dioxus::prelude::*;
use routes::Route;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const DX_COMPONENT_CSS: Asset = asset!("/assets/dx-components-theme.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DX_COMPONENT_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: FAVICON }

        Router::<Route> {}
    }
}
