#[cfg(feature = "server")]
pub type AppStateExt = axum::extract::Extension<std::sync::Arc<infra::setup::AppState>>;

#[cfg(not(feature = "server"))]
pub struct AppStateExt;
