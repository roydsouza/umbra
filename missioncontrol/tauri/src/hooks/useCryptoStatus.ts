import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export interface CryptoStatus {
    zen: NodeStatus;
    monero: NodeStatus;
    last_updated: number;
}

export interface NodeStatus {
    connected: boolean;
    synced: boolean;
    block_height?: number;
    peers?: number;
    version?: string;
}

export function useCryptoStatus() {
    const [status, setStatus] = useState<CryptoStatus | null>(null);

    useEffect(() => {
        const fetchStatus = async () => {
            try {
                const data = await invoke<CryptoStatus>('get_crypto_status');
                setStatus(data);
            } catch (e) {
                console.error("Failed to fetch crypto status:", e);
            }
        };

        fetchStatus();
        const interval = setInterval(fetchStatus, 10000); // Poll every 10s
        return () => clearInterval(interval);
    }, []);

    return status;
}
