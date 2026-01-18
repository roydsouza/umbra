//! MissionControl: Secure Arti-based Router & Dashboard
//!
//! A privacy-first command center for the AntiGravity ecosystem.
//! Provides Arti management, network monitoring, and project integration APIs.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::State,
    http::HeaderValue,
    response::Html,
    routing::get,
    Router,
};
use tokio::sync::RwLock;
use tower_http::set_header::SetResponseHeaderLayer;
use tracing::{info, warn, Level};
use tracing_subscriber::EnvFilter;

mod arti;
mod config;
mod db;
mod integrations;
mod web;

use crate::arti::ArtiManager;
use crate::config::Config;
use crate::db::Database;

/// Shared application state
pub struct AppState {
    pub config: Config,
    pub arti: RwLock<Option<ArtiManager>>,
    pub db: Database,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let db = Database::open(&config.database_path)?;
        
        Ok(Self {
            config,
            arti: RwLock::new(None),
            db,
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing with environment filter
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(Level::INFO.into())
        )
        .init();

    info!("🎛️  MissionControl v{}", env!("CARGO_PKG_VERSION"));
    info!("Starting secure Arti-based dashboard...");

    // Load configuration
    let config = Config::load()?;
    info!("Configuration loaded from: {:?}", config.config_path);

    // Initialize application state
    let state = Arc::new(AppState::new(config.clone()).await?);
    info!("Database initialized at: {:?}", config.database_path);

    // Bootstrap Arti in background (non-blocking startup)
    let arti_state = state.clone();
    tokio::spawn(async move {
        info!("Bootstrapping embedded Arti client...");
        match ArtiManager::bootstrap().await {
            Ok(manager) => {
                info!("✅ Arti bootstrapped successfully");
                let mut lock = arti_state.arti.write().await;
                *lock = Some(manager);
            }
            Err(e) => {
                warn!("⚠️  Arti bootstrap failed: {}. Dashboard available but Tor features disabled.", e);
            }
        }
    });

    // Build router with security middleware
    let app = Router::new()
        // Web pages
        .route("/", get(web::pages::home))
        .route("/circuits", get(web::pages::circuits))
        .route("/services", get(web::pages::services))
        .route("/metrics", get(web::pages::metrics))
        .route("/integrations", get(web::pages::integrations))
        .route("/config", get(web::pages::config_page))
        // API endpoints
        .nest("/api", web::api::router())
        // Static assets
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        // Security: Strip identifying headers
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::SERVER,
            HeaderValue::from_static(""),
        ))
        .with_state(state);

    // Bind to localhost only (security requirement)
    let addr: SocketAddr = config.listen_addr.parse()?;
    info!("🔒 Binding to {} (localhost only)", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("🚀 MissionControl ready at http://{}", addr);

    // Graceful shutdown on Ctrl+C
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("👋 MissionControl shutdown complete");
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    info!("Received shutdown signal...");
}
