//! Configuration management for MissionControl
//!
//! Uses figment for layered configuration: defaults -> file -> env vars

use std::path::PathBuf;
use figment::{Figment, providers::{Format, Toml, Env}};
use serde::Deserialize;

/// MissionControl configuration
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Address to bind the web server (default: 127.0.0.1:3030)
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,
    
    /// Path to SQLite database
    #[serde(default = "default_database_path")]
    pub database_path: PathBuf,
    
    /// Path to configuration file (for display only)
    #[serde(skip)]
    pub config_path: PathBuf,
    
    /// Arti-specific configuration
    #[serde(default)]
    pub arti: ArtiConfig,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ArtiConfig {
    /// Path to Arti state directory
    #[serde(default = "default_arti_state_dir")]
    pub state_dir: PathBuf,
    
    /// Path to Arti cache directory  
    #[serde(default = "default_arti_cache_dir")]
    pub cache_dir: PathBuf,
}

fn default_listen_addr() -> String {
    "127.0.0.1:3030".to_string()
}

fn default_database_path() -> PathBuf {
    PathBuf::from("data/missioncontrol.db")
}

fn default_arti_state_dir() -> PathBuf {
    PathBuf::from("data/arti/state")
}

fn default_arti_cache_dir() -> PathBuf {
    PathBuf::from("data/arti/cache")
}

impl Config {
    /// Load configuration with priority: defaults < file < environment
    pub fn load() -> anyhow::Result<Self> {
        let config_path = PathBuf::from("config/missioncontrol.toml");
        
        let mut config: Config = Figment::new()
            // Start with defaults
            .merge(Toml::string(include_str!("../config/defaults.toml")))
            // Override with local config if exists
            .merge(Toml::file(&config_path))
            // Override with environment variables (MC_ prefix)
            .merge(Env::prefixed("MC_").split("_"))
            .extract()?;
        
        config.config_path = config_path;
        
        // Ensure data directories exist
        if let Some(parent) = config.database_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::create_dir_all(&config.arti.state_dir)?;
        std::fs::create_dir_all(&config.arti.cache_dir)?;
        
        Ok(config)
    }
}
