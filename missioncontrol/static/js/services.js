// Onion Services page JavaScript
// Handles service creation, control, and Arti management

function showNewServiceModal() {
    document.getElementById('new-service-template').style.display = 'block';
}

function hideNewServiceModal() {
    document.getElementById('new-service-template').style.display = 'none';
}

async function createService(event) {
    event.preventDefault();
    const form = event.target;
    const data = {
        nickname: form.nickname.value,
        target_port: parseInt(form.target_port.value),
        virtual_port: parseInt(form.virtual_port.value) || 80,
        client_auth: form.client_auth.checked
    };

    try {
        const response = await fetch('/api/services', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data)
        });

        if (response.ok) {
            alert('Service created! Configure in arti.toml and restart Arti.');
            hideNewServiceModal();
            location.reload();
        } else {
            alert('Failed to create service');
        }
    } catch (error) {
        console.error('Service creation error:', error);
        alert('Error creating service');
    }
}

async function restartArti() {
    if (!confirm('Restart Arti client? Active connections will be interrupted.')) {
        return;
    }

    try {
        const response = await fetch('/api/arti/restart', { method: 'POST' });
        if (response.ok) {
            alert('Arti restart initiated. Please wait 30-60 seconds for reconnection.');
        }
    } catch (error) {
        alert('Restart command sent (check logs for status)');
    }
}

async function rebuildCircuits() {
    try {
        const response = await fetch('/api/arti/circuits/rebuild', { method: 'POST' });
        if (response.ok) {
            alert('Circuit rebuild initiated');
        }
    } catch (error) {
        alert('Circuit rebuild command sent');
    }
}

// Copy onion address to clipboard
function copyAddress(address) {
    navigator.clipboard.writeText(address).then(() => {
        alert('Onion address copied to clipboard');
    });
}
