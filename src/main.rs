use axum::{
    http::Method,
    response::Json,
    routing::get,
    Router,
};
use serde_json::json;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rust_crud_api::{
    config::Config,
    db::connect,
    routes::{user_route, api_docs, swagger_ui},
    services::user_service::UserService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_crud_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded successfully");

    // Connect to database
    let db = connect(&config).await?;
    tracing::info!("Database connected successfully");

    // Create user service
    let user_service = UserService::new(db);

    // Build CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    // Create the application router
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .merge(api_docs::docs_routes())
        .merge(swagger_ui::swagger_ui_routes())
        .nest("/api", user_route::user_routes().with_state(user_service))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        );

    // Start the server
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!(
        "Server running on {}:{}",
        config.server_host,
        config.server_port
    );

    axum::Server::from_tcp(listener.into_std()?)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Health check endpoint
/// 
/// Returns the current status of the API server.
async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "message": "Rust CRUD API is running",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
