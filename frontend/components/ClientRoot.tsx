"use client";

import { useState, useEffect } from "react";
import { GameSidebar } from "@/components/GameSidebar";

export default function ClientRoot({ children }: { children: React.ReactNode }) {
    const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);
    const [isMobile, setIsMobile] = useState(false);

    useEffect(() => {
        const checkIfMobile = () => {
            setIsMobile(window.innerWidth < 768);
        };

        checkIfMobile();
        window.addEventListener("resize", checkIfMobile);

        return () => window.removeEventListener("resize", checkIfMobile);
    }, []);

    return (
        <div className="flex h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900 text-white">
            <GameSidebar collapsed={isSidebarCollapsed} setCollapsed={setIsSidebarCollapsed} />
            <main className="flex-1 overflow-auto">
                <div className="md:hidden flex items-center p-4 border-b border-gray-800">
                    <GameSidebar isMobileView={true} collapsed={false} setCollapsed={() => { }} />
                    <div className="ml-4 flex items-center">
                        <div className="h-16 w-16 relative">
                            <img src="/images/StarkmateLogo.png" alt="StarkMate" className="object-contain" />
                        </div>
                    </div>
                </div>
                <div className="container mx-auto p-4 md:p-8">{children}</div>
            </main>
        </div>
    );
}
