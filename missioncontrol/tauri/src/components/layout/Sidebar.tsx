import React from 'react';
import { NavLink } from 'react-router-dom';
import { Home, CircuitBoard, Cloud, BarChart3, Grip, Shield, Hexagon } from 'lucide-react';
import { clsx } from 'clsx';

interface SidebarItemProps {
    icon: React.ElementType;
    label: string;
    to: string;
}

const SidebarItem: React.FC<SidebarItemProps> = ({ icon: Icon, label, to }) => {
    return (
        <NavLink
            to={to}
            className={({ isActive }) => clsx(
                "flex items-center gap-3 px-4 py-3 cursor-pointer transition-all duration-200 decoration-0",
                "hover:bg-white/5",
                isActive && "bg-white/10 border-l-2 border-accent-blue text-white shadow-[0_0_15px_rgba(74,158,255,0.2)]",
                !isActive && "text-text-secondary hover:text-white"
            )}
        >
            <Icon size={20} className="text-current" />
            <span className="font-medium tracking-wide">{label}</span>
        </NavLink>
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
                <SidebarItem icon={Home} label="Home" to="/" />
                <SidebarItem icon={CircuitBoard} label="Circuits" to="/circuits" />
                <SidebarItem icon={Cloud} label="Services" to="/services" />
                <SidebarItem icon={BarChart3} label="Metrics" to="/metrics" />
                <SidebarItem icon={Grip} label="Integrations" to="/integrations" />
                <SidebarItem icon={Shield} label="Guardian" to="/guardian" />
                <SidebarItem icon={Hexagon} label="DarkMatter" to="/darkmatter" />
            </div>

            <div className="p-4 border-t border-glass-border">
                <div className="text-xs font-mono text-text-muted text-center">
                    v0.2.0-tauri
                </div>
            </div>
        </div>
    );
};
