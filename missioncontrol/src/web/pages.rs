//! Web page handlers
//!
//! SSR HTML pages with GravityLens-inspired dark theme.

use std::sync::Arc;
use axum::{extract::State, response::Html};
use crate::AppState;

/// Render the HTML shell with navigation
fn render_page(title: &str, content: &str) -> String {
    format!(r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="description" content="MissionControl - Secure Arti Dashboard">
    <meta name="theme-color" content="#0a0a1a">
    <title>{title} | MissionControl</title>
    <link rel="stylesheet" href="/static/style/main.css">
    <link rel="preload" href="/static/fonts/inter.woff2" as="font" type="font/woff2" crossorigin>
</head>
<body>
    <div class="app-container">
        <nav class="nav">
            <div class="nav-brand">
                <a href="/" class="brand-link">
                    <span class="brand-icon">🎛️</span>
                    <span class="brand-text">MissionControl</span>
                </a>
            </div>
            <div class="nav-links">
                <a href="/" class="nav-link">Home</a>
                <a href="/circuits" class="nav-link">Circuits</a>
                <a href="/services" class="nav-link">Services</a>
                <a href="/metrics" class="nav-link">Metrics</a>
                <a href="/integrations" class="nav-link" style="color: #a855f7;">Integrations</a>
                <a href="/config" class="nav-link">Config</a>
            </div>
        </nav>
        <div class="content">
            {content}
        </div>
    </div>
</body>
</html>"###, title = title, content = content)
}

/// Home dashboard page
pub async fn home(State(state): State<Arc<AppState>>) -> Html<String> {
    let arti_lock = state.arti.read().await;
    let (arti_status, circuit_count) = match &*arti_lock {
        Some(manager) => {
            let status = manager.status();
            let circuits = manager.circuit_count();
            (format!("✅ {}", status.message), circuits.to_string())
        }
        None => ("⏳ Bootstrapping...".to_string(), "—".to_string()),
    };
    
    let content = format!(r#"
<div class="page home-page">
    <header class="page-header">
        <h1>🎛️ MissionControl</h1>
        <p class="subtitle">Secure Arti-Based Router & Dashboard</p>
    </header>
    
    <div class="dashboard-grid">
        <a href="/circuits" class="dashboard-card card-purple">
            <div class="card-icon">🔌</div>
            <h3 class="card-title">Circuits</h3>
            <p class="card-description">View and manage Tor circuits</p>
            <span class="card-arrow">→</span>
        </a>
        <a href="/services" class="dashboard-card card-blue">
            <div class="card-icon">🧅</div>
            <h3 class="card-title">Onion Services</h3>
            <p class="card-description">Manage hidden service endpoints</p>
            <span class="card-arrow">→</span>
        </a>
        <a href="/metrics" class="dashboard-card card-green">
            <div class="card-icon">📊</div>
            <h3 class="card-title">Metrics</h3>
            <p class="card-description">Bandwidth, latency, and health stats</p>
            <span class="card-arrow">→</span>
        </a>
    </div>
    
    <section class="quick-stats">
        <h2>System Status</h2>
        <div class="stats-grid">
            <div class="stat-card">
                <span class="stat-value">{arti_status}</span>
                <span class="stat-label">Arti Status</span>
            </div>
            <div class="stat-card">
                <span class="stat-value">{circuit_count}</span>
                <span class="stat-label">Active Circuits</span>
            </div>
            <div class="stat-card">
                <span class="stat-value">0</span>
                <span class="stat-label">Onion Services</span>
            </div>
            <div class="stat-card">
                <span class="stat-value">0</span>
                <span class="stat-label">Integrations</span>
            </div>
        </div>
    </section>
    
    <section class="section">
        <div class="section-header">
            <h2>🔗 Integrations</h2>
            <span class="badge badge-info">Coming Soon</span>
        </div>
        <div class="integration-list">
            <div class="integration-card">
                <span class="integration-icon">🌑</span>
                <span class="integration-name">DarkMatter</span>
                <span class="integration-status status-pending">Not Connected</span>
            </div>
            <div class="integration-card">
                <span class="integration-icon">🔭</span>
                <span class="integration-name">GravityLens</span>
                <span class="integration-status status-pending">Not Connected</span>
            </div>
        </div>
    </section>
</div>
"#, arti_status = arti_status, circuit_count = circuit_count);
    
    Html(render_page("Home", &content))
}

/// Circuits page with visualization
pub async fn circuits(State(state): State<Arc<AppState>>) -> Html<String> {
    let arti_lock = state.arti.read().await;
    let circuit_count = match &*arti_lock {
        Some(manager) => manager.circuit_count(),
        None => 0,
    };
    
    let content = format!(r#"
<div class="page">
    <header class="page-header">
        <h1>🔌 Circuits</h1>
        <p class="subtitle">View and manage Tor circuits</p>
    </header>
    
    <section class="section">
        <div class="section-header">
            <h2>Circuit Path Visualization</h2>
            <div class="button-group">
                <button class="btn btn-secondary" onclick="refreshCircuits()">🔄 Refresh</button>
                <button class="btn btn-primary" onclick="newCircuit()">+ New Circuit</button>
            </div>
        </div>
        
        <div class="circuit-path-container">
            <div class="circuit-path">
                <div class="circuit-node client">
                    <div class="node-icon">💻</div>
                    <div class="node-label">You</div>
                    <div class="node-detail">MissionControl</div>
                </div>
                <div class="circuit-connector"></div>
                <div class="circuit-node guard">
                    <div class="node-icon">🛡️</div>
                    <div class="node-label">Guard</div>
                    <div class="node-detail" id="guard-relay">—</div>
                </div>
                <div class="circuit-connector"></div>
                <div class="circuit-node middle">
                    <div class="node-icon">🔀</div>
                    <div class="node-label">Middle</div>
                    <div class="node-detail" id="middle-relay">—</div>
                </div>
                <div class="circuit-connector"></div>
                <div class="circuit-node exit">
                    <div class="node-icon">🚪</div>
                    <div class="node-label">Exit</div>
                    <div class="node-detail" id="exit-relay">—</div>
                </div>
                <div class="circuit-connector"></div>
                <div class="circuit-node destination">
                    <div class="node-icon">🌐</div>
                    <div class="node-label">Destination</div>
                    <div class="node-detail">Tor Network</div>
                </div>
            </div>
        </div>
    </section>
    
    <section class="section">
        <div class="section-header">
            <h2>Active Circuits ({count})</h2>
        </div>
        <div class="table-container">
            <table class="data-table" id="circuits-table">
                <thead>
                    <tr>
                        <th>ID</th>
                        <th>State</th>
                        <th>Guard</th>
                        <th>Middle</th>
                        <th>Exit</th>
                        <th>Age</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td><code>0x01</code></td>
                        <td><span class="badge badge-success">Ready</span></td>
                        <td>🇩🇪 DE</td>
                        <td>🇳🇱 NL</td>
                        <td>🇸🇪 SE</td>
                        <td>2m 34s</td>
                        <td><button class="btn-icon" title="Close">✕</button></td>
                    </tr>
                    <tr>
                        <td><code>0x02</code></td>
                        <td><span class="badge badge-success">Ready</span></td>
                        <td>🇩🇪 DE</td>
                        <td>🇫🇷 FR</td>
                        <td>🇨🇭 CH</td>
                        <td>1m 12s</td>
                        <td><button class="btn-icon" title="Close">✕</button></td>
                    </tr>
                    <tr>
                        <td><code>0x03</code></td>
                        <td><span class="badge badge-info">Building</span></td>
                        <td>🇫🇮 FI</td>
                        <td>—</td>
                        <td>—</td>
                        <td>5s</td>
                        <td><button class="btn-icon" title="Close">✕</button></td>
                    </tr>
                </tbody>
            </table>
        </div>
    </section>
</div>
<script src="/static/js/circuits.js"></script>
"#, count = circuit_count);
    
    Html(render_page("Circuits", &content))
}
/// Onion Services & Arti Administration page
pub async fn services(State(state): State<Arc<AppState>>) -> Html<String> {
    let arti_lock = state.arti.read().await;
    let (arti_status, arti_status_class) = match &*arti_lock {
        Some(manager) => {
            let status = manager.status();
            if status.bootstrapped {
                ("Connected to Tor Network", "online")
            } else {
                ("Bootstrapping...", "pending")
            }
        }
        None => ("Not Running", "offline"),
    };
    
    let content = format!(r#"
<div class="page">
    <header class="page-header">
        <h1>🧅 Onion Services & Arti</h1>
        <p class="subtitle">Manage Tor hidden services and Arti client</p>
    </header>
    
    <section class="section arti-status-section">
        <div class="section-header">
            <h2>🌐 Arti Client Status</h2>
            <span class="node-status-indicator {status_class}"></span>
        </div>
        <div class="arti-status-grid">
            <div class="arti-status-card">
                <span class="status-icon">📡</span>
                <div class="status-info">
                    <span class="status-label">Connection</span>
                    <span class="status-value">{status}</span>
                </div>
            </div>
            <div class="arti-status-card">
                <span class="status-icon">🔌</span>
                <div class="status-info">
                    <span class="status-label">Mode</span>
                    <span class="status-value">Embedded Client</span>
                </div>
            </div>
            <div class="arti-status-card">
                <span class="status-icon">🛡️</span>
                <div class="status-info">
                    <span class="status-label">Vanguards</span>
                    <span class="status-value">Lite (default)</span>
                </div>
            </div>
            <div class="arti-status-card">
                <span class="status-icon">🔒</span>
                <div class="status-info">
                    <span class="status-label">PQC</span>
                    <span class="status-value">Hybrid Ready</span>
                </div>
            </div>
        </div>
        <div class="arti-controls">
            <button class="btn btn-secondary" onclick="restartArti()">🔄 Restart</button>
            <button class="btn btn-secondary" onclick="rebuildCircuits()">🔌 Rebuild Circuits</button>
            <a href="/config" class="btn btn-secondary">⚙️ Configuration</a>
        </div>
    </section>
    
    <section class="section">
        <div class="section-header">
            <h2>🧅 Onion Services</h2>
            <button class="btn btn-primary" onclick="showNewServiceModal()">+ New Service</button>
        </div>
        
        <div class="service-list">
            <div class="service-card service-active">
                <div class="service-header">
                    <div class="service-icon">🏠</div>
                    <div class="service-info">
                        <h3 class="service-name">event-horizon</h3>
                        <code class="service-address">xxxxxx...xxxx.onion</code>
                    </div>
                    <span class="node-status-indicator online"></span>
                </div>
                <div class="service-details">
                    <div class="service-detail">
                        <span class="detail-label">Target</span>
                        <span class="detail-value">127.0.0.1:3030</span>
                    </div>
                    <div class="service-detail">
                        <span class="detail-label">Virtual Port</span>
                        <span class="detail-value">80</span>
                    </div>
                    <div class="service-detail">
                        <span class="detail-label">Client Auth</span>
                        <span class="detail-value auth-enabled">Enabled (2 clients)</span>
                    </div>
                </div>
                <div class="service-actions">
                    <button class="btn btn-secondary btn-small">Copy Address</button>
                    <button class="btn btn-secondary btn-small">Manage Auth</button>
                    <button class="btn btn-secondary btn-small btn-danger">Stop</button>
                </div>
            </div>
            
            <div class="service-card service-template" id="new-service-template" style="display: none;">
                <form class="new-service-form" onsubmit="createService(event)">
                    <h3>Create New Onion Service</h3>
                    <div class="form-group">
                        <label>Nickname</label>
                        <input type="text" name="nickname" placeholder="my-service" required>
                    </div>
                    <div class="form-row">
                        <div class="form-group">
                            <label>Target Port</label>
                            <input type="number" name="target_port" placeholder="8080" required>
                        </div>
                        <div class="form-group">
                            <label>Virtual Port</label>
                            <input type="number" name="virtual_port" placeholder="80" value="80">
                        </div>
                    </div>
                    <div class="form-group">
                        <label>
                            <input type="checkbox" name="client_auth"> Enable Client Authorization
                        </label>
                    </div>
                    <div class="form-actions">
                        <button type="button" class="btn btn-secondary" onclick="hideNewServiceModal()">Cancel</button>
                        <button type="submit" class="btn btn-primary">Create Service</button>
                    </div>
                </form>
            </div>
        </div>
    </section>
    
    <section class="section">
        <div class="section-header">
            <h2>📖 Quick Reference</h2>
        </div>
        <div class="quick-ref">
            <div class="ref-item">
                <h4>Test via Tor Browser</h4>
                <code>http://[your-onion].onion</code>
            </div>
            <div class="ref-item">
                <h4>Config Location</h4>
                <code>umbra/arti.toml</code>
            </div>
            <div class="ref-item">
                <h4>Keys Directory</h4>
                <code>umbra/keys/arti/</code>
            </div>
            <div class="ref-item">
                <h4>View Logs</h4>
                <code>tail -f umbra/var/log/arti.log</code>
            </div>
        </div>
    </section>
</div>
<script src="/static/js/services.js"></script>
"#, status = arti_status, status_class = arti_status_class);
    
    Html(render_page("Onion Services", &content))
}

/// Metrics page with bandwidth and latency visualization
pub async fn metrics(State(_state): State<Arc<AppState>>) -> Html<String> {
    let content = r#"
<div class="page">
    <header class="page-header">
        <h1>📊 Metrics</h1>
        <p class="subtitle">Bandwidth, latency, and health statistics</p>
    </header>
    
    <section class="quick-stats">
        <h2>Current Statistics</h2>
        <div class="stats-grid">
            <div class="stat-card">
                <span class="stat-value" id="stat-tx">0 KB/s</span>
                <span class="stat-label">Upload</span>
            </div>
            <div class="stat-card">
                <span class="stat-value" id="stat-rx">0 KB/s</span>
                <span class="stat-label">Download</span>
            </div>
            <div class="stat-card">
                <span class="stat-value" id="stat-latency">— ms</span>
                <span class="stat-label">Avg Latency</span>
            </div>
            <div class="stat-card">
                <span class="stat-value" id="stat-uptime">0h 0m</span>
                <span class="stat-label">Uptime</span>
            </div>
        </div>
    </section>
    
    <section class="section">
        <div class="section-header">
            <h2>Bandwidth Usage</h2>
            <div class="button-group">
                <button class="btn btn-secondary btn-small active" data-range="1h">1H</button>
                <button class="btn btn-secondary btn-small" data-range="24h">24H</button>
                <button class="btn btn-secondary btn-small" data-range="7d">7D</button>
            </div>
        </div>
        <div class="chart-container" id="bandwidth-chart">
            <div class="chart-placeholder">
                <div class="chart-bars">
                    <div class="chart-bar" style="height: 30%"></div>
                    <div class="chart-bar" style="height: 45%"></div>
                    <div class="chart-bar" style="height: 60%"></div>
                    <div class="chart-bar" style="height: 40%"></div>
                    <div class="chart-bar" style="height: 75%"></div>
                    <div class="chart-bar" style="height: 55%"></div>
                    <div class="chart-bar" style="height: 80%"></div>
                    <div class="chart-bar" style="height: 65%"></div>
                    <div class="chart-bar" style="height: 50%"></div>
                    <div class="chart-bar" style="height: 35%"></div>
                    <div class="chart-bar" style="height: 70%"></div>
                    <div class="chart-bar" style="height: 85%"></div>
                </div>
                <div class="chart-legend">
                    <span class="legend-item"><span class="legend-color tx"></span> Upload</span>
                    <span class="legend-item"><span class="legend-color rx"></span> Download</span>
                </div>
            </div>
        </div>
    </section>
    
    <section class="section">
        <div class="section-header">
            <h2>Circuit Latency</h2>
        </div>
        <div class="latency-grid">
            <div class="latency-card">
                <div class="latency-ring good">
                    <span class="latency-value">120ms</span>
                </div>
                <span class="latency-label">Circuit 0x01</span>
            </div>
            <div class="latency-card">
                <div class="latency-ring good">
                    <span class="latency-value">98ms</span>
                </div>
                <span class="latency-label">Circuit 0x02</span>
            </div>
            <div class="latency-card">
                <div class="latency-ring warning">
                    <span class="latency-value">340ms</span>
                </div>
                <span class="latency-label">Circuit 0x03</span>
            </div>
        </div>
    </section>
    
    <section class="section">
        <div class="section-header">
            <h2>Event Log</h2>
            <button class="btn btn-secondary" onclick="clearLogs()">Clear</button>
        </div>
        <div class="log-viewer" id="log-viewer">
            <div class="log-entry info">
                <span class="log-time">17:08:42</span>
                <span class="log-level">INFO</span>
                <span class="log-msg">Arti client bootstrapped successfully</span>
            </div>
            <div class="log-entry info">
                <span class="log-time">17:08:45</span>
                <span class="log-level">INFO</span>
                <span class="log-msg">Circuit 0x01 established via DE → NL → SE</span>
            </div>
            <div class="log-entry info">
                <span class="log-time">17:08:48</span>
                <span class="log-level">INFO</span>
                <span class="log-msg">Circuit 0x02 established via DE → FR → CH</span>
            </div>
            <div class="log-entry warn">
                <span class="log-time">17:09:01</span>
                <span class="log-level">WARN</span>
                <span class="log-msg">High latency detected on circuit 0x03 (340ms)</span>
            </div>
        </div>
    </section>
</div>
<script src="/static/js/metrics.js"></script>
"#;
    Html(render_page("Metrics", content))
}

/// Configuration page
pub async fn config_page(State(state): State<Arc<AppState>>) -> Html<String> {
    let content = format!(r#"
<div class="page">
    <header class="page-header">
        <h1>⚙️ Configuration</h1>
        <p class="subtitle">MissionControl settings and options</p>
    </header>
    
    <section class="section">
        <div class="section-header">
            <h2>Current Configuration</h2>
        </div>
        <div class="config-display">
            <div class="config-item">
                <span class="config-label">Listen Address</span>
                <code class="config-value">{listen}</code>
            </div>
            <div class="config-item">
                <span class="config-label">Database Path</span>
                <code class="config-value">{db_path}</code>
            </div>
            <div class="config-item">
                <span class="config-label">Arti State</span>
                <code class="config-value">{arti_state}</code>
            </div>
        </div>
    </section>
</div>
"#, 
        listen = state.config.listen_addr,
        db_path = state.config.database_path.display(),
        arti_state = state.config.arti.state_dir.display()
    );
    Html(render_page("Configuration", &content))
}

/// Integrations page - Multi-node crypto dashboard
pub async fn integrations(State(_state): State<Arc<AppState>>) -> Html<String> {
    let content = r#"
<div class="page">
    <header class="page-header">
        <h1>🔗 Integrations</h1>
        <p class="subtitle">Multi-node crypto dashboard powered by Umbra</p>
    </header>
    
    <section class="section">
        <div class="section-header">
            <h2>Active Nodes</h2>
            <button class="btn btn-primary">+ Add Node</button>
        </div>
        
        <div class="node-grid">
            <div class="node-card node-active">
                <div class="node-header">
                    <span class="node-icon">🛡️</span>
                    <div class="node-info">
                        <h3 class="node-name">Zcash (Zebra)</h3>
                        <span class="node-type">Privacy Coin</span>
                    </div>
                    <span class="node-status-indicator online"></span>
                </div>
                <div class="node-stats">
                    <div class="node-stat">
                        <span class="stat-label">Block Height</span>
                        <span class="stat-value" id="zcash-height">2,847,521</span>
                    </div>
                    <div class="node-stat">
                        <span class="stat-label">Peers</span>
                        <span class="stat-value" id="zcash-peers">8</span>
                    </div>
                    <div class="node-stat">
                        <span class="stat-label">Sync</span>
                        <span class="stat-value synced" id="zcash-sync">100%</span>
                    </div>
                </div>
                <div class="node-actions">
                    <button class="btn btn-secondary btn-small">View Metrics</button>
                    <button class="btn btn-secondary btn-small">Restart</button>
                </div>
            </div>
            
            <div class="node-card node-pending">
                <div class="node-header">
                    <span class="node-icon">₿</span>
                    <div class="node-info">
                        <h3 class="node-name">Bitcoin Core</h3>
                        <span class="node-type">Coming Soon</span>
                    </div>
                    <span class="node-status-indicator pending"></span>
                </div>
                <div class="node-placeholder">
                    <p>Configure Bitcoin Core integration</p>
                    <button class="btn btn-secondary btn-small" disabled>Setup</button>
                </div>
            </div>
            
            <div class="node-card node-pending">
                <div class="node-header">
                    <span class="node-icon">ɱ</span>
                    <div class="node-info">
                        <h3 class="node-name">Monero</h3>
                        <span class="node-type">Coming Soon</span>
                    </div>
                    <span class="node-status-indicator pending"></span>
                </div>
                <div class="node-placeholder">
                    <p>Configure Monero integration</p>
                    <button class="btn btn-secondary btn-small" disabled>Setup</button>
                </div>
            </div>
        </div>
    </section>
    
    <section class="section">
        <div class="section-header">
            <h2>🌑 DarkMatter Status</h2>
            <span class="badge badge-success">Connected</span>
        </div>
        <div class="config-display">
            <div class="config-item">
                <span class="config-label">Metrics Endpoint</span>
                <code class="config-value">http://127.0.0.1:9999/metrics</code>
            </div>
            <div class="config-item">
                <span class="config-label">Tor Routing</span>
                <code class="config-value">Disabled (localhost)</code>
            </div>
            <div class="config-item">
                <span class="config-label">Last Sync</span>
                <code class="config-value" id="darkmatter-sync">Just now</code>
            </div>
        </div>
    </section>
    
    <section class="section">
        <div class="section-header">
            <h2>🔭 GravityLens Status</h2>
            <span class="badge badge-info">Phase 2</span>
        </div>
        <div class="integration-placeholder">
            <p>GravityLens integration will route blockchain API calls through Tor for enhanced privacy.</p>
            <ul class="feature-list">
                <li>✓ Per-chain circuit isolation</li>
                <li>✓ RPC provider IP obfuscation</li>
                <li>◯ Rate limiting aggregation</li>
            </ul>
        </div>
    </section>
</div>
<script src="/static/js/integrations.js"></script>
"#;
    Html(render_page("Integrations", content))
}
