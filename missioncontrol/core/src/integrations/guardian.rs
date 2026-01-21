//! Guardian integration client
//! 
//! Connects to the Guardian service (default port 9109) to fetch status, 
//! update configuration, and stream live leak events.

use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, error, warn};
use reqwest::Client;

use crate::config::GuardianConfig;
use crate::db::{Database, LeakEvent};

/// Client for communicating with the Guardian service
pub struct GuardianClient {
    config: GuardianConfig,
    http_client: Client,
    db: Database,
    event_tx: broadcast::Sender<LeakEvent>,
}

impl GuardianClient {
    pub fn new(config: GuardianConfig, db: Database) -> (Arc<Self>, broadcast::Receiver<LeakEvent>) {
        let (event_tx, event_rx) = broadcast::channel(100);
        let client = Arc::new(Self {
            config,
            http_client: Client::new(),
            db,
            event_tx,
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
    async fn monitor_stream(&self) -> anyhow::Result<()> {
        info!("Connecting to Guardian WebSocket at {}:{}/stream", self.config.api_url, self.config.api_port);
        
        // --- REAL IMPLEMENTATION WILL GO HERE ---
        // For now, let's simulate receiving an event every 30 seconds if enabled
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            
            if self.config.enabled {
                let mock_event = LeakEvent {
                    id: None,
                    timestamp: None,
                    process_name: Some("zebrad".to_string()),
                    process_pid: Some(1234),
                    process_path: Some("/usr/local/bin/zebrad".to_string()),
                    dest_ip: Some("8.8.8.8".to_string()),
                    dest_port: Some(53),
                    severity: "CRITICAL".to_string(),
                    event_type: Some("dns".to_string()),
                    binary_info: None,
                };
                
                if let Err(e) = self.handle_event(mock_event).await {
                    error!("Failed to handle mock leak event: {}", e);
                }
            }
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
}
