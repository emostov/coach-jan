use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use coachjan::{AppState, build_app};
use coachjan::config::Config;

#[tokio::main]
async fn main() {
    // Load .env file if present
    let _ = dotenvy::dotenv();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "coachjan=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    // Set up SQLite connection pool with foreign keys enabled
    let connect_options = SqliteConnectOptions::from_str(&config.database_url)
        .expect("Invalid DATABASE_URL")
        .create_if_missing(true)
        .pragma("foreign_keys", "ON");

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to SQLite");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database connected and migrations applied");

    let state = AppState {
        db,
        config: config.clone(),
    };

    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind(config.listen_addr())
        .await
        .expect("Failed to bind listener");

    tracing::info!("CoachJan listening on {}", config.listen_addr());

    axum::serve(listener, app).await.expect("Server error");
}
