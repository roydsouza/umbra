use missioncontrol_core::{Config, Database};
use missioncontrol_core::arti::ArtiManager;
use missioncontrol_core::integrations::guardian::GuardianClient;
use missioncontrol_core::integrations::manager::CryptoManager;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AppState {
    pub config: Config,
    pub arti: RwLock<Option<ArtiManager>>,
    pub db: Database,
    pub guardian: Arc<GuardianClient>,
    pub crypto: Arc<CryptoManager>,
}
