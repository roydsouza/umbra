import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

export function useArtiStatus() {
    const [status, setStatus] = useState<"init" | "ready" | "error">("init");
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        // Listen for ready event
        const unlistenReady = listen('arti://ready', () => {
            console.log("Arti Ready Event Received");
            setStatus("ready");
            setError(null);
        });

        // Listen for error event
        const unlistenError = listen<string>('arti://error', (event) => {
            console.error("Arti Error:", event.payload);
            setStatus("error");
            setError(event.payload);
        });

        return () => {
            unlistenReady.then((f: any) => f());
            unlistenError.then((f: any) => f());
        };
    }, []);

    return { status, error };
}
