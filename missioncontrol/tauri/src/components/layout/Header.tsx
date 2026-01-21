import React from 'react';
import { useArtiStatus } from '../../hooks/useArtiStatus';

const StatusIndicator: React.FC<{ label: string; status: 'active' | 'inactive' | 'warning' | 'error' | 'init' }> = ({ label, status }) => {
    let colorClass = 'bg-accent-green';
    let textClass = 'text-accent-green';
    let labelText = status.toUpperCase();

    if (status === 'inactive' || status === 'init') {
        colorClass = 'bg-text-muted';
        textClass = 'text-text-muted';
        labelText = status === 'init' ? 'INIT...' : 'INACTIVE';
    } else if (status === 'warning') {
        colorClass = 'bg-accent-yellow';
        textClass = 'text-accent-yellow';
    } else if (status === 'error') {
        colorClass = 'bg-accent-red';
        textClass = 'text-accent-red';
    }

    return (
        <div className={`flex items-center gap-2 px-3 py-1.5 rounded-full bg-bg-tertiary border border-glass-border`}>
            <span className="text-xs font-mono text-text-secondary">{label}:</span>
            <span className={`text-xs font-bold ${textClass}`}>{labelText}</span>
            <div className={`w-2 h-2 rounded-full ${colorClass} animate-pulse shadow-[0_0_8px_currentColor]`}></div>
        </div>
    );
};

export const Header: React.FC = () => {
    const { status: artiStatus } = useArtiStatus();

    // Map hook status to indicator status
    const indicatorStatus = artiStatus === 'ready' ? 'active' : artiStatus;

    return (
        <div className="h-16 border-b border-glass-border glass-panel flex items-center justify-between px-6 z-10 relative">
            <div className="flex items-center gap-4">
                <div className="text-sm font-mono text-text-secondary">
                    MissionControl // Status Level: <span className="text-accent-green font-bold">NOMINAL</span>
                </div>
            </div>

            <div className="flex items-center gap-4">
                <StatusIndicator label="Arti" status={indicatorStatus as any} />
                <StatusIndicator label="Shield" status="active" /> {/* Mocked for now */}
            </div>
        </div>
    );
};
