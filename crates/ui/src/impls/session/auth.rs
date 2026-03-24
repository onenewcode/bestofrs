#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PendingOAuth {
    pub state: String,
    pub code_verifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SessionAuthUser {
    pub id: i64,
    pub login: String,
    pub avatar_url: Option<String>,
    pub role: domain::Role,
}

impl From<domain::AuthUser> for SessionAuthUser {
    fn from(user: domain::AuthUser) -> Self {
        Self {
            id: user.user_id.as_str().parse().unwrap_or(0),
            login: user.login,
            avatar_url: user.avatar_url,
            role: user.role,
        }
    }
}

#[cfg(feature = "server")]
mod server {
    use anyhow::{anyhow, Result};
    use axum::extract::FromRequestParts;
    use axum::http::request::Parts;
    use dioxus::prelude::{ServerFnError, ServerFnResult};
    use domain::Role;

    use super::{PendingOAuth, SessionAuthUser};
    use crate::impls::error::api_error;
    use crate::impls::session::AppSession;
    use crate::impls::session::consts::{SESSION_AUTH_PENDING_OAUTH_KEY, SESSION_AUTH_USER_KEY};

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum AuthField {
        PendingOAuth,
        User,
    }

    impl AuthField {
        fn key(self) -> &'static str {
            match self {
                Self::PendingOAuth => SESSION_AUTH_PENDING_OAUTH_KEY,
                Self::User => SESSION_AUTH_USER_KEY,
            }
        }
    }

    fn set<T>(session: &AppSession, field: AuthField, value: T)
    where
        T: serde::Serialize,
    {
        session.set(field.key(), value);
    }

    fn get<T>(session: &AppSession, field: AuthField) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        session.get::<T>(field.key())
    }

    fn take<T>(session: &AppSession, field: AuthField) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        session.get_remove::<T>(field.key())
    }

    pub fn consume_pending_oauth(session: &AppSession, state: &str) -> Result<PendingOAuth> {
        let Some(pending) = take::<PendingOAuth>(session, AuthField::PendingOAuth) else {
            return Err(anyhow!("missing pending oauth"));
        };

        if pending.state != state {
            return Err(anyhow!("invalid oauth state"));
        }

        Ok(pending)
    }

    fn clear_auth_namespace(session: &AppSession) {
        let _ = take::<SessionAuthUser>(session, AuthField::User);
        let _ = take::<PendingOAuth>(session, AuthField::PendingOAuth);
    }

    pub fn begin_oauth(session: &AppSession, pending: PendingOAuth) {
        set(session, AuthField::PendingOAuth, pending);
        session.set_store(true);
    }

    pub fn login(session: &AppSession, user: SessionAuthUser) {
        set(session, AuthField::User, user);
        session.renew();
        session.set_store(true);
    }

    pub fn current_user(session: &AppSession) -> Option<SessionAuthUser> {
        get::<SessionAuthUser>(session, AuthField::User)
    }

    pub fn logout(session: &AppSession) {
        clear_auth_namespace(session);
        session.renew();
        session.set_store(true);
    }

    #[derive(Clone)]
    pub struct AdminAuth;

    impl<S> FromRequestParts<S> for AdminAuth
    where
        S: Send + Sync,
    {
        type Rejection = ServerFnError;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
            let session = AppSession::from_request_parts(parts, state)
                .await
                .map_err(|_| api_error(app::app_error::AppError::InvalidCredentials))?;

            permission_validation(&session, Role::Admin).await?;
            Ok(AdminAuth)
        }
    }

    pub async fn permission_validation(
        session: &AppSession,
        required_role: Role,
    ) -> ServerFnResult<()> {
        let Some(user) = current_user(session) else {
            return Err(api_error(app::app_error::AppError::InvalidCredentials));
        };

        if user.role == required_role {
            Ok(())
        } else {
            Err(api_error(app::app_error::AppError::InvalidCredentials))
        }
    }
}

#[cfg(feature = "server")]
pub use server::*;

#[cfg(not(feature = "server"))]
mod client {
    use anyhow::{anyhow, Result};
    use dioxus::prelude::ServerFnResult;

    use super::{PendingOAuth, SessionAuthUser};
    use crate::impls::session::AppSession;

    pub fn consume_pending_oauth(_session: &AppSession, _state: &str) -> Result<PendingOAuth> {
        Err(anyhow!("missing pending oauth"))
    }

    pub fn current_user(_session: &AppSession) -> Option<SessionAuthUser> {
        None
    }

    pub fn begin_oauth(_session: &AppSession, _pending: PendingOAuth) {}

    pub fn login(_session: &AppSession, _user: SessionAuthUser) {}

    pub fn logout(_session: &AppSession) {}

    #[derive(Clone)]
    pub struct AdminAuth;

    pub async fn permission_validation(
        _session: &AppSession,
        _required_role: domain::Role,
    ) -> ServerFnResult<()> {
        Err(crate::impls::error::api_error(
            app::app_error::AppError::InvalidCredentials,
        ))
    }
}

#[cfg(not(feature = "server"))]
pub use client::*;
