import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';

export interface SystemStats {
    uptime_seconds: number;
    arti_ready: boolean;
    guardian_connected: boolean;
    leaks_detected: number;
}

export function useSystemStats() {
    const [stats, setStats] = useState<SystemStats | null>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const fetchStats = async () => {
            try {
                const data = await invoke<SystemStats>('get_system_stats');
                setStats(data);
            } catch (err) {
                console.error("Failed to fetch system stats:", err);
            } finally {
                setLoading(false);
            }
        };

        fetchStats();
        // Poll every 5 seconds for now
        const interval = setInterval(fetchStats, 5000);
        return () => clearInterval(interval);
    }, []);

    return { stats, loading };
}
