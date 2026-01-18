//! Embedded Arti client management
//!
//! Provides a wrapper around arti-client for Tor network access.

pub mod onion;

use arti_client::{TorClient, TorClientConfig, DataStream};
use tor_rtcompat::PreferredRuntime;
use tracing::{info, debug};

/// Manages the embedded Arti Tor client
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

impl ArtiManager {
    /// Bootstrap a new Arti client
    /// 
    /// This creates an isolated Tor client that doesn't interfere
    /// with the external umbra/bin/arti instance.
    pub async fn bootstrap() -> anyhow::Result<Self> {
        info!("Creating Arti configuration...");
        
        // Use default config for now - can be customized later
        let config = TorClientConfig::default();
        
        info!("Bootstrapping Tor client (this may take 30-60 seconds)...");
        let client = TorClient::create_bootstrapped(config).await?;
        
        info!("Tor client bootstrapped successfully");
        
        Ok(Self { client })
    }
    
    /// Get current bootstrap status
    pub fn status(&self) -> BootstrapStatus {
        // In a real implementation, we'd query the client's bootstrap state
        BootstrapStatus {
            bootstrapped: true,
            percentage: 100,
            message: "Connected to Tor network".to_string(),
        }
    }
    
    /// Create an isolated client for a specific project
    /// 
    /// This ensures circuit isolation between GravityLens and DarkMatter.
    pub fn isolated_client(&self) -> TorClient<PreferredRuntime> {
        self.client.isolated_client()
    }
    
    /// Connect to a remote host through Tor
    pub async fn connect(&self, host: &str, port: u16) -> anyhow::Result<DataStream> {
        debug!("Connecting to {}:{} through Tor", host, port);
        let stream = self.client.connect((host, port)).await?;
        Ok(stream)
    }
    
    /// Get circuit count (placeholder - actual implementation needs tor-circmgr)
    pub fn circuit_count(&self) -> usize {
        // TODO: Query actual circuit manager
        3
    }
}
