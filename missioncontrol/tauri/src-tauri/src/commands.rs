use crate::state::AppState;
use tauri::State;
use std::sync::Arc;

#[derive(serde::Serialize)]
pub struct ArtiStatusResponse {
    pub bootstrapped: bool,
    pub percentage: u8,
    pub message: String,
    pub circuit_count: usize,
}

#[derive(serde::Serialize)]
pub struct SystemStats {
    pub uptime_seconds: u64,
    pub arti_ready: bool,
    pub guardian_connected: bool,
    pub leaks_detected: usize,
}

#[tauri::command]
pub async fn get_arti_status(state: State<'_, Arc<AppState>>) -> Result<ArtiStatusResponse, String> {
    // Check if system Arti is running on port 9050
    let system_arti_active = std::net::TcpStream::connect("127.0.0.1:9050").is_ok();

    if system_arti_active {
        return Ok(ArtiStatusResponse {
            bootstrapped: true,
            percentage: 100,
            message: "System Service Active".to_string(),
            circuit_count: 0, // TODO: Connect to RPC for real count
        });
    }

    let arti_lock = state.arti.read().await;
    match &*arti_lock {
        Some(manager) => {
            let status = manager.status();
            Ok(ArtiStatusResponse {
                bootstrapped: status.bootstrapped,
                percentage: status.percentage,
                message: status.message,
                circuit_count: manager.circuit_count(),
            })
        }
        None => Ok(ArtiStatusResponse {
            bootstrapped: false,
            percentage: 0,
            message: "Bootstrapping...".to_string(),
            circuit_count: 0,
        }),
    }
}

#[tauri::command]
pub async fn get_guardian_status(state: State<'_, Arc<AppState>>) -> Result<serde_json::Value, String> {
    state.guardian.get_status().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_system_stats(state: State<'_, Arc<AppState>>) -> Result<SystemStats, String> {
    let arti_ready = state.arti.read().await.is_some();
    // In a real app, track uptime properly. For now, just 0/placeholder.
    let uptime = 0; 
    
    // Check Guardian connection by getting status and checking process state
    let guardian_running = state.guardian.is_running().await;
    let guardian_status = state.guardian.get_status().await;
    let guardian_connected = guardian_running && guardian_status.is_ok();
    
    // Get recent leaks count (mocked or from db)
    // let leaks = state.db.get_recent_leaks_count().unwrap_or(0);
    let leaks = 0; // DB method needs to be exposed first

    Ok(SystemStats {
        uptime_seconds: uptime,
        arti_ready,
        guardian_connected,
        leaks_detected: leaks,
    })
}

#[tauri::command]
pub async fn get_circuits(state: State<'_, Arc<AppState>>) -> Result<Vec<missioncontrol_core::arti::CircuitInfo>, String> {
    let arti_lock = state.arti.read().await;
    match &*arti_lock {
        Some(manager) => Ok(manager.get_circuits()),
        None => Ok(vec![]),
    }
}

#[tauri::command]
pub async fn get_crypto_status(state: State<'_, Arc<AppState>>) -> Result<missioncontrol_core::integrations::manager::CryptoStatus, String> {
    Ok(state.crypto.get_status().await)
}

#[tauri::command]
pub async fn start_zcash_node(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.crypto.start_zcash()
}

#[tauri::command]
pub async fn stop_zcash_node(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.crypto.stop_zcash()
}

#[tauri::command]
pub async fn start_monero_node(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.crypto.start_monero()
}

#[tauri::command]
pub async fn stop_monero_node(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.crypto.stop_monero()
}

#[tauri::command]
pub async fn start_guardian_service(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.crypto.start_guardian().await
}

#[tauri::command]
pub async fn stop_guardian_service(state: State<'_, Arc<AppState>>) -> Result<(), String> {
    state.crypto.stop_guardian()
}
