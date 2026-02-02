//! Guardian integration client
//! 
//! Connects to the Guardian service (default port 9109) to fetch status, 
//! update configuration, and stream live leak events.

use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, error, warn};
use reqwest::Client;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::StreamExt;

use crate::config::GuardianConfig;
use crate::db::{Database, LeakEvent};

/// Client for communicating with the Guardian service
pub struct GuardianClient {
    config: GuardianConfig,
    http_client: Client,
    db: Database,
    event_tx: broadcast::Sender<LeakEvent>,
    child: tokio::sync::Mutex<Option<tokio::process::Child>>,
}

impl GuardianClient {
    pub fn new(config: GuardianConfig, db: Database) -> (Arc<Self>, broadcast::Receiver<LeakEvent>) {
        let (event_tx, event_rx) = broadcast::channel(100);
        let client = Arc::new(Self {
            config,
            http_client: Client::new(),
            db,
            event_tx,
            child: tokio::sync::Mutex::new(None),
        });
        
        (client, event_rx)
    }

    /// Subscribe to real-time leak events
    pub fn subscribe(&self) -> broadcast::Receiver<LeakEvent> {
        self.event_tx.subscribe()
    }

    /// Primary entry point to start background monitoring
    pub fn spawn_worker(self: Arc<Self>) {
        let client = self.clone();
        tokio::spawn(async move {
            info!("Starting Guardian integration worker (API: {}:{})", 
                client.config.api_url, client.config.api_port);
            
            loop {
                if client.config.enabled {
                    if let Err(e) = client.monitor_stream().await {
                        error!("Guardian stream error: {}. Retrying in 10s...", e);
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        });
    }

    /// Maintain a WebSocket connection to Guardian
    /// Maintain a WebSocket connection to Guardian
    async fn monitor_stream(&self) -> anyhow::Result<()> {
        let ws_url = format!("ws://{}:{}/stream", self.config.api_url, self.config.api_port);
        info!("Connecting to Guardian WebSocket at {}", ws_url);

        loop {
            if !self.config.enabled {
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }

            match connect_async(&ws_url).await {
                Ok((ws_stream, _)) => {
                    info!("✅ Connected to Guardian stream");
                    let (_, mut read) = ws_stream.split();

                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                match serde_json::from_str::<LeakEvent>(&text) {
                                    Ok(event) => {
                                        if let Err(e) = self.handle_event(event).await {
                                            error!("Failed to handle event: {}", e);
                                        }
                                    }
                                    Err(e) => warn!("Failed to parse Guardian event: {}", e),
                                }
                            }
                            Ok(Message::Close(_)) => {
                                warn!("Guardian WebSocket closed connection");
                                break; 
                            }
                            Err(e) => {
                                error!("WebSocket error: {}", e);
                                break;
                            }
                            _ => {} // Ignore Ping/Pong/Binary for now
                        }
                    }
                    warn!("Disconnected from Guardian. Reconnecting in 5s...");
                }
                Err(e) => {
                    // Only log error if we expect it to be running (which we do if enabled)
                    // But to avoid spamming logs if service is down, maybe backoff
                    warn!("Failed to connect to Guardian: {}. Retrying in 5s...", e);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    /// Process a single leak event
    async fn handle_event(&self, event: LeakEvent) -> anyhow::Result<()> {
        info!("🚨 Leak detected: {} -> {}:{}", 
            event.process_name.as_deref().unwrap_or("unknown"),
            event.dest_ip.as_deref().unwrap_or("?"),
            event.dest_port.unwrap_or(0)
        );

        // 1. Persist to database if enabled
        if self.config.store_history {
            self.db.insert_leak_event(&event)?;
        }

        // 2. Broadcast to all active WebSocket clients
        let _ = self.event_tx.send(event);
        
        Ok(())
    }

    /// Fetch current status from Guardian
    pub async fn get_status(&self) -> anyhow::Result<serde_json::Value> {
        let url = format!("{}:{}/status", self.config.api_url, self.config.api_port);
        let resp = self.http_client.get(url).send().await?.json().await?;
        Ok(resp)
    }

    /// Update Guardian configuration
    pub async fn update_config(&self, new_config: serde_json::Value) -> anyhow::Result<()> {
        let url = format!("{}:{}/config", self.config.api_url, self.config.api_port);
        let response = self.http_client.post(url).json(&new_config).send().await?;
        
        if !response.status().is_success() {
            anyhow::bail!("Guardian config update failed: {}", response.status());
        }
        
        Ok(())
    }

    /// Start the Guardian service binary
    pub async fn start_service(&self) -> Result<(), String> {
        info!("Starting Guardian service...");
        let mut child_lock = self.child.lock().await;
        
        // Kill existing if any
        if let Some(mut old_child) = child_lock.take() {
            let _ = old_child.kill().await;
        }

        let child = tokio::process::Command::new("/Users/rds/antigravity/umbra/guardian/target/debug/guardian")
            .spawn();

        match child {
            Ok(c) => {
                *child_lock = Some(c);
                Ok(())
            }
            Err(e) => Err(format!("Failed to start Guardian: {}", e)),
        }
    }

    /// Check if the Guardian service is still running
    pub async fn is_running(&self) -> bool {
        let mut child_lock = self.child.lock().await;
        if let Some(child) = child_lock.as_mut() {
            match child.try_wait() {
                Ok(None) => true, // Still running
                _ => {
                    *child_lock = None;
                    false
                }
            }
        } else {
            false
        }
    }

    /// Stop the Guardian service
    pub fn stop_service(&self) -> Result<(), String> {
        info!("Stopping Guardian service...");
        let output = std::process::Command::new("killall")
            .arg("guardian")
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err("Guardian service not found or failed to stop".to_string())
        }
    }
}
