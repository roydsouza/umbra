//! Integration modules for external projects
//!
//! Provides connectors for DarkMatter, GravityLens, and future projects.

pub mod darkmatter;
pub mod guardian;

use serde::Serialize;

/// Common trait for all node integrations
pub trait NodeIntegration {
    /// Get the display name
    fn name(&self) -> &str;
    
    /// Get current status
    fn status(&self) -> NodeStatus;
    
    /// Check if the node is reachable
    async fn health_check(&self) -> bool;
}

/// Common node status structure
#[derive(Debug, Clone, Serialize)]
pub struct NodeStatus {
    pub connected: bool,
    pub synced: bool,
    pub block_height: Option<u64>,
    pub peers: Option<u32>,
    pub version: Option<String>,
}

impl Default for NodeStatus {
    fn default() -> Self {
        Self {
            connected: false,
            synced: false,
            block_height: None,
            peers: None,
            version: None,
        }
    }
}
