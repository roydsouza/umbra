import { GlassCard } from '../components/ui/GlassCard';
import { Cloud } from 'lucide-react';

export const Services = () => {
    return (
        <div className="h-full flex flex-col gap-6">
            <h1 className="text-3xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-accent-cyan to-accent-blue">
                Services
            </h1>
            <GlassCard className="p-8 flex flex-col items-center justify-center gap-4 opacity-70">
                <Cloud size={64} className="text-accent-cyan opacity-50" />
                <h2 className="text-2xl font-bold text-white">Service Management</h2>
                <p className="text-text-muted text-center max-w-md">
                    Advanced service control and configuration coming soon.
                    Currently managed via Dashboard.
                </p>
            </GlassCard>
        </div>
    );
};
