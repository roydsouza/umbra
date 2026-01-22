use std::sync::Arc;
use tokio::sync::RwLock;
use serde::Serialize;
use tracing::info;
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
    guardian: Arc<crate::integrations::guardian::GuardianClient>,
    state: Arc<RwLock<CryptoStatus>>,
}

impl CryptoManager {
    pub fn new(zcash_url: &str, monero_url: &str, guardian: Arc<crate::integrations::guardian::GuardianClient>) -> Self {
        Self {
            zcash: DarkMatterIntegration::new(zcash_url, false), // TODO: Tor toggle
            monero: MoneroIntegration::new(monero_url),
            guardian,
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
    pub fn start_zcash(&self) -> Result<(), String> {
        self.zcash.start_node()
    }

    pub fn stop_zcash(&self) -> Result<(), String> {
        self.zcash.stop_node()
    }

    pub fn start_monero(&self) -> Result<(), String> {
        self.monero.start_node()
    }

    pub fn stop_monero(&self) -> Result<(), String> {
        self.monero.stop_node()
    }

    pub async fn start_guardian(&self) -> Result<(), String> {
        self.guardian.start_service().await
    }

    pub fn stop_guardian(&self) -> Result<(), String> {
        self.guardian.stop_service()
    }
}
