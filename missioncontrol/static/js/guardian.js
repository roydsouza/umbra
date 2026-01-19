/**
 * Guardian Dashboard JavaScript
 * Handles real-time leak events and configuration updates
 */

document.addEventListener('DOMContentLoaded', () => {
    initGuardianDashboard();
});

async function initGuardianDashboard() {
    console.log("🛡️ Guardian Dashboard Initializing...");

    // 1. Fetch historical data from MissionControl Brain
    await fetchLeakHistory();

    // 2. Connect to real-time stream
    connectToLeakStream();
}

/**
 * Fetch historical leak events from the MissionControl database
 */
async function fetchLeakHistory() {
    try {
        const response = await fetch('/api/guardian/history?limit=20');
        const history = await response.json();
        const tbody = document.getElementById('leak-history-body');

        if (!tbody) return;
        tbody.innerHTML = '';

        history.forEach(event => {
            appendHistoryRow(event);
        });

        // Update stats
        document.getElementById('live-leak-count').textContent = history.length;
    } catch (error) {
        console.error("Failed to fetch leak history:", error);
    }
}

/**
 * Connect to the real-time WebSocket stream for new leak events
 */
function connectToLeakStream() {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}/api/guardian/stream`;
    const socket = new WebSocket(wsUrl);
    const viewer = document.getElementById('leak-log-viewer');
    const statusBadge = document.getElementById('stream-status');

    socket.onopen = () => {
        console.log("✅ Connected to Guardian stream");
        statusBadge.textContent = "Live Connected";
        statusBadge.className = "badge badge-success";
    };

    socket.onmessage = (event) => {
        const leak = JSON.parse(event.data);
        handleNewLeak(leak);
    };

    socket.onclose = () => {
        console.warn("❌ Disconnected from Guardian stream. Retrying in 5s...");
        statusBadge.textContent = "Disconnected";
        statusBadge.className = "badge badge-danger";
        setTimeout(connectToLeakStream, 5000);
    };

    socket.onerror = (err) => {
        console.error("WebSocket Error:", err);
    };
}

/**
 * Process a new leak event from the stream
 */
function handleNewLeak(leak) {
    // 1. Update the live stream viewer (log style)
    appendStreamEntry(leak);

    // 2. Add to the history table
    appendHistoryRow(leak, true);

    // 3. Update counter
    const countEl = document.getElementById('live-leak-count');
    countEl.textContent = parseInt(countEl.textContent) + 1;

    // 4. Update home page stat if we're on home (optional, but good for SPA feel)
    const homeStat = document.getElementById('guardian-stat-leaks');
    if (homeStat) homeStat.textContent = countEl.textContent;
}

function appendStreamEntry(leak) {
    const viewer = document.getElementById('leak-log-viewer');
    if (!viewer) return;

    // Remove placeholder
    const placeholder = viewer.querySelector('.placeholder-entry');
    if (placeholder) placeholder.remove();

    const entry = document.createElement('div');
    entry.className = `log-entry ${leak.severity.toLowerCase()}`;

    const time = new Date().toLocaleTimeString();

    entry.innerHTML = `
        <span class="log-time">${time}</span>
        <span class="log-level">${leak.severity}</span>
        <span class="log-msg">
            <b>${leak.process_name || 'unknown'}</b> attempted connection to 
            <code>${leak.dest_ip}:${leak.dest_port}</code> [${leak.event_type || 'TCP'}]
        </span>
    `;

    viewer.prepend(entry);

    // Limit log size
    if (viewer.childNodes.length > 50) {
        viewer.lastChild.remove();
    }
}

function appendHistoryRow(leak, prepend = false) {
    const tbody = document.getElementById('leak-history-body');
    if (!tbody) return;

    const row = document.createElement('tr');
    const time = leak.timestamp ? new Date(leak.timestamp).toLocaleString() : new Date().toLocaleString();

    row.innerHTML = `
        <td>${time}</td>
        <td><code>${leak.process_name || 'unknown'}</code></td>
        <td>${leak.process_pid || '—'}</td>
        <td><code>${leak.dest_ip}:${leak.dest_port}</code></td>
        <td><span class="badge ${leak.severity === 'CRITICAL' ? 'badge-danger' : 'badge-info'}">${leak.severity}</span></td>
        <td>${leak.event_type || 'TCP'}</td>
        <td>
            <button class="btn-icon" title="View Details">👁️</button>
            <button class="btn-icon" title="Whitelist">🛡️</button>
        </td>
    `;

    if (prepend) {
        tbody.prepend(row);
    } else {
        tbody.appendChild(row);
    }
}

/**
 * Config Update Handler
 */
async function updateGuardianConfig(event) {
    event.preventDefault();
    const formData = new FormData(event.target);
    const data = {
        dns_policy: formData.get('dns_policy')
    };

    try {
        const response = await fetch('/api/guardian/config', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data)
        });
        const result = await response.json();
        if (result.status === 'success') {
            alert("✅ Guardian configuration updated successfully");
        } else {
            alert("❌ Failed to update configuration: " + result.message);
        }
    } catch (error) {
        alert("❌ Error connecting to MissionControl API");
    }
}

function emergencyKill() {
    if (confirm("⚠️ EMERGENCY: This will terminate all non-Tor network connections. Proceed?")) {
        console.warn("EMERGENCY KILLSWITCH ACTIVATED");
        // TODO: Implement via API
    }
}
