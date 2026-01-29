use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone)]
pub struct OnionService {
    pub name: String,
    pub hostname: Option<String>,
    pub status: OnionStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OnionStatus {
    Active,
    Pending,
    Error,
}

pub struct OnionScanner {
    base_path: PathBuf,
}

impl OnionScanner {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    pub fn scan(&self, service_names: &[String]) -> Vec<OnionService> {
        let mut results = Vec::new();
        
        for name in service_names {
            let hostname_path = self.base_path.join(name).join("hostname");
            let (hostname, status) = if hostname_path.exists() {
                match fs::read_to_string(hostname_path) {
                    Ok(h) => (Some(h.trim().to_string()), OnionStatus::Active),
                    Err(_) => (None, OnionStatus::Error),
                }
            } else {
                (None, OnionStatus::Pending)
            };
            
            results.push(OnionService {
                name: name.clone(),
                hostname,
                status,
            });
        }
        
        results
    }
}
