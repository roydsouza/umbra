use sysinfo::{System, ProcessRefreshKind, RefreshKind};

#[derive(Debug, Clone, Default)]
pub struct ArtiStatus {
    pub running: bool,
    pub pid: Option<u32>,
    pub memory_mb: f64,
    pub cpu_percent: f32,
}

pub struct ArtiMonitor {
    system: System,
}

impl ArtiMonitor {
    pub fn new() -> Self {
        Self {
            system: System::new_with_specifics(
                RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
            ),
        }
    }

    pub fn check(&mut self) -> ArtiStatus {
        self.system.refresh_processes();
        
        // Find process named "arti"
        // Note: sysinfo process names might be case sensitive or contain path
        for (pid, process) in self.system.processes() {
            let name = process.name().to_lowercase();
            if name.contains("arti") {
                // Found it
                return ArtiStatus {
                    running: true,
                    pid: Some(pid.as_u32()),
                    memory_mb: process.memory() as f64 / 1024.0 / 1024.0,
                    cpu_percent: process.cpu_usage(),
                };
            }
        }
        
        ArtiStatus {
            running: false,
            pid: None,
            memory_mb: 0.0,
            cpu_percent: 0.0,
        }
    }
}
