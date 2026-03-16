#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PendingOAuth {
    pub state: String,
    pub code_verifier: String,
}

/// Session user shared by server and client.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i64,
    pub anonymous: bool,
    pub login: String,
    pub avatar_url: Option<String>,
    pub role: domain::Role,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: 0,
            anonymous: true,
            login: "Guest".to_string(),
            avatar_url: None,
            role: domain::Role::Member,
        }
    }
}

impl User {
    pub fn from_auth_user(u: domain::AuthUser) -> Self {
        Self {
            id: u.user_id.as_str().parse().unwrap_or(0),
            anonymous: false,
            login: u.login,
            avatar_url: u.avatar_url,
            role: u.role,
        }
    }
}

#[cfg(feature = "server")]
mod server {
    use dioxus::prelude::{ServerFnError, ServerFnResult};

    use super::{PendingOAuth, User};
    use crate::impls::error::api_error;

    use anyhow::{anyhow, Result};
    use app::auth::AuthUserCache;
    use async_trait::async_trait;
    use axum::extract::FromRequestParts;
    use axum::http::request::Parts;
    use axum_session::Session;
    use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
    use axum_session_redispool::SessionRedisPool;
    use domain::Role;

    use std::{fmt, sync::Arc};

    const KEY_PENDING_OAUTH: &str = "pending_oauth";

    pub type WebSession = Session<SessionRedisPool>;

    #[derive(Clone)]
    pub struct AppStateHandle {
        pub app: Arc<infra::setup::AppState>,
        pub user_cache: Arc<dyn AuthUserCache>,
    }

    impl fmt::Debug for AppStateHandle {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("AppStateHandle(..)")
        }
    }

    pub fn set_pending_oauth(session: &WebSession, pending: PendingOAuth) {
        session.set(KEY_PENDING_OAUTH, pending);
    }

    pub fn take_pending_oauth(session: &WebSession) -> Option<PendingOAuth> {
        session.get_remove(KEY_PENDING_OAUTH)
    }

    pub fn take_pending_oauth_checked(session: &WebSession, state: &str) -> Result<PendingOAuth> {
        let Some(pending) = take_pending_oauth(session) else {
            return Err(anyhow!("missing pending oauth"));
        };

        if pending.state != state {
            return Err(anyhow!("invalid oauth state"));
        }

        Ok(pending)
    }

    #[async_trait]
    impl Authentication<User, i64, AppStateHandle> for User {
        async fn load_user(userid: i64, pool: Option<&AppStateHandle>) -> Result<User> {
            if let Some(handle) = pool {
                let user_id = domain::UserId::new(userid.to_string());
                if let Some(auth_user) = handle.user_cache.get(&user_id).await {
                    return Ok(User::from_auth_user(auth_user));
                }
            }
            // Fallback as anonymous (e.g. cache miss after Redis flush).
            Ok(User::default())
        }

        fn is_authenticated(&self) -> bool {
            !self.anonymous
        }

        fn is_active(&self) -> bool {
            !self.anonymous
        }

        fn is_anonymous(&self) -> bool {
            self.anonymous
        }
    }

    pub type Auth = AuthSession<User, i64, SessionRedisPool, AppStateHandle>;

    pub type AuthLayer = AuthSessionLayer<User, i64, SessionRedisPool, AppStateHandle>;

    pub fn auth_config() -> AuthConfig<i64> {
        AuthConfig::<i64>::default()
            .with_anonymous_user_id(Some(0))
            .set_cache(true)
            .with_session_id("github_user_id".to_string())
    }

    pub fn auth_layer(
        app_state: Arc<infra::setup::AppState>,
        user_cache: Arc<dyn AuthUserCache>,
    ) -> AuthLayer {
        let handle = AppStateHandle {
            app: app_state,
            user_cache,
        };
        AuthLayer::new(Some(handle)).with_config(auth_config())
    }

    #[derive(Clone)]
    pub struct AdminAuth(pub Auth);

    impl<S> FromRequestParts<S> for AdminAuth
    where
        S: Send + Sync,
    {
        type Rejection = ServerFnError;
        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
            let auth = Auth::from_request_parts(parts, state)
                .await
                .map_err(|_| api_error(app::app_error::AppError::InvalidCredentials))?;

            permission_validation(&auth, Role::Admin).await?;
            Ok(AdminAuth(auth))
        }
    }

    pub async fn permission_validation(auth: &Auth, required_role: Role) -> ServerFnResult<()> {
        let user = auth.current_user.clone().unwrap_or_default();
        if auth.is_authenticated() && user.role == required_role {
            Ok(())
        } else {
            Err(api_error(app::app_error::AppError::InvalidCredentials))
        }
    }
}

#[cfg(not(feature = "server"))]
mod client {
    use anyhow::{anyhow, Result};
    use dioxus::prelude::ServerFnResult;

    use super::{PendingOAuth, User};
    use crate::impls::error::api_error;

    pub type WebSession = ();

    #[derive(Debug, Clone, Default)]
    pub struct Auth {
        pub session: WebSession,
        pub current_user: Option<User>,
    }

    impl Auth {
        pub fn login_user(&self, _id: i64) {}
        pub fn logout_user(&self) {}
        pub fn is_authenticated(&self) -> bool {
            false
        }
    }

    pub type AuthLayer = ();

    pub fn auth_config() {}

    pub fn auth_layer<T>(_app_state: std::sync::Arc<T>) -> AuthLayer {}

    pub fn set_pending_oauth(_session: &WebSession, _pending: PendingOAuth) {}

    pub fn take_pending_oauth(_session: &WebSession) -> Option<PendingOAuth> {
        None
    }

    pub fn take_pending_oauth_checked(_session: &WebSession, _state: &str) -> Result<PendingOAuth> {
        Err(anyhow!("missing pending oauth"))
    }

    pub struct AdminAuth;

    pub async fn permission_validation(_auth: &Auth, _role: domain::Role) -> ServerFnResult<()> {
        Err(api_error(app::app_error::AppError::InvalidCredentials))
    }
}

#[cfg(feature = "server")]
pub use server::*;

#[cfg(not(feature = "server"))]
pub use client::*;
