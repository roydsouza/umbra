// Circuit page JavaScript
// Handles circuit refresh and visualization updates

async function refreshCircuits() {
    try {
        const response = await fetch('/api/arti/status');
        const data = await response.json();

        // Update circuit count in header
        console.log('Arti status:', data);

        // TODO: Fetch actual circuit data when API is ready
        console.log('Circuits refreshed');
    } catch (error) {
        console.error('Failed to refresh circuits:', error);
    }
}

function newCircuit() {
    alert('Circuit creation will be available when Arti is fully bootstrapped');
}

// Periodic refresh every 30 seconds
setInterval(refreshCircuits, 30000);

// Initial load
document.addEventListener('DOMContentLoaded', refreshCircuits);
