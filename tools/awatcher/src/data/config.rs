use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct ArtiConfig {
    pub onion_services: Option<HashMap<String, OnionServiceConfig>>,
}

#[derive(Deserialize, Debug)]
pub struct OnionServiceConfig {
    pub proxy_ports: Vec<Vec<serde_json::Value>>, // e.g. [ [80, "127.0.0.1:8080"] ] - messy tomtl parsing
}

// Actually arti.toml structure for onion services involves:
// [onion_services."service-name"]
// proxy_ports = [ [80, "127.0.0.1:1234"] ]
// It's a bit complex. Let's simplify and just look for keys.

pub fn read_onion_services(path: PathBuf) -> Vec<String> {
    // Simple greedy parse or use toml value
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let value = match content.parse::<toml::Value>() {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut services = Vec::new();
    
    if let Some(os) = value.get("onion_services") {
        if let Some(table) = os.as_table() {
            for (name, _) in table {
                services.push(name.clone());
            }
        }
    }
    
    services
}
