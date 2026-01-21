import { useRef } from 'react';
import { Shield, Radio, Lock, Activity, WifiOff } from 'lucide-react';
import { useVirtualizer } from '@tanstack/react-virtual';
import { GlassCard } from '../components/ui/GlassCard';
import { useGuardianEvents } from '../hooks/useGuardianEvents';
import type { LeakEvent } from '../hooks/useGuardianEvents';
import { useSystemStats } from '../hooks/useSystemStats';

const EventRow = ({ event }: { event: LeakEvent }) => (
    <div className={`p-3 border-b border-glass-border flex items-center justify-between font-mono text-sm
        ${event.severity === 'CRITICAL' ? 'bg-red-500/10 text-red-200' : 'text-text-secondary hover:bg-white/5'}
    `}>
        <div className="flex items-center gap-3">
            <span className="text-xs opacity-50">{new Date(Number(event.timestamp || 0) * 1000).toLocaleTimeString()}</span>
            <span className={`font-bold ${event.severity === 'CRITICAL' ? 'text-red-400' : 'text-accent-blue'}`}>
                {event.process_name || 'unknown'}
            </span>
            <span className="text-xs opacity-50">PID: {event.process_pid}</span>
        </div>
        <div className="flex items-center gap-4">
            <span className="text-white">{event.dest_ip}:{event.dest_port}</span>
            <span className={`px-2 py-0.5 rounded text-[10px] font-bold uppercase
                ${event.event_type === 'dns' ? 'bg-accent-purple/20 text-accent-purple' : 'bg-gray-700/50 text-gray-400'}
            `}>
                {event.event_type || 'net'}
            </span>
        </div>
    </div>
);

export const Guardian = () => {
    const { events } = useGuardianEvents();
    const { stats } = useSystemStats();

    // Virtualizer Setup
    const parentRef = useRef<HTMLDivElement>(null);
    const rowVirtualizer = useVirtualizer({
        count: events.length,
        getScrollElement: () => parentRef.current,
        estimateSize: () => 50, // Approx height of row
    });

    return (
        <div className="h-[calc(100vh-100px)] flex flex-col gap-6">
            {/* Control Panel */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <GlassCard className="p-6 flex flex-col gap-4 border-l-4 border-l-accent-green">
                    <div className="flex justify-between items-start">
                        <div>
                            <h2 className="text-xl font-bold flex items-center gap-2">
                                <Shield className="text-accent-green" />
                                Network Shield
                            </h2>
                            <p className="text-sm text-text-muted mt-1">Global Traffic Filter</p>
                        </div>
                        <div className={`w-3 h-3 rounded-full animate-pulse ${stats?.guardian_connected ? 'bg-accent-green' : 'bg-red-500'}`} />
                    </div>
                    <div className="flex items-center gap-2 mt-2">
                        <span className="text-2xl font-mono font-bold">ACTIVE</span>
                        <span className="text-xs px-2 py-1 bg-accent-green/10 text-accent-green rounded border border-accent-green/20">PROTECTED</span>
                    </div>
                </GlassCard>

                <GlassCard className="p-6 flex flex-col gap-4">
                    <div className="flex justify-between items-start">
                        <div>
                            <h2 className="text-lg font-semibold flex items-center gap-2">
                                <Radio className="text-accent-purple" />
                                Monitor Mode
                            </h2>
                            <p className="text-sm text-text-muted mt-1">PKTAP Interface</p>
                        </div>
                    </div>
                    <div className="mt-2 font-mono text-sm text-text-secondary">
                        Interface: <span className="text-white">pktap0</span>
                        <br />
                        Filter: <span className="text-accent-purple">Strict (Tor-Only)</span>
                    </div>
                </GlassCard>

                <GlassCard className="p-6 flex flex-col gap-4">
                    <div className="flex justify-between items-start">
                        <div>
                            <h2 className="text-lg font-semibold flex items-center gap-2">
                                <Lock className="text-accent-red" />
                                Leak Prevention
                            </h2>
                            <p className="text-sm text-text-muted mt-1">Last 24 Hours</p>
                        </div>
                    </div>
                    <div className="mt-2 text-3xl font-mono font-bold text-white">
                        {events.length} <span className="text-sm font-sans font-normal text-text-muted">events</span>
                    </div>
                </GlassCard>
            </div>

            {/* Live Feed */}
            <GlassCard className="flex-1 flex flex-col overflow-hidden">
                <div className="p-4 border-b border-glass-border flex justify-between items-center bg-white/5">
                    <h3 className="font-semibold flex items-center gap-2">
                        <Activity size={18} className="text-accent-blue" />
                        Live Traffic Feed
                    </h3>
                    <div className="flex gap-2 text-xs">
                        <span className="flex items-center gap-1 text-text-muted"><WifiOff size={12} /> Clearnet Blocked</span>
                    </div>
                </div>

                <div ref={parentRef} className="flex-1 overflow-y-auto">
                    <div
                        style={{
                            height: `${rowVirtualizer.getTotalSize()}px`,
                            width: '100%',
                            position: 'relative',
                        }}
                    >
                        {rowVirtualizer.getVirtualItems().map((virtualRow) => (
                            <div
                                key={virtualRow.key}
                                style={{
                                    position: 'absolute',
                                    top: 0,
                                    left: 0,
                                    width: '100%',
                                    height: `${virtualRow.size}px`,
                                    transform: `translateY(${virtualRow.start}px)`,
                                }}
                            >
                                <EventRow event={events[virtualRow.index]} />
                            </div>
                        ))}
                    </div>

                    {events.length === 0 && (
                        <div className="h-full flex flex-col items-center justify-center text-text-muted">
                            <Activity size={48} className="opacity-20 mb-4" />
                            <p>No traffic events detected yet...</p>
                        </div>
                    )}
                </div>
            </GlassCard>
        </div>
    );
};
