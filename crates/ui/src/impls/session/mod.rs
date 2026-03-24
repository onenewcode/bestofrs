#[cfg(feature = "server")]
pub type AppSession = axum_session::Session<axum_session_redispool::SessionRedisPool>;

#[cfg(not(feature = "server"))]
pub type AppSession = ();

pub mod auth;
pub mod consts;
pub mod preference;

#[cfg(feature = "server")]
mod setup;

#[cfg(feature = "server")]
pub use setup::*;
