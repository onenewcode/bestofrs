use dioxus::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub struct ConfigContext {
    pub site_url: String,
}

#[component]
pub fn ConfigProvider(config: ConfigContext, children: Element) -> Element {
    use_context_provider(|| config.clone());
    children
}
