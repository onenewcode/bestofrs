use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::icons::LogOutIcon;
use crate::root::layouts::{UserContext, UserState};
use crate::IO::auth::logout;

#[component]
pub fn UserProfile() -> Element {
    let mut user_state = use_context::<UserContext>();
    let navigator = use_navigator();

    rsx! {
        match user_state() {
            UserState::User(me) => {
                let fallback = me
                    .login
                    .chars()
                    .next()
                    .map(|c| c.to_ascii_uppercase().to_string())
                    .unwrap_or_else(|| "?".to_string());

                rsx! {
                    DropdownMenu {
                        DropdownMenuTrigger { style: "padding:0; border-radius:9999px; background:transparent; box-shadow:none; display:flex; align-items:center; justify-content:center;",
                            Avatar {
                                size: AvatarImageSize::Small,
                                style: "border:none; border-radius:9999px;",
                                if let Some(url) = me.avatar_url {
                                    AvatarImage { src: url, alt: me.login }
                                }
                                AvatarFallback { "{fallback}" }
                            }
                        }
                        DropdownMenuContent { style: "left:auto; right:0; min-width:10rem;",
                            DropdownMenuItem::<String> {
                                index: 0usize,
                                value: "logout",
                                on_select: move |_| {
                                    spawn(async move {
                                        if logout().await.is_ok() {
                                            user_state.set(UserState::Guest);
                                        }
                                        navigator.replace("/");
                                    });
                                },
                                span { class: "inline-flex gap-2 items-center",
                                    LogOutIcon { width: 16, height: 16 }
                                    span { {t!("layout_user_user_profile_logout")} }
                                }
                            }
                        }
                    }
                }
            }
            _ => rsx! {},
        }
    }
}
