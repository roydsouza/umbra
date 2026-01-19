//! REST API handlers
//!
//! JSON API for integration with other projects (DarkMatter, GravityLens).

use std::sync::Arc;
use axum::{
    extract::{State, Query, ws::{WebSocket, WebSocketUpgrade, Message}},
    routing::{get, post},
    response::IntoResponse,
    Json, Router,
};
use serde::{Serialize, Deserialize};
use crate::AppState;

/// Build the API router
pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/status", get(status))
        .route("/arti/status", get(arti_status))
        // Guardian endpoints
        .route("/guardian/status", get(guardian_status))
        .route("/guardian/history", get(guardian_history))
        .route("/guardian/config", post(update_guardian_config))
        .route("/guardian/stream", get(guardian_stream))
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

/// GET /api/guardian/status - Guardian service status
async fn guardian_status(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    match state.guardian.get_status().await {
        Ok(status) => Json(status),
        Err(e) => {
            error!("Failed to fetch Guardian status: {}", e);
            Json(serde_json::json!({ "error": e.to_string(), "connected": false }))
        }
    }
}

#[derive(Deserialize)]
struct HistoryQuery {
    limit: Option<usize>,
}

/// GET /api/guardian/history - Historical leak events from local DB
async fn guardian_history(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HistoryQuery>,
) -> Json<Vec<crate::db::LeakEvent>> {
    let limit = query.limit.unwrap_or(50);
    match state.db.get_leak_history(limit) {
        Ok(history) => Json(history),
        Err(e) => {
            error!("Failed to fetch leak history: {}", e);
            Json(vec![])
        }
    }
}

/// POST /api/guardian/config - Update Guardian configuration
async fn update_guardian_config(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    match state.guardian.update_config(payload).await {
        Ok(_) => Json(serde_json::json!({ "status": "success" })),
        Err(e) => {
            error!("Failed to update Guardian config: {}", e);
            Json(serde_json::json!({ "status": "error", "message": e.to_string() }))
        }
    }
}

/// GET /api/guardian/stream - Live leak event WebSocket
async fn guardian_stream(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_guardian_socket(socket, state))
}

async fn handle_guardian_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut rx = state.guardian.subscribe();
    
    info!("New Guardian stream subscriber connected");

    loop {
        tokio::select! {
            // Forward events from the broadcast channel to the WebSocket
            Ok(event) = rx.recv() => {
                let msg = match serde_json::to_string(&event) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                if socket.send(Message::Text(msg.into())).await.is_err() {
                    break;
                }
            }
            // Handle client disconnect or pings (optional)
            msg = socket.recv() => {
                if msg.is_none() || matches!(msg, Some(Ok(Message::Close(_)))) {
                    break;
                }
            }
        }
    }
    
    info!("Guardian stream subscriber disconnected");
}

use tracing::{info, error};
