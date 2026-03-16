use app::auth::AuthUserCache;
use domain::{AuthUser, UserId};
use redis_pool::SingleRedisPool;

const REDIS_USER_PREFIX: &str = "auth:user:";
const REDIS_USER_TTL_SECONDS: u64 = 60 * 60 * 24 * 7; // 7 days

pub struct RedisAuthUserCache {
    pool: SingleRedisPool,
}

impl RedisAuthUserCache {
    pub fn new(pool: SingleRedisPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AuthUserCache for RedisAuthUserCache {
    async fn get(&self, user_id: &UserId) -> Option<AuthUser> {
        let key = format!("{REDIS_USER_PREFIX}{}", user_id.as_str());
        let mut conn = self.pool.acquire().await.ok()?;
        let json: String = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .ok()?;
        serde_json::from_str(&json).ok()
    }

    fn put(&self, user: &AuthUser) {
        let key = format!("{REDIS_USER_PREFIX}{}", user.user_id.as_str());
        let json = serde_json::to_string(user).unwrap_or_default();
        let pool = self.pool.clone();
        tokio::spawn(async move {
            let conn = match pool.acquire().await {
                Ok(c) => c,
                Err(e) => {
                    tracing::warn!(key = %key, error = %e, "failed to acquire redis connection for user cache put");
                    return;
                }
            };
            let mut conn = conn;
            if let Err(e) = redis::pipe()
                .atomic()
                .set(&key, &json)
                .ignore()
                .expire(&key, REDIS_USER_TTL_SECONDS as i64)
                .ignore()
                .query_async::<()>(&mut conn)
                .await
            {
                tracing::warn!(key = %key, error = %e, "failed to write user cache to redis");
            }
        });
    }
}
