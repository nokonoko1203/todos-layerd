use crate::handler::{health::health_check, todo::*};
use crate::module::Modules;
use axum::{extract::Extension, routing::get, Router};
use std::{env, sync::Arc};

// All the setup code is in this file
pub async fn create_app(modules: Arc<Modules>) {
    let hc_router = Router::new().route("/", get(health_check));
    let todo_router = Router::new()
        .route("/", get(all_todo).post(create_todo))
        .route(
            "/:id",
            get(find_todo).delete(delete_todo).patch(update_todo),
        );

    let app = Router::new()
        .nest("/", hc_router)
        .nest("/todo", todo_router)
        .layer(Extension(modules));

    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    tracing::debug!("Starting server on");
    tracing::debug!("http://localhost:8080");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
