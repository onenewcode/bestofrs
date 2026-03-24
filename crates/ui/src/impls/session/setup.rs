use axum_session::{SameSite, SessionConfig, SessionLayer, SessionMode, SessionStore};
use axum_session_redispool::SessionRedisPool;
use redis_pool::SingleRedisPool;

use crate::impls::session::consts::{
    SESSION_COOKIE_NAME, SESSION_MAX_LIFETIME_DAYS, SESSION_MAX_LIFETIME_FALLBACK_DAYS,
    SESSION_TTL_FALLBACK_DAYS,
};

/// Result of session store initialization.
pub struct SessionSetup {
    pub layer: SessionLayer<SessionRedisPool>,
}

/// Create a Redis-backed session store from an existing pool.
pub async fn create_session_setup(
    redis_pool: SingleRedisPool,
    session_ttl_seconds: u64,
    is_production: bool,
) -> SessionSetup {
    let ttl = chrono::Duration::try_seconds(session_ttl_seconds as i64).unwrap_or_else(|| {
        chrono::Duration::try_days(SESSION_TTL_FALLBACK_DAYS).unwrap()
    });

    let max_lifetime = chrono::Duration::try_days(SESSION_MAX_LIFETIME_DAYS)
        .unwrap_or_else(|| chrono::Duration::try_days(SESSION_MAX_LIFETIME_FALLBACK_DAYS).unwrap());

    let session_config = SessionConfig::default()
        .with_session_name(SESSION_COOKIE_NAME)
        .with_mode(SessionMode::OptIn)
        .with_cookie_same_site(SameSite::Lax)
        .with_http_only(true)
        .with_secure(is_production)
        .with_lifetime(ttl)
        .with_max_lifetime(max_lifetime)
        .with_max_age(Some(max_lifetime));

    let session_store =
        SessionStore::<SessionRedisPool>::new(Some(redis_pool.into()), session_config)
            .await
            .expect("init redis session store failed");

    SessionSetup {
        layer: SessionLayer::new(session_store),
    }
}
