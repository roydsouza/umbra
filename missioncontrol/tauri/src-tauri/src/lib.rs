#[cfg(mobile)]
mod mobile;
mod state;
mod commands;

use missioncontrol_core::{Config, Database};
use missioncontrol_core::integrations::guardian::GuardianClient;
use std::sync::Arc;
use tokio::sync::RwLock;
use tauri::{Manager, Emitter};
use tracing::{info, warn, error};
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
        commands::start_zcash_node,
        commands::stop_zcash_node,
        commands::start_monero_node,
        commands::stop_monero_node,
        commands::start_guardian_service,
        commands::stop_guardian_service
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
      
      // Auto-start and supervise Guardian if enabled
      if config.guardian.enabled {
          let guardian_supervisor = guardian.clone();
          tauri::async_runtime::spawn(async move {
              let mut retry_delay = std::time::Duration::from_secs(2);
              let max_delay = std::time::Duration::from_secs(60);

              loop {
                  if !guardian_supervisor.is_running().await {
                      warn!("Guardian Supervisor: Service not running or crashed. Attempting restart...");
                      match guardian_supervisor.start_service().await {
                          Ok(_) => {
                              info!("✅ Guardian service started/restarted successfully");
                              retry_delay = std::time::Duration::from_secs(2);
                          }
                          Err(e) => {
                              error!("❌ Guardian restart failed: {}. Retrying in {:?}...", e, retry_delay);
                              retry_delay = std::cmp::min(retry_delay * 2, max_delay);
                          }
                      }
                  }
                  tokio::time::sleep(retry_delay).await;
              }
          });
      }
      
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
          &config.crypto.monero_url,
          guardian.clone()
      ));
      
      // Spawn Crypto Worker in Async Context
      let crypto_worker = crypto.clone();
      tauri::async_runtime::spawn(async move {
          crypto_worker.spawn_worker();
      });

      // Initialize Arti
      let arti_manager = tauri::async_runtime::block_on(async {
          missioncontrol_core::arti::ArtiManager::new().expect("Failed to initialize Arti")
      });
      let arti_worker = arti_manager.clone();

      // Manage State
      let state = AppState {
          config,
          arti: RwLock::new(Some(arti_manager)),
          db,
          guardian,
          crypto,
      };
      app.manage(Arc::new(state));

      // Bootstrap Arti with Supervisor Loop
      let handle = app.handle().clone();
      tauri::async_runtime::spawn(async move {
          let mut retry_delay = std::time::Duration::from_secs(2);
          let max_delay = std::time::Duration::from_secs(60);

          loop {
              info!("Arti Supervisor: Attempting bootstrap...");
              match arti_worker.bootstrap().await {
                  Ok(_) => {
                      info!("✅ Arti bootstrapped successfully");
                      let _ = handle.emit("arti://ready", ());
                      // Once bootstrapped, we stay in the loop but the bootstrap() call is done.
                      // In a more complex setup, we'd monitor the connection.
                      // For now, if bootstrap succeeds, we just wait a long time or exit worker.
                      break; 
                  }
                  Err(e) => {
                      error!("❌ Arti bootstrap failed: {}. Retrying in {:?}...", e, retry_delay);
                      let _ = handle.emit("arti://error", e.to_string());
                      tokio::time::sleep(retry_delay).await;
                      retry_delay = std::cmp::min(retry_delay * 2, max_delay);
                  }
              }
          }
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
