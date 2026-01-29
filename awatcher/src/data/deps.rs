use std::process::Command;

#[derive(Debug, Clone)]
pub struct DependentApp {
    pub name: String,
    pub pid: u32,
    pub connected: bool,
}

pub fn scan_dependent_apps() -> Vec<DependentApp> {
    // lsof -n -P -i:9050 -F pcn
    let output = Command::new("lsof")
        .args(["-n", "-P", "-i:9050", "-F", "pcn"])
        .output();

    let mut apps = Vec::new();

    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        // Output format:
        // p1234
        // ccurl
        // n...:9050
        
        let mut current_pid = 0;
        let mut current_name;

        for line in stdout.lines() {
            if line.starts_with('p') {
                if let Ok(pid) = line[1..].parse::<u32>() {
                    current_pid = pid;
                }
            } else if line.starts_with('c') {
                current_name = line[1..].to_string();
                if current_pid != 0 && !current_name.is_empty() && current_name != "arti" {
                    // Check if we already have this PID (lsof lists one entry per connection)
                    // We just want unique apps
                    if !apps.iter().any(|a: &DependentApp| a.pid == current_pid) {
                        apps.push(DependentApp {
                            name: current_name.clone(),
                            pid: current_pid,
                            connected: true,
                        });
                    }
                }
            }
        }
    }

    apps
}
