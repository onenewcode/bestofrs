#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PendingOAuth {
    pub state: String,
    pub code_verifier: String,
}

#[cfg(feature = "server")]
mod server {
    use dioxus::prelude::{ServerFnError, ServerFnResult};

    use super::PendingOAuth;
    use crate::impls::error::api_error;

    use anyhow::{anyhow, Result};
    use async_trait::async_trait;
    use axum::extract::FromRequestParts;
    use axum::http::request::Parts;
    use axum_session::{Session, SessionNullPool};
    use axum_session_auth::{AuthConfig, AuthSession, AuthSessionLayer, Authentication};
    use domain::Role;

    use std::{
        collections::HashMap,
        fmt,
        sync::{Arc, LazyLock, RwLock},
    };

    const KEY_PENDING_OAUTH: &str = "pending_oauth";

    pub type WebSession = Session<SessionNullPool>;

    /// Authenticated user principal stored in (server-side) cache and loaded by `axum_session_auth`.
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct User {
        pub id: i64,
        pub anonymous: bool,
        pub login: String,
        pub avatar_url: Option<String>,
        pub role: Role,
    }

    impl Default for User {
        fn default() -> Self {
            Self {
                id: 0,
                anonymous: true,
                login: "Guest".to_string(),
                avatar_url: None,
                role: Role::Member,
            }
        }
    }

    #[derive(Clone)]
    pub struct AppStateHandle(pub Arc<infra::setup::AppState>);

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

    static USER_CACHE: LazyLock<RwLock<HashMap<i64, User>>> =
        LazyLock::new(|| RwLock::new(HashMap::new()));

    pub fn cache_user(user: User) {
        if let Ok(mut map) = USER_CACHE.write() {
            map.insert(user.id, user);
        }
    }

    fn get_cached_user(id: i64) -> Option<User> {
        USER_CACHE.read().ok().and_then(|m| m.get(&id).cloned())
    }

    #[async_trait]
    impl Authentication<User, i64, AppStateHandle> for User {
        async fn load_user(userid: i64, _pool: Option<&AppStateHandle>) -> Result<User> {
            // fallback as anonymous(e.g. cache miss).
            // This avoids turning auth resolution failures into 500s.
            Ok(get_cached_user(userid).unwrap_or_default())
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

    pub type Auth = AuthSession<User, i64, SessionNullPool, AppStateHandle>;

    pub type AuthLayer = AuthSessionLayer<User, i64, SessionNullPool, AppStateHandle>;

    pub fn auth_config() -> AuthConfig<i64> {
        AuthConfig::<i64>::default()
            .with_anonymous_user_id(Some(0))
            .set_cache(true)
            .with_session_id("github_user_id".to_string())
    }

    pub fn auth_layer(app_state: Arc<infra::setup::AppState>) -> AuthLayer {
        AuthLayer::new(Some(AppStateHandle(app_state))).with_config(auth_config())
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

    use super::PendingOAuth;
    use crate::impls::error::api_error;

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

    pub fn cache_user(_user: User) {}

    pub struct AdminAuth;

    pub async fn permission_validation(_auth: &Auth, _role: domain::Role) -> ServerFnResult<()> {
        Err(api_error(app::app_error::AppError::InvalidCredentials))
    }
}

#[cfg(feature = "server")]
pub use server::*;

#[cfg(not(feature = "server"))]
pub use client::*;
