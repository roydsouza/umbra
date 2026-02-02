use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Start,
    Stop,
    Restart,
}

impl Action {
    pub fn execute(&self) -> bool {
        match self {
            Action::Start => {
                let bin = "/Users/rds/antigravity/umbra/bin/arti";
                let cfg = "/Users/rds/antigravity/umbra/arti.toml";
                let log = "/Users/rds/antigravity/umbra/var/log/arti.log";
                
                let cmd = format!("{} proxy -c {} >> {} 2>&1", bin, cfg, log);
                
                Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .spawn()
                    .is_ok()
            }
            Action::Stop => {
                // pkill is reliable
                Command::new("pkill")
                    .arg("arti")
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            }
            Action::Restart => {
                let _ = Command::new("pkill").arg("arti").status();
                std::thread::sleep(std::time::Duration::from_secs(1));
                
                let bin = "/Users/rds/antigravity/umbra/bin/arti";
                let cfg = "/Users/rds/antigravity/umbra/arti.toml";
                let log = "/Users/rds/antigravity/umbra/var/log/arti.log";
                
                let cmd = format!("{} proxy -c {} >> {} 2>&1", bin, cfg, log);
                
                Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .spawn()
                    .is_ok()
            }
        }
    }
}
