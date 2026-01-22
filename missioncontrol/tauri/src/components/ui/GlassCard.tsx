import React from 'react';
import { clsx } from 'clsx';

interface GlassCardProps {
    children: React.ReactNode;
    className?: string;
    hoverEffect?: boolean;
    onClick?: () => void;
}

export const GlassCard: React.FC<GlassCardProps> = ({ children, className, hoverEffect = true, onClick }) => {
    return (
        <div
            onClick={onClick}
            className={clsx(
                "p-6 rounded-xl border border-glass-border bg-bg-card backdrop-blur-md relative overflow-hidden",
                hoverEffect && "transition-all duration-300 hover:-translate-y-1 hover:border-accent-blue hover:shadow-[0_0_20px_rgba(74,158,255,0.2)]",
                className
            )}>
            {children}
        </div>
    );
};
