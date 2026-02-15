pub mod api;
pub mod auth;
pub mod config;
pub mod db;
pub mod domain;
pub mod error;

use axum::routing::get;
use axum::{Router, response::Html};
use tower_http::trace::TraceLayer;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub config: Config,
}

async fn hello() -> Html<&'static str> {
    Html("<h1>Hello from CoachJan</h1>")
}

/// Build the full application router. Used by both `main.rs` and integration tests.
pub fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello))
        .nest("/api/auth", api::auth::router())
        .nest("/api/athlete", api::athletes::router())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
