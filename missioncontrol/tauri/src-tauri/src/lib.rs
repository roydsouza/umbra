#[cfg(mobile)]
mod mobile;
mod state;
mod commands;

use missioncontrol_core::{Config, Database};
use missioncontrol_core::arti::ArtiManager;
use missioncontrol_core::integrations::guardian::GuardianClient;
use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::{Manager, Emitter};
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
        commands::get_arti_status,
        commands::get_guardian_status,
        commands::get_system_stats,
        commands::get_circuits,
        commands::get_crypto_status,
        commands::start_darkmatter_node,
        commands::stop_darkmatter_node
    ])
    .setup(|app| {
      #[cfg(debug_assertions)]
      {
        // Log plugin removed for now to simplify build, using standard log
      }

      // Load Config - fail fast if config is bad
      let config = Config::load().expect("Failed to load configuration");
      
      // Initialize Database
      let db = Database::open(&config.server.database_path).expect("Failed to open database");

      // Initialize Guardian
      let (guardian, mut guardian_rx) = GuardianClient::new(config.guardian.clone(), db.clone());
      
      // Spawn Guardian Worker in Async Context
      let guardian_worker = guardian.clone();
      tauri::async_runtime::spawn(async move {
          guardian_worker.spawn_worker();
      });

      // Bridge Guardian Events to Tauri
      let app_handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
          while let Ok(event) = guardian_rx.recv().await {
              let _ = app_handle.emit("guardian://leak", event);
          }
      });

      // Initialize Crypto Manager
      let crypto = Arc::new(missioncontrol_core::integrations::manager::CryptoManager::new(
          &config.crypto.zcash_url,
          &config.crypto.monero_url
      ));
      
      // Spawn Crypto Worker in Async Context
      let crypto_worker = crypto.clone();
      tauri::async_runtime::spawn(async move {
          crypto_worker.spawn_worker();
      });

      // Manage State
      let state = AppState {
          config,
          arti: RwLock::new(None),
          db,
          guardian,
          crypto,
      };
      app.manage(Arc::new(state));

      // Bootstrap Arti in Background
      let handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
          match ArtiManager::bootstrap().await {
              Ok(manager) => {
                  println!("✅ Arti bootstrapped successfully");
                  if let Some(state) = handle.try_state::<Arc<AppState>>() {
                      let mut lock = state.arti.write().await;
                      *lock = Some(manager);
                  }
                  let _ = handle.emit("arti://ready", ());
              }
              Err(e) => {
                  println!("❌ Arti bootstrap failed: {}", e);
                  let _ = handle.emit("arti://error", e.to_string());
              }
          }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
