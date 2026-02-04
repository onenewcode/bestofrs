use dioxus::prelude::*;

use crate::impls::auth;
use crate::impls::auth::{Auth, PendingOAuth, User};
use crate::impls::error::api_error;
use crate::impls::state::State;
use crate::types::auth::MeDto;
use dioxus_fullstack::response::{IntoResponse, Redirect, Response};

use domain::Role;

#[get("/api/auth/login/github", state: State, auth: Auth)]
pub async fn github_login_start() -> ServerFnResult<Response> {
    let app_state = state.0;

    let start = app_state
        .auth
        .command
        .start_login()
        .await
        .map_err(api_error)?;

    auth::set_pending_oauth(
        &auth.session,
        PendingOAuth {
            state: start.state,
            code_verifier: start.code_verifier,
        },
    );

    auth.session.set_store(true);
    Ok(Redirect::to(&start.authorization_url).into_response())
}

#[get("/api/auth/callback/github?code&state", ext_state: State, auth: Auth)]
pub async fn github_login_callback(code: String, state: String) -> ServerFnResult<Response> {
    let app_state = ext_state.0;

    let pending = auth::take_pending_oauth_checked(&auth.session, &state).map_err(|e| {
        ServerFnError::ServerError {
            code: 401,
            message: e.to_string(),
            details: None,
        }
    })?;

    let auth_user = app_state
        .auth
        .command
        .complete_login(code, pending.code_verifier)
        .await
        .map_err(api_error)?;

    let github_id: i64 =
        auth_user
            .user_id
            .as_str()
            .parse()
            .map_err(|e| ServerFnError::ServerError {
                code: 400,
                message: format!("invalid github id: {e}"),
                details: None,
            })?;

    auth::cache_user(User {
        id: github_id,
        anonymous: false,
        login: auth_user.login,
        avatar_url: auth_user.avatar_url,
        role: auth_user.role,
    });

    auth.login_user(github_id);

    auth.session.set_store(true);

    Ok(Redirect::to("/").into_response())
}

#[get("/api/me", auth: Auth)]
pub async fn me() -> ServerFnResult<Option<MeDto>> {
    if !auth.is_authenticated() {
        return Ok(None);
    }

    let user = auth.current_user.clone().unwrap_or_default();

    let role = match user.role {
        Role::Admin => "Admin",
        Role::Member => "Member",
    }
    .to_string();

    Ok(Some(MeDto {
        user_id: user.id.to_string(),
        login: user.login,
        avatar_url: user.avatar_url,
        role,
    }))
}

#[post("/api/auth/logout", auth: Auth)]
pub async fn logout() -> ServerFnResult<()> {
    auth.logout_user();
    Ok(())
}
