import { Shield, Lock, Activity, Server, Play, Square } from 'lucide-react';
import { GlassCard } from '../components/ui/GlassCard';
import { useCryptoStatus } from '../hooks/useCryptoStatus';
import type { NodeStatus } from '../hooks/useCryptoStatus';
import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';

const NodeCard = ({ title, status, icon, color, onStart, onStop }: {
    title: string,
    status?: NodeStatus,
    icon: React.ReactNode,
    color: string,
    onStart: () => Promise<void>,
    onStop: () => Promise<void>
}) => {
    const [loading, setLoading] = useState(false);

    const handleAction = async (action: () => Promise<void>) => {
        setLoading(true);
        try {
            await action();
        } catch (e) {
            console.error(e);
        } finally {
            setLoading(false);
        }
    };

    if (!status) return (
        <GlassCard className="p-6 opacity-50 flex flex-col gap-4">
            <div className="flex justify-between items-start">
                <h2 className="text-xl font-bold flex items-center gap-2 text-white">
                    {icon} {title}
                </h2>
            </div>
            <div className="text-sm font-mono text-text-muted">Initializing...</div>
        </GlassCard>
    );

    return (
        <GlassCard className={`p-6 flex flex-col gap-4 border-l-4 ${color}`}>
            <div className="flex justify-between items-start">
                <div>
                    <h2 className="text-xl font-bold flex items-center gap-2 text-white">
                        {icon} {title}
                    </h2>
                    <p className="text-sm text-text-muted mt-1">{status.version || 'Unknown Version'}</p>
                </div>
                <div className="flex flex-col items-end gap-2">
                    <div className={`px-2 py-1 rounded text-xs font-bold uppercase
                        ${status.synced ? 'bg-accent-green/20 text-accent-green' : 'bg-yellow-500/20 text-yellow-500'}
                    `}>
                        {status.synced ? 'SYNCED' : 'SYNCING'}
                    </div>
                    <div className="flex gap-2">
                        <button
                            onClick={() => handleAction(onStart)}
                            disabled={loading || status.connected}
                            className="p-1.5 rounded bg-accent-green/10 text-accent-green hover:bg-accent-green/20 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
                            title="Start Node"
                        >
                            <Play size={14} fill="currentColor" />
                        </button>
                        <button
                            onClick={() => handleAction(onStop)}
                            disabled={loading || !status.connected}
                            className="p-1.5 rounded bg-red-500/10 text-red-500 hover:bg-red-500/20 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
                            title="Stop Node"
                        >
                            <Square size={14} fill="currentColor" />
                        </button>
                    </div>
                </div>
            </div>

            <div className="grid grid-cols-2 gap-4 mt-2">
                <div className="bg-white/5 p-3 rounded">
                    <div className="text-xs text-text-muted uppercase mb-1">Block Height</div>
                    <div className="text-xl font-mono font-bold text-white">
                        {status.block_height?.toLocaleString() || '-'}
                    </div>
                </div>
                <div className="bg-white/5 p-3 rounded">
                    <div className="text-xs text-text-muted uppercase mb-1">Peers</div>
                    <div className="text-xl font-mono font-bold text-white flex items-center gap-2">
                        <Activity size={14} className="text-accent-blue" />
                        {status.peers || 0}
                    </div>
                </div>
            </div>

            <div className="flex items-center gap-2 text-xs text-text-muted font-mono">
                <div className={`w-2 h-2 rounded-full ${status.connected ? 'bg-accent-green' : 'bg-red-500'}`} />
                {status.connected ? 'RPC Connected' : 'RPC Disconnected'}
            </div>
        </GlassCard>
    );
};

export const DarkMatter = () => {
    const status = useCryptoStatus();

    return (
        <div className="h-full flex flex-col gap-6">
            <div className="flex justify-between items-end">
                <div>
                    <h1 className="text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-accent-blue to-accent-purple">
                        DarkMatter
                    </h1>
                    <p className="text-text-secondary mt-2">Crypto Node Integrations</p>
                </div>
                {status?.last_updated && (
                    <div className="text-xs font-mono text-text-muted">
                        Updated: {new Date(Number(status.last_updated) * 1000).toLocaleTimeString()}
                    </div>
                )}
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <NodeCard
                    title="Zcash (Zebra)"
                    status={status?.zen}
                    icon={<Shield className="text-yellow-400" />}
                    color="border-yellow-400"
                    onStart={() => invoke('start_darkmatter_node')}
                    onStop={() => invoke('stop_darkmatter_node')}
                />

                <NodeCard
                    title="Monero"
                    status={status?.monero}
                    icon={<Lock className="text-orange-500" />}
                    color="border-orange-500"
                    onStart={() => invoke('start_monero_node')}
                    onStop={() => invoke('stop_monero_node')}
                />
            </div>

            <GlassCard className="p-6 mt-auto">
                <div className="flex items-center gap-4">
                    <Server className="text-accent-blue" />
                    <div>
                        <h3 className="font-bold text-white">Integration Status</h3>
                        <p className="text-sm text-text-muted">
                            External node managers are running. You can now start/stop nodes directly from this dashboard.
                        </p>
                    </div>
                </div>
            </GlassCard>
        </div>
    );
};
