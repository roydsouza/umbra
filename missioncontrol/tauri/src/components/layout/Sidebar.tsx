import React from 'react';
import { Home, CircuitBoard, Cloud, BarChart3, Grip, Shield } from 'lucide-react';
import { clsx } from 'clsx';

interface SidebarItemProps {
    icon: React.ElementType;
    label: string;
    active?: boolean;
}

const SidebarItem: React.FC<SidebarItemProps> = ({ icon: Icon, label, active }) => {
    return (
        <div
            className={clsx(
                "flex items-center gap-3 px-4 py-3 cursor-pointer transition-all duration-200",
                "hover:bg-white/5",
                active && "bg-white/10 border-l-2 border-accent-blue text-white shadow-[0_0_15px_rgba(74,158,255,0.2)]",
                !active && "text-text-secondary hover:text-white"
            )}
        >
            <Icon size={20} className={clsx(active ? "text-accent-blue" : "text-text-secondary")} />
            <span className="font-medium tracking-wide">{label}</span>
        </div>
    );
};

export const Sidebar: React.FC = () => {
    return (
        <div className="w-64 h-full border-r border-glass-border glass-panel flex flex-col">
            <div className="p-6 border-b border-glass-border mb-2">
                <h1 className="text-xl font-bold font-mono tracking-tighter text-transparent bg-clip-text bg-gradient-to-r from-accent-blue to-accent-purple">
                    MissionControl
                </h1>
            </div>

            <div className="flex-1 py-4 flex flex-col gap-1">
                <SidebarItem icon={Home} label="Home" active />
                <SidebarItem icon={CircuitBoard} label="Circuits" />
                <SidebarItem icon={Cloud} label="Services" />
                <SidebarItem icon={BarChart3} label="Metrics" />
                <SidebarItem icon={Grip} label="Integrations" />
                <SidebarItem icon={Shield} label="Guardian" />
            </div>

            <div className="p-4 border-t border-glass-border">
                <div className="text-xs font-mono text-text-muted text-center">
                    v0.2.0-tauri
                </div>
            </div>
        </div>
    );
};
