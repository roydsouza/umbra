import { GlassCard } from '../components/ui/GlassCard';
import { Grip } from 'lucide-react';

export const Integrations = () => {
    return (
        <div className="h-full flex flex-col gap-6">
            <h1 className="text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-accent-green to-accent-blue">
                Integrations
            </h1>
            <GlassCard className="p-8 flex flex-col items-center justify-center gap-4 opacity-70">
                <Grip size={64} className="text-accent-green opacity-50" />
                <h2 className="text-2xl font-bold text-white">External Systems</h2>
                <p className="text-text-muted text-center max-w-md">
                    Configuration for Zcash, Monero, and other external node integrations coming soon.
                </p>
            </GlassCard>
        </div>
    );
};
