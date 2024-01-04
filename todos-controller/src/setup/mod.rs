use axum::{extract::Extension, routing::get, Router};
use std::{env, sync::Arc};

use crate::{handler::health::health_check, module::Modules};

pub async fn create_app(modules: Arc<Modules>) {
    let hc_router = Router::new().route("/", get(health_check));

    let app = Router::new()
        .nest("/hc", hc_router)
        .layer(Extension(modules));

    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    tracing::debug!("Starting server on");
    tracing::debug!("http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
