use dioxus::prelude::*;

const DOTLOTTIE_WC_CDN: &str = "https://unpkg.com/@lottiefiles/dotlottie-wc/dist/dotlottie-wc.js";

#[component]
pub fn LottieWebComp(
    src: String,
    #[props(default = 256)] width: u32,
    #[props(default = 256)] height: u32,
) -> Element {
    let style = format!("width: {width}px; height: {height}px; display: inline-block;");

    rsx! {
        document::Script { src: DOTLOTTIE_WC_CDN, r#type: "module" }
        dotlottie-wc {
            src,
            autoplay: true,
            "loop": true,
            style,
        }
    }
}
