//! Monero Integration
//!
//! Connects to Monerod RPC to fetch status.

use super::{NodeIntegration, NodeStatus};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};
use reqwest::Client;

/// Monero integration client
pub struct MoneroIntegration {
    rpc_url: String,
    client: Client,
}

#[derive(Serialize)]
struct JsonRpcRequest<'a> {
    jsonrpc: &'a str,
    id: &'a str,
    method: &'a str,
    params: serde_json::Value,
}

#[derive(Deserialize)]
struct MoneroInfoResponse {
    result: Option<MoneroInfoResult>,
}

#[derive(Deserialize)]
struct MoneroInfoResult {
    height: u64,
    target_height: Option<u64>,
    outgoing_connections_count: u32,
    incoming_connections_count: u32,
    status: String,
    version: String,
}

impl MoneroIntegration {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            client: Client::new(),
        }
    }

    async fn fetch_info(&self) -> anyhow::Result<MoneroInfoResult> {
        let body = JsonRpcRequest {
            jsonrpc: "2.0",
            id: "0",
            method: "get_info",
            params: serde_json::json!({}),
        };

        let resp = self.client.post(&format!("{}/json_rpc", self.rpc_url))
            .json(&body)
            .send()
            .await?
            .json::<MoneroInfoResponse>()
            .await?;
            
        if let Some(result) = resp.result {
            Ok(result)
        } else {
            anyhow::bail!("No result in Monero RPC response")
        }
    }
}

impl NodeIntegration for MoneroIntegration {
    fn name(&self) -> &str {
        "Monero"
    }

    async fn health_check(&self) -> bool {
        self.fetch_info().await.is_ok()
    }

    fn status(&self) -> NodeStatus {
        // This trait method is synchronous but our fetch is async.
        // We might need to refactor the trait or use internal state.
        // For now, returning default - the manager will likely call async methods directly.
        NodeStatus::default()
    }
}

impl MoneroIntegration {
    pub async fn get_status_async(&self) -> NodeStatus {
        match self.fetch_info().await {
            Ok(info) => {
                let synced = if let Some(target) = info.target_height {
                    info.height >= target.saturating_sub(10) // Approx sync
                } else {
                    true // Assume synced if no target
                };

                NodeStatus {
                    connected: true,
                    synced,
                    block_height: Some(info.height),
                    peers: Some(info.outgoing_connections_count + info.incoming_connections_count),
                    version: Some(info.version),
                }
            },
            Err(e) => {
                warn!("Failed to fetch Monero info: {}", e);
                NodeStatus::default()
            }
        }
    }
}
