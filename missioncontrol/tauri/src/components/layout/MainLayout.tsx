import React from 'react';
import { Sidebar } from './Sidebar';
import { Header } from './Header';

interface MainLayoutProps {
    children: React.ReactNode;
}

export const MainLayout: React.FC<MainLayoutProps> = ({ children }) => {
    return (
        <div className="flex h-screen w-screen overflow-hidden bg-bg-primary text-text-primary">
            <Sidebar />
            <div className="flex-1 flex flex-col h-full overflow-hidden relative">
                <Header />
                <main className="flex-1 overflow-y-auto p-6 relative z-0">
                    {children}
                </main>
            </div>
        </div>
    );
};
