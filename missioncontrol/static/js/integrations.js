// Integrations page JavaScript
// Handles DarkMatter connection and node status updates

async function fetchDarkMatterStatus() {
    try {
        const response = await fetch('/api/integrations/darkmatter/status');
        const data = await response.json();

        if (data.connected) {
            document.getElementById('zcash-height').textContent = formatNumber(data.block_height || 0);
            document.getElementById('zcash-peers').textContent = data.peers || 0;
            document.getElementById('zcash-sync').textContent = data.synced ? '100%' : 'Syncing...';
            document.getElementById('darkmatter-sync').textContent = 'Just now';
        }
    } catch (error) {
        console.log('DarkMatter not connected:', error.message);
        document.getElementById('darkmatter-sync').textContent = 'Disconnected';
    }
}

function formatNumber(num) {
    return num.toLocaleString();
}

// Update last sync time
function updateSyncTime() {
    const el = document.getElementById('darkmatter-sync');
    if (el && el.textContent !== 'Disconnected') {
        // Increment time indicator
    }
}

// Periodic refresh
setInterval(fetchDarkMatterStatus, 10000);

// Initial load
document.addEventListener('DOMContentLoaded', fetchDarkMatterStatus);
