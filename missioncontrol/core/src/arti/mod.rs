//! Embedded Arti client management
//!
//! Provides a wrapper around arti-client for Tor network access.

pub mod onion;

use arti_client::{TorClient, TorClientConfig, DataStream};
use tor_rtcompat::PreferredRuntime;
use tracing::{info, debug};

/// Manages the embedded Arti Tor client
#[derive(Clone)]
pub struct ArtiManager {
    client: TorClient<PreferredRuntime>,
}

/// Bootstrap status for UI display
#[derive(Debug, Clone, serde::Serialize)]
pub struct BootstrapStatus {
    pub bootstrapped: bool,
    pub percentage: u8,
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CircuitNode {
    pub fingerprint: String,
    pub nickname: String,
    pub country: String,
    pub role: String, // "Guard", "Middle", "Exit"
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CircuitInfo {
    pub id: String,
    pub state: String, // "Ready", "Building"
    pub path: Vec<CircuitNode>,
    pub age_seconds: u64,
}

impl ArtiManager {
    /// Create a new Arti manager and start bootstrapping in the background
    pub fn new() -> anyhow::Result<Self> {
        info!("Initializing Arti client...");
        let config = TorClientConfig::default();
        let client = TorClient::create(config)?;
        
        Ok(Self { client })
    }

    /// Start the bootstrap process
    pub async fn bootstrap(&self) -> anyhow::Result<()> {
        info!("Bootstrapping Tor client...");
        self.client.bootstrap().await?;
        info!("Tor client bootstrapped successfully");
        Ok(())
    }
    
    /// Get current bootstrap status
    pub fn status(&self) -> BootstrapStatus {
        let status = self.client.bootstrap_status();
        BootstrapStatus {
            bootstrapped: status.ready(),
            percentage: (status.progress() * 100.0) as u8,
            message: status.to_string(),
        }
    }
    
    /// Create an isolated client for a specific project
    pub fn isolated_client(&self) -> TorClient<PreferredRuntime> {
        self.client.isolated_client()
    }
    
    /// Connect to a remote host through Tor
    pub async fn connect(&self, host: &str, port: u16) -> anyhow::Result<DataStream> {
        debug!("Connecting to {}:{} through Tor", host, port);
        let stream = self.client.connect((host, port)).await?;
        Ok(stream)
    }
    
    /// Get active circuit count
    pub fn circuit_count(&self) -> usize {
        self.get_circuits().len()
    }

    /// Get active circuits with real metadata
    pub fn get_circuits(&self) -> Vec<CircuitInfo> {
        let circmgr = self.client.circmgr();
        
        // Attempt to get the latest network directory for relay lookups
        let netdir = self.client.dirmgr().opt_netdir();

        circmgr.circuits().into_iter().map(|circ| {
            let state = if circ.is_closing() { "Closing" } else { "Ready" };
            
            let path = circ.path().iter().map(|hop| {
                let fingerprint = hop.display_relay_id().to_string();
                let mut nickname = "Unknown".to_string();
                let mut country = "??".to_string();
                
                // If we have a netdir, look up the relay's actual nickname and country
                if let Some(nd) = &netdir {
                    if let Some(relay) = nd.by_ids(hop) {
                        nickname = relay.nickname().to_string();
                        // Country code lookup usually requires geoip feature, 
                        // but nickname is a good start.
                    }
                }

                CircuitNode {
                    fingerprint,
                    nickname,
                    country,
                    role: "Relay".to_string(), // Simplified role
                }
            }).collect();

            CircuitInfo {
                id: format!("{:?}", circ.unique_id()),
                state: state.to_string(),
                age_seconds: 0, // Age not easily accessible from circmgr::Circuit
                path,
            }
        }).collect()
    }
}
