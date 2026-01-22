use dioxus::logger;
use dioxus::logger::tracing::Level;
use dioxus::prelude::*;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use dioxus_server::DioxusRouterExt;
    use std::sync::Arc;
    use ui::IO::api_error::api_error;

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

    let app_state = Arc::new(container);
    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::new(), ui::root::App)
        .layer(axum::Extension(app_state));

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
