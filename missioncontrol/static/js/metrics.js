// Metrics page JavaScript
// Handles live stats updates and chart animations

let startTime = Date.now();

async function updateStats() {
    try {
        const response = await fetch('/api/status');
        const data = await response.json();

        // Update uptime
        const uptimeMs = Date.now() - startTime;
        const hours = Math.floor(uptimeMs / 3600000);
        const minutes = Math.floor((uptimeMs % 3600000) / 60000);
        document.getElementById('stat-uptime').textContent = `${hours}h ${minutes}m`;

        // TODO: Fetch actual bandwidth when metrics API is ready
    } catch (error) {
        console.error('Failed to update stats:', error);
    }
}

function clearLogs() {
    const logViewer = document.getElementById('log-viewer');
    logViewer.innerHTML = '<div class="log-entry info"><span class="log-time">--:--:--</span><span class="log-level">INFO</span><span class="log-msg">Log cleared</span></div>';
}

// Animate chart bars randomly for demo
function animateChart() {
    const bars = document.querySelectorAll('.chart-bar');
    bars.forEach(bar => {
        const height = 20 + Math.random() * 60;
        bar.style.height = `${height}%`;
    });
}

// Time range button handling
document.querySelectorAll('[data-range]').forEach(btn => {
    btn.addEventListener('click', function () {
        document.querySelectorAll('[data-range]').forEach(b => b.classList.remove('active'));
        this.classList.add('active');
        animateChart();
    });
});

// Periodic updates
setInterval(updateStats, 5000);
setInterval(animateChart, 3000);

// Initial load
document.addEventListener('DOMContentLoaded', () => {
    updateStats();
    animateChart();
});
