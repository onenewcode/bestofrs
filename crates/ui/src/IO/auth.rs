use dioxus::prelude::*;

use crate::impls::error::api_error;
use crate::impls::session::auth;
use crate::impls::session::AppSession;
use crate::impls::state::State;
use crate::types::auth::MeDto;
use dioxus_fullstack::response::{IntoResponse, Redirect, Response};

use domain::Role;

#[get("/api/auth/login/github", state: State, session: AppSession)]
pub async fn github_login_start() -> ServerFnResult<Response> {
    let app_state = state.0;

    let start = app_state
        .auth
        .command
        .start_login()
        .await
        .map_err(api_error)?;

    auth::begin_oauth(
        &session,
        auth::PendingOAuth {
            state: start.state,
            code_verifier: start.code_verifier,
        },
    );

    Ok(Redirect::to(&start.authorization_url).into_response())
}

#[get("/api/auth/callback/github?code&state", ext_state: State, session: AppSession)]
pub async fn github_login_callback(code: String, state: String) -> ServerFnResult<Response> {
    let app_state = ext_state.0;

    let pending =
        auth::consume_pending_oauth(&session, &state).map_err(|e| ServerFnError::ServerError {
            code: 401,
            message: e.to_string(),
            details: None,
        })?;

    let auth_user = app_state
        .auth
        .command
        .complete_login(code, pending.code_verifier)
        .await
        .map_err(api_error)?;

    auth::login(&session, auth_user.into());

    Ok(Redirect::to("/").into_response())
}

#[get("/api/me", session: AppSession)]
pub async fn me() -> ServerFnResult<Option<MeDto>> {
    let Some(user) = auth::current_user(&session) else {
        return Ok(None);
    };

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

#[post("/api/auth/logout", session: AppSession)]
pub async fn logout() -> ServerFnResult<()> {
    auth::logout(&session);
    Ok(())
}
