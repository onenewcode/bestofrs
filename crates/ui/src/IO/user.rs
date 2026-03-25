use dioxus::prelude::*;

use crate::impls::session::auth;
use crate::impls::session::preference;
use crate::impls::session::preference::PreferenceField;
use crate::impls::session::AppSession;
use crate::types::auth::MeDto;
use crate::types::preference::Preference;
use domain::Role;

#[get("/api/user/locale", session: AppSession, seed: preference::PreferenceSeed)]
pub async fn get_locale() -> ServerFnResult<String> {
    Ok(preference::resolve_locale(&session, &seed))
}

#[post("/api/user/locale/:locale", session: AppSession)]
pub async fn set_locale(locale: String) -> ServerFnResult<()> {
    preference::update_locale(&session, &locale);
    Ok(())
}

#[get("/api/user/theme", session: AppSession)]
pub async fn get_theme() -> ServerFnResult<Option<String>> {
    Ok(preference::resolve_theme(&session))
}

#[post("/api/user/theme/:theme", session: AppSession)]
pub async fn set_theme(theme: String) -> ServerFnResult<()> {
    preference::update_theme(&session, &theme);
    Ok(())
}

#[get("/api/user/grid-theme", session: AppSession)]
pub async fn get_grid_theme() -> ServerFnResult<String> {
    Ok(preference::resolve_grid_theme(&session))
}

#[post("/api/user/grid-theme/:grid_theme", session: AppSession)]
pub async fn set_grid_theme(grid_theme: String) -> ServerFnResult<()> {
    preference::update_grid_theme(&session, &grid_theme);
    Ok(())
}

#[get("/api/user/preference", session: AppSession, seed: preference::PreferenceSeed)]
pub async fn get_preference() -> ServerFnResult<Preference> {
    let locale = preference::resolve_locale(&session, &seed);
    let user = auth::current_user(&session).map(|user| {
        let role = match user.role {
            Role::Admin => "Admin",
            Role::Member => "Member",
        }
        .to_string();

        MeDto {
            user_id: user.id.to_string(),
            login: user.login,
            avatar_url: user.avatar_url,
            role,
        }
    });
    Ok(Preference {
        locale,
        theme: preference::resolve_theme(&session),
        grid_theme: preference::resolve_grid_theme(&session),
        privacy: preference::get(&session, PreferenceField::Privacy)
            .unwrap_or_else(|| "public".to_string()),
        user,
    })
}
