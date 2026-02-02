import { GlassCard } from '../components/ui/GlassCard';
import { BarChart3 } from 'lucide-react';

export const Metrics = () => {
    return (
        <div className="h-full flex flex-col gap-6">
            <h1 className="text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-accent-purple to-accent-pink">
                System Metrics
            </h1>
            <GlassCard className="p-8 flex flex-col items-center justify-center gap-4 opacity-70">
                <BarChart3 size={64} className="text-accent-purple opacity-50" />
                <h2 className="text-2xl font-bold text-white">Detailed Telemetry</h2>
                <p className="text-text-muted text-center max-w-md">
                    Real-time system resource usage and historical data visualization coming soon.
                </p>
            </GlassCard>
        </div>
    );
};
