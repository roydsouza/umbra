use std::sync::Arc;
use tokio::sync::RwLock;
use serde::Serialize;
use tracing::{info, error};
use super::NodeStatus;
use super::darkmatter::DarkMatterIntegration;
use super::monero::MoneroIntegration;

#[derive(Debug, Clone, Serialize, Default)]
pub struct CryptoStatus {
    pub zen: NodeStatus,
    pub monero: NodeStatus,
    pub last_updated: u64,
}

pub struct CryptoManager {
    zcash: DarkMatterIntegration,
    monero: MoneroIntegration,
    state: Arc<RwLock<CryptoStatus>>,
}

impl CryptoManager {
    pub fn new(zcash_url: &str, monero_url: &str) -> Self {
        Self {
            zcash: DarkMatterIntegration::new(zcash_url, false), // TODO: Tor toggle
            monero: MoneroIntegration::new(monero_url),
            state: Arc::new(RwLock::new(CryptoStatus::default())),
        }
    }

    pub async fn get_status(&self) -> CryptoStatus {
        self.state.read().await.clone()
    }

    pub fn spawn_worker(self: Arc<Self>) {
        tokio::spawn(async move {
            info!("Starting CryptoManager polling (Zcash & Monero)...");
            loop {
                let z_status = self.zcash.get_status().await;
                let m_status = self.monero.get_status_async().await;

                let mut lock = self.state.write().await;
                lock.zen = z_status;
                lock.monero = m_status;
                lock.last_updated = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                drop(lock); // Release lock before sleep

                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            }
        });
    }
}
