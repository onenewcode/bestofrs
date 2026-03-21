use dioxus::prelude::*;

use crate::types::auth::MeDto;

#[derive(Clone, PartialEq)]
pub enum UserState {
    Loading,
    Guest,
    User(MeDto),
    Error(String),
}

pub type UserContext = Signal<UserState>;

#[component]
pub fn UserProvider(state: UserState, children: Element) -> Element {
    let mut user_state: UserContext = use_signal(|| state.clone());
    use_context_provider(|| user_state);

    use_effect(move || {
        if matches!(user_state(), UserState::Loading) && user_state() != state {
            user_state.set(state.clone());
        }
    });

    children
}
