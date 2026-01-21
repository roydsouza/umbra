import React from 'react';
import { Activity, ShieldCheck, Zap, Server, Globe } from 'lucide-react';
import { GlassCard } from '../components/ui/GlassCard';
import { useArtiStatus } from '../hooks/useArtiStatus';
import { useSystemStats } from '../hooks/useSystemStats';

export const Dashboard: React.FC = () => {
    const { status: artiStatus, error: artiError } = useArtiStatus();
    const { stats } = useSystemStats();

    return (
        <div className="flex flex-col gap-6">
            {artiError && (
                <div className="bg-red-500/10 border border-red-500/50 text-red-500 p-3 rounded-lg flex items-center gap-2">
                    <ShieldCheck size={16} />
                    <span className="text-sm font-mono">ARTI ERROR: {artiError}</span>
                </div>
            )}
            {/* Quick Stats Row */}
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                <GlassCard className="p-4 flex flex-col items-center justify-center gap-1" hoverEffect={false}>
                    <div className="text-2xl font-mono font-bold text-accent-blue">
                        {artiStatus === 'ready' ? 'ONLINE' : 'BOOT...'}
                    </div>
                    <div className="text-xs text-text-muted uppercase tracking-wider">Arti Status</div>
                </GlassCard>
                <GlassCard className="p-4 flex flex-col items-center justify-center gap-1" hoverEffect={false}>
                    <div className="text-2xl font-mono font-bold text-accent-purple">
                        {stats?.arti_ready ? '3' : '0'}
                        {/* Mocked circuit count for now if ready, real implementation needs get_arti_status return */}
                    </div>
                    <div className="text-xs text-text-muted uppercase tracking-wider">Active Circuits</div>
                </GlassCard>
                <GlassCard className="p-4 flex flex-col items-center justify-center gap-1" hoverEffect={false}>
                    <div className="text-2xl font-mono font-bold text-accent-green">
                        {stats?.guardian_connected ? 'SECURE' : 'OFFLINE'}
                    </div>
                    <div className="text-xs text-text-muted uppercase tracking-wider">Guardian Shield</div>
                </GlassCard>
                <GlassCard className="p-4 flex flex-col items-center justify-center gap-1" hoverEffect={false}>
                    <div className="text-2xl font-mono font-bold text-accent-red">
                        {stats?.leaks_detected || 0}
                    </div>
                    <div className="text-xs text-text-muted uppercase tracking-wider">Leaks Detected</div>
                </GlassCard>
            </div>

            {/* Main Grid */}
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                {/* Active Circuits */}
                <GlassCard className="col-span-2 relative overflow-hidden group cursor-pointer">
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-lg font-semibold flex items-center gap-2">
                            <Zap className="text-accent-blue" />
                            Active Circuits
                        </h2>
                        <span className="text-sm font-mono text-accent-green">OPTIMIZED</span>
                    </div>
                    <div className="h-48 flex items-center justify-center border border-dashed border-glass-border rounded-lg bg-bg-tertiary/30">
                        {/* Placeholder for Circuit Visualizer */}
                        <div className="flex flex-col items-center gap-2 text-text-muted">
                            <Globe size={48} className="opacity-20" />
                            <span>Circuit Map Visualization (Coming Phase 3)</span>
                        </div>
                    </div>
                    <div className="absolute top-0 left-0 w-full h-1 bg-gradient-primary opacity-0 group-hover:opacity-100 transition-opacity" />
                </GlassCard>

                {/* Bandwidth / Metrics */}
                <GlassCard className="col-span-1 cursor-pointer">
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-lg font-semibold flex items-center gap-2">
                            <Activity className="text-accent-purple" />
                            Bandwidth
                        </h2>
                    </div>
                    <div className="h-48 flex items-center justify-center border border-dashed border-glass-border rounded-lg bg-bg-tertiary/30">
                        <span className="text-text-muted">Metrics Chart</span>
                    </div>
                </GlassCard>

                {/* Onion Services */}
                <GlassCard className="col-span-1 cursor-pointer">
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-lg font-semibold flex items-center gap-2">
                            <Server className="text-accent-cyan" />
                            Onion Services
                        </h2>
                    </div>
                    <div className="flex flex-col gap-3">
                        <div className="p-3 rounded-lg bg-bg-tertiary/50 border border-glass-border flex justify-between items-center">
                            <div className="flex items-center gap-2">
                                <div className="w-2 h-2 rounded-full bg-accent-green"></div>
                                <span className="font-mono text-sm">event-horizon</span>
                            </div>
                            <span className="text-xs text-text-muted">80 &rarr; 3030</span>
                        </div>
                        <div className="p-3 rounded-lg bg-bg-tertiary/30 border border-glass-border border-dashed flex justify-center items-center text-text-muted text-sm">
                            + Add Service
                        </div>
                    </div>
                </GlassCard>

                {/* Guardian Shield Detail */}
                <GlassCard className="col-span-2 border-accent-green/30 hover:border-accent-green cursor-pointer">
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-lg font-semibold flex items-center gap-2">
                            <ShieldCheck className="text-accent-green" />
                            Guardian Network Shield
                        </h2>
                        <span className="badge bg-green-500/10 text-green-500 px-2 py-1 rounded text-xs">ACTIVE</span>
                    </div>
                    <div className="grid grid-cols-3 gap-4">
                        <div className="p-3 bg-bg-tertiary/50 rounded-lg">
                            <div className="text-xs text-text-muted mb-1">Protection Level</div>
                            <div className="font-bold text-accent-green">STRICT</div>
                        </div>
                        <div className="p-3 bg-bg-tertiary/50 rounded-lg">
                            <div className="text-xs text-text-muted mb-1">DNS Policy</div>
                            <div className="font-bold text-white">Tor-Only</div>
                        </div>
                        <div className="p-3 bg-bg-tertiary/50 rounded-lg">
                            <div className="text-xs text-text-muted mb-1">Process Monitor</div>
                            <div className="font-bold text-white">Running</div>
                        </div>
                    </div>
                </GlassCard>
            </div>
        </div>
    );
};
