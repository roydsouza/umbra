//! REST API handlers
//!
//! JSON API for integration with other projects (DarkMatter, GravityLens).

use std::sync::Arc;
use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use crate::AppState;

/// Build the API router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/status", get(status))
        .route("/arti/status", get(arti_status))
}

#[derive(Serialize)]
struct StatusResponse {
    version: String,
    uptime_seconds: u64,
    arti_ready: bool,
}

/// GET /api/status - Overall system status
async fn status(State(state): State<Arc<AppState>>) -> Json<StatusResponse> {
    let arti_lock = state.arti.read().await;
    Json(StatusResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0, // TODO: Track actual uptime
        arti_ready: arti_lock.is_some(),
    })
}

#[derive(Serialize)]
struct ArtiStatusResponse {
    bootstrapped: bool,
    percentage: u8,
    message: String,
    circuit_count: usize,
}

/// GET /api/arti/status - Arti client status
async fn arti_status(State(state): State<Arc<AppState>>) -> Json<ArtiStatusResponse> {
    let arti_lock = state.arti.read().await;
    match &*arti_lock {
        Some(manager) => {
            let status = manager.status();
            Json(ArtiStatusResponse {
                bootstrapped: status.bootstrapped,
                percentage: status.percentage,
                message: status.message,
                circuit_count: manager.circuit_count(),
            })
        }
        None => Json(ArtiStatusResponse {
            bootstrapped: false,
            percentage: 0,
            message: "Bootstrapping...".to_string(),
            circuit_count: 0,
        }),
    }
}
