use crate::types::auth::MeDto;
use crate::{
    components::{toast::ToastProvider, ScrollProgress},
    root::theme::theme_seed,
    root::Route,
    IO::auth::me,
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum UserState {
    Loading,
    Guest,
    User(MeDto),
    Error(String),
}

pub type UserContext = Signal<UserState>;

#[component]
pub fn RootLayout() -> Element {
    // Initialize user state once at root level
    let mut user_state: UserContext = use_signal(|| UserState::Loading);
    use_context_provider(|| user_state);
    let me_fut = use_server_future(move || me())?;

    // Init theme
    use_effect(move || {
        theme_seed();
    });

    // Load user data
    use_effect(move || {
        if !matches!(user_state(), UserState::Loading) {
            return;
        }
        match me_fut() {
            Some(Ok(Some(me))) => user_state.set(UserState::User(me)),
            Some(Ok(None)) => user_state.set(UserState::Guest),
            Some(Err(err)) => user_state.set(UserState::Error(err.to_string())),
            None => {}
        }
    });

    rsx! {
        ToastProvider {
            ScrollProgress {}
            Outlet::<Route> {}
        }
    }
}
