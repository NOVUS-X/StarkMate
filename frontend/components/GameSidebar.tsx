"use client"

import type React from "react"
import Image from "next/image"
import Link from "next/link"
import { Button } from "@/components/ui/button"
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet"
import {
    ChessIcon,
    WatchIcon,
    NewsIcon,
    UserIcon,
    SettingsIcon,
    SupportIcon,
    CollapseIcon,
    WalletIcon,
    MenuIcon,
} from "@/components/icons"

interface SidebarProps {
    collapsed: boolean
    setCollapsed: (collapsed: boolean) => void
    isMobileView?: boolean
}

export function GameSidebar({ collapsed, setCollapsed, isMobileView = false }: SidebarProps) {
    // For mobile view, we'll use a Sheet component
    if (isMobileView) {
        return (
            <Sheet>
                <SheetTrigger asChild>
                    <Button variant="ghost" size="icon" className="md:hidden">
                        <MenuIcon />
                        <span className="sr-only">Toggle menu</span>
                    </Button>
                </SheetTrigger>
                <SheetContent side="left" className="p-0 w-64 bg-gray-900 border-r border-gray-800">
                    <MobileSidebar />
                </SheetContent>
            </Sheet>
        )
    }

    // Desktop sidebar
    return (
        <div
            className={`bg-gray-900 border-r border-gray-800 flex-col transition-all duration-300 hidden md:flex ${collapsed ? "w-16" : "w-64"
                }`}
        >
            <div className="p-4 flex items-center justify-center">
                {collapsed ? (
                    <div className="w-16 h-16 relative">
                        <Image src="/images/StarkmateLogo.png" alt="StarkMate" fill className="object-contain" />
                    </div>
                ) : (
                    <div className="flex items-center space-x-2">
                        <div className="w-16 h-16 relative">
                            <Image src="/images/StarkmateLogo.png" alt="StarkMate" fill className="object-contain" />
                        </div>
                    </div>
                )}
            </div>

            <nav className="flex-1">
                <SidebarItem icon={<ChessIcon />} label="Play" collapsed={collapsed} active />
                <SidebarItem icon={<WatchIcon />} label="Watch" collapsed={collapsed} />
                <SidebarItem icon={<NewsIcon />} label="News" collapsed={collapsed} />
                <SidebarItem icon={<UserIcon />} label="Profile" collapsed={collapsed} />
                <SidebarItem icon={<SettingsIcon />} label="Settings" collapsed={collapsed} />
                <SidebarItem icon={<SupportIcon />} label="Support" collapsed={collapsed} />
            </nav>

            <div className="p-4 space-y-2">
                <Button className="w-full bg-gradient-to-r from-teal-500 to-blue-700 hover:from-teal-600 hover:to-blue-800 text-white">
                    {collapsed ? (
                        <WalletIcon />
                    ) : (
                        <div className="flex items-center">
                            <WalletIcon />
                            <span className="ml-2">Connect Wallet</span>
                        </div>
                    )}
                </Button>
                {!collapsed && (
                    <>
                        <Button className="w-full bg-teal-600 hover:bg-teal-700">Sign Up</Button>
                        <Button variant="outline" className="w-full border-gray-700 text-gray-300">
                            Log In
                        </Button>
                    </>
                )}
            </div>

            <div className="border-t border-gray-800 p-2">
                <button
                    onClick={() => setCollapsed(!collapsed)}
                    className="p-2 w-full flex items-center justify-center text-gray-400 hover:text-white hover:bg-gray-800 rounded-md transition-all duration-300 group"
                >
                    <div className={`transform transition-transform duration-300 ${collapsed ? "rotate-180" : ""}`}>
                        <CollapseIcon />
                    </div>
                    {!collapsed && (
                        <span className="ml-2 transition-opacity duration-300 group-hover:text-teal-400">Collapse</span>
                    )}
                </button>
            </div>
        </div>
    )
}

// Mobile sidebar component (always expanded)
function MobileSidebar() {
    return (
        <div className="flex flex-col h-full bg-gray-900">
            <div className="p-4 flex items-center space-x-2">
                <div className="w-16 h-16 relative">
                    <Image src="/images/StarkmateLogo.png" alt="StarkMate" fill className="object-contain" />
                </div>
            </div>

            <nav className="flex-1">
                <MobileSidebarItem icon={<ChessIcon />} label="Play" active />
                <MobileSidebarItem icon={<WatchIcon />} label="Watch" />
                <MobileSidebarItem icon={<NewsIcon />} label="News" />
                <MobileSidebarItem icon={<UserIcon />} label="Profile" />
                <MobileSidebarItem icon={<SettingsIcon />} label="Settings" />
                <MobileSidebarItem icon={<SupportIcon />} label="Support" />
            </nav>

            <div className="p-4 space-y-2">
                <Button className="w-full bg-gradient-to-r from-teal-500 to-blue-700 hover:from-teal-600 hover:to-blue-800 text-white">
                    <div className="flex items-center">
                        <WalletIcon />
                        <span className="ml-2">Connect Wallet</span>
                    </div>
                </Button>
                <Button className="w-full bg-teal-600 hover:bg-teal-700">Sign Up</Button>
                <Button variant="outline" className="w-full border-gray-700 text-gray-300">
                    Log In
                </Button>
            </div>
        </div>
    )
}

interface SidebarItemProps {
    icon: React.ReactNode
    label: string
    collapsed?: boolean
    active?: boolean
}

function SidebarItem({ icon, label, collapsed, active }: SidebarItemProps) {
    return (
        <Link
            href="#"
            className={`flex items-center p-3 ${collapsed ? "justify-center" : "px-4"
                } hover:bg-gray-800 transition-colors duration-200 ${active ? "bg-gray-800" : ""}`}
        >
            <span
                className={`${active ? "text-teal-400" : "text-gray-300"} transition-colors duration-200 hover:text-teal-400`}
            >
                {icon}
            </span>
            {!collapsed && <span className="ml-3 text-sm text-white font-medium">{label}</span>}
        </Link>
    )
}

function MobileSidebarItem({ icon, label, active }: SidebarItemProps) {
    return (
        <Link
            href="#"
            className={`flex items-center p-3 px-4 hover:bg-gray-800 transition-colors duration-200 ${active ? "bg-gray-800" : ""
                }`}
        >
            <span
                className={`${active ? "text-teal-400" : "text-gray-300"} transition-colors duration-200 hover:text-teal-400`}
            >
                {icon}
            </span>
            <span className="ml-3 text-white text-sm font-medium">{label}</span>
        </Link>
    )
}
