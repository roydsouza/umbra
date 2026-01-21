//! DarkMatter Integration
//!
//! Connects to Zebra (Zcash node) metrics and provides status for the dashboard.
//! Designed to be extensible for additional crypto nodes.

use super::NodeStatus;
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

/// DarkMatter integration for Zcash/Zebra node
pub struct DarkMatterIntegration {
    /// Base URL for Zebra metrics (default: http://127.0.0.1:9999)
    metrics_url: String,
    /// Whether to route through Tor (for remote nodes)
    #[allow(dead_code)]
    use_tor: bool,
}

impl DarkMatterIntegration {
    pub fn new(metrics_url: &str, use_tor: bool) -> Self {
        Self {
            metrics_url: metrics_url.to_string(),
            use_tor,
        }
    }
    
    /// Fetch raw Prometheus metrics from Zebra
    pub async fn fetch_metrics(&self) -> anyhow::Result<String> {
        debug!("Fetching DarkMatter metrics from {}", self.metrics_url);
        
        // For now, direct HTTP. TODO: Route through Arti when use_tor is true
        let response = reqwest::get(&format!("{}/metrics", self.metrics_url)).await?;
        let body = response.text().await?;
        
        Ok(body)
    }
    
    /// Parse Zebra metrics into structured status
    pub async fn get_status(&self) -> NodeStatus {
        match self.fetch_metrics().await {
            Ok(metrics) => self.parse_metrics(&metrics),
            Err(e) => {
                warn!("Failed to fetch DarkMatter metrics: {}", e);
                NodeStatus::default()
            }
        }
    }
    
    /// Parse Prometheus metrics text format
    fn parse_metrics(&self, metrics: &str) -> NodeStatus {
        let mut status = NodeStatus {
            connected: true,
            ..Default::default()
        };
        
        for line in metrics.lines() {
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            
            // Parse key metrics
            if line.starts_with("zebra_state_best_block_height") {
                if let Some(value) = extract_value(line) {
                    status.block_height = Some(value as u64);
                }
            } else if line.starts_with("zebra_network_peer_connections") {
                if let Some(value) = extract_value(line) {
                    status.peers = Some(value as u32);
                }
            } else if line.starts_with("zebra_sync_is_synced") {
                if let Some(value) = extract_value(line) {
                    status.synced = value > 0.0;
                }
            }
        }
        
        status.version = Some("Zebra (Zcash)".to_string());
        status
    }
}

/// Extract numeric value from Prometheus metric line
fn extract_value(line: &str) -> Option<f64> {
    line.split_whitespace()
        .last()
        .and_then(|v| v.parse().ok())
}

/// Supported crypto nodes for future expansion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CryptoNode {
    Zcash,      // Zebra
    Bitcoin,    // Bitcoin Core (future)
    Ethereum,   // Geth/Reth (future)
    Monero,     // Monerod (future)
}

impl CryptoNode {
    pub fn name(&self) -> &str {
        match self {
            CryptoNode::Zcash => "Zcash (Zebra)",
            CryptoNode::Bitcoin => "Bitcoin Core",
            CryptoNode::Ethereum => "Ethereum (Geth)",
            CryptoNode::Monero => "Monero",
        }
    }
    
    pub fn icon(&self) -> &str {
        match self {
            CryptoNode::Zcash => "🛡️",
            CryptoNode::Bitcoin => "₿",
            CryptoNode::Ethereum => "⟠",
            CryptoNode::Monero => "ɱ",
        }
    }
}
