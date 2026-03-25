use dioxus::prelude::*;

use crate::types::preference::Preference;

pub type PreferenceContext = Signal<Preference>;

#[component]
pub fn PreferenceProvider(initial: Preference, children: Element) -> Element {
    let preference: PreferenceContext = use_signal(|| initial.clone());
    use_context_provider(|| preference);

    children
}
