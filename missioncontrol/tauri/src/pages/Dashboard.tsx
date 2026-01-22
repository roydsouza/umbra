import React from 'react';
import { Activity, ShieldCheck, Zap, Server, Globe } from 'lucide-react';
import { GlassCard } from '../components/ui/GlassCard';
import { useArtiStatus } from '../hooks/useArtiStatus';
import { useSystemStats } from '../hooks/useSystemStats';
import { useCryptoStatus } from '../hooks/useCryptoStatus';
import { invoke } from '@tauri-apps/api/core';

export const Dashboard: React.FC = () => {
    const { status: artiStatus, error: artiError } = useArtiStatus();
    const { stats } = useSystemStats();
    const cryptoStatus = useCryptoStatus();

    const [circuitCount, setCircuitCount] = React.useState(0);

    React.useEffect(() => {
        const fetchArtiDetails = async () => {
            try {
                const data = await invoke<any>('get_arti_status');
                setCircuitCount(data.circuit_count);
            } catch (e) {
                console.error(e);
            }
        };
        if (artiStatus === 'ready') {
            fetchArtiDetails();
            const interval = setInterval(fetchArtiDetails, 10000);
            return () => clearInterval(interval);
        }
    }, [artiStatus]);

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
                        {circuitCount}
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
                {/* Services Section */}
                <GlassCard className="col-span-1">
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-lg font-semibold flex items-center gap-2">
                            <Server className="text-accent-cyan" />
                            Station Services
                        </h2>
                    </div>
                    <div className="flex flex-col gap-3">
                        <ServiceItem name="Arti Tor" status={artiStatus === 'ready'} desc="Network Proxy" />
                        <ServiceItem name="Guardian" status={stats?.guardian_connected || false} desc="Packet Filter" />
                        <ServiceItem name="Zcash (Zebra)" status={cryptoStatus?.zen.connected || false} desc="Privacy Node" />
                        <ServiceItem name="Monero" status={cryptoStatus?.monero.connected || false} desc="Privacy Node" />
                    </div>
                </GlassCard>

                {/* Active Circuits */}
                <GlassCard className="col-span-2 relative overflow-hidden group cursor-pointer" onClick={() => window.location.hash = '#/circuits'}>
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-lg font-semibold flex items-center gap-2">
                            <Zap className="text-accent-blue" />
                            Active Circuits
                        </h2>
                        <span className="text-sm font-mono text-accent-green">VIEW MAP &rarr;</span>
                    </div>
                    <div className="h-48 flex items-center justify-center border border-dashed border-glass-border rounded-lg bg-bg-tertiary/30">
                        <div className="flex flex-col items-center gap-2 text-text-muted">
                            <Globe size={48} className="opacity-20 animate-pulse" />
                            <span>{circuitCount} Active Paths Detected</span>
                        </div>
                    </div>
                    <div className="absolute top-0 left-0 w-full h-1 bg-gradient-primary opacity-50 transition-opacity" />
                </GlassCard>

                {/* Metrics */}
                <GlassCard className="col-span-1">
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-lg font-semibold flex items-center gap-2">
                            <Activity className="text-accent-purple" />
                            System Load
                        </h2>
                    </div>
                    <div className="flex flex-col gap-4">
                        <MetricBar label="CPU" value={24} color="bg-accent-blue" />
                        <MetricBar label="MEM" value={68} color="bg-accent-purple" />
                        <MetricBar label="NET" value={12} color="bg-accent-green" />
                    </div>
                </GlassCard>

                {/* Guardian Details */}
                <GlassCard className="col-span-2 border-accent-green/30 hover:border-accent-green cursor-pointer" onClick={() => window.location.hash = '#/guardian'}>
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-lg font-semibold flex items-center gap-2">
                            <ShieldCheck className="text-accent-green" />
                            Guardian Network Shield
                        </h2>
                        <span className={`badge ${stats?.guardian_connected ? 'bg-green-500/10 text-green-500' : 'bg-red-500/10 text-red-500'} px-2 py-1 rounded text-xs`}>
                            {stats?.guardian_connected ? 'ACTIVE' : 'OFFLINE'}
                        </span>
                    </div>
                    <div className="grid grid-cols-3 gap-4">
                        <div className="p-3 bg-bg-tertiary/50 rounded-lg">
                            <div className="text-xs text-text-muted mb-1">Protection</div>
                            <div className="font-bold text-accent-green">STRICT</div>
                        </div>
                        <div className="p-3 bg-bg-tertiary/50 rounded-lg">
                            <div className="text-xs text-text-muted mb-1">DNS Filter</div>
                            <div className="font-bold text-white">REJECT</div>
                        </div>
                        <div className="p-3 bg-bg-tertiary/50 rounded-lg">
                            <div className="text-xs text-text-muted mb-1">History</div>
                            <div className="font-bold text-white">{stats?.leaks_detected || 0} event(s)</div>
                        </div>
                    </div>
                </GlassCard>
            </div>
        </div>
    );
};

const ServiceItem = ({ name, status, desc }: { name: string, status: boolean, desc: string }) => (
    <div className="p-3 rounded-lg bg-bg-tertiary/50 border border-glass-border flex justify-between items-center group hover:border-accent-blue/30 transition-colors">
        <div className="flex items-center gap-3">
            <div className={`w-2 h-2 rounded-full ${status ? 'bg-accent-green shadow-[0_0_8px_rgba(16,185,129,0.5)]' : 'bg-red-500 animate-pulse'}`}></div>
            <div>
                <div className="font-mono text-sm text-white">{name}</div>
                <div className="text-[10px] text-text-muted">{desc}</div>
            </div>
        </div>
        <div className={`text-[10px] font-bold ${status ? 'text-accent-green' : 'text-red-500'}`}>
            {status ? 'UP' : 'DOWN'}
        </div>
    </div>
);

const MetricBar = ({ label, value, color }: { label: string, value: number, color: string }) => (
    <div className="flex flex-col gap-1">
        <div className="flex justify-between text-[10px] font-mono text-text-secondary uppercase">
            <span>{label}</span>
            <span>{value}%</span>
        </div>
        <div className="h-1.5 w-full bg-white/5 rounded-full overflow-hidden">
            <div className={`h-full ${color} transition-all duration-1000`} style={{ width: `${value}%` }} />
        </div>
    </div>
);
