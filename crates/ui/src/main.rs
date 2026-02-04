use dioxus::logger;
use dioxus::logger::tracing::Level;
use dioxus::prelude::*;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum_session::{
        SameSite, SessionConfig, SessionLayer, SessionMode, SessionNullPool, SessionStore,
    };
    use dioxus_server::DioxusRouterExt;
    use std::sync::Arc;
    use ui::impls::error::api_error;

    logger::init(Level::INFO).expect("Logger init failed");

    error!("info init with Level::INFO success");

    let container = infra::setup::init_app_container()
        .await
        .map_err(api_error)
        .expect("init app container failed");

    let app_env = std::env::var("APP_ENV").ok();
    let server_addr = if app_env.is_none() {
        dioxus::cli_config::fullstack_address_or_localhost()
    } else {
        container.config.server_addr()
    };

    let is_production = std::env::var("APP_ENV").ok().as_deref() == Some("production");

    let session_config = SessionConfig::default()
        .with_session_name("bestofrs_session")
        .with_mode(SessionMode::OptIn)
        .with_cookie_same_site(SameSite::Lax)
        .with_http_only(true)
        .with_secure(is_production);

    let session_store = SessionStore::<SessionNullPool>::new(None, session_config)
        .await
        .expect("init session store failed");

    let app_state = Arc::new(container);
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new(), ui::root::App)
        .layer(axum::Extension(app_state.clone()))
        .layer(ui::impls::auth::auth_layer(app_state.clone()))
        .layer(SessionLayer::new(session_store));

    let listener = tokio::net::TcpListener::bind(server_addr)
        .await
        .expect("bind server addr failed");
    warn!("🚀 Server running on :{server_addr}");

    axum::serve(listener, router).await.expect("server error");
}

#[cfg(not(feature = "server"))]
fn main() {
    logger::init(Level::INFO).expect("Logger init failed");
    info!("info init with Level::INFO success wasm");

    dioxus::launch(ui::root::App);
}
