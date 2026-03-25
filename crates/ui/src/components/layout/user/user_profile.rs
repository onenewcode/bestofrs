use dioxus::prelude::*;
use dioxus_i18n::t;

use crate::components::avatar::{Avatar, AvatarFallback, AvatarImage, AvatarImageSize};
use crate::components::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger,
};
use crate::components::icons::LogOutIcon;
use crate::components::providers::PreferenceContext;
use crate::IO::auth::logout;

#[component]
pub fn UserProfile() -> Element {
    let mut preference = use_context::<PreferenceContext>();
    let navigator = use_navigator();

    let Some(me) = preference().user.clone() else {
        return rsx! {};
    };

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
                    AvatarImage { src: me.avatar_url.unwrap_or_default(), alt: me.login }
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
                                preference.with_mut(|pref| pref.user = None);
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
