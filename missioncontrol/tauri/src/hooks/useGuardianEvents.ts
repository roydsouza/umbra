import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

export interface LeakEvent {
    id?: number;
    timestamp?: string;
    process_name?: string;
    process_pid?: number;
    process_path?: string;
    dest_ip?: string;
    dest_port?: number;
    severity: string;
    event_type?: string;
}

export function useGuardianEvents() {
    const [events, setEvents] = useState<LeakEvent[]>([]);

    useEffect(() => {
        const unlisten = listen<LeakEvent>('guardian://leak', (event) => {
            console.log("Leak Event:", event.payload);
            setEvents((prev) => [event.payload, ...prev].slice(0, 50)); // Keep last 50
        });

        return () => {
            unlisten.then(f => f());
        };
    }, []);

    return { events, clearEvents: () => setEvents([]) };
}
