"use client"

import { useState, useEffect } from "react"
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

export function GameSidebar({ collapsed: propCollapsed, setCollapsed, isMobileView = false }: SidebarProps) {
    const [isHovered, setIsHovered] = useState(false);
    const [collapsed, setLocalCollapsed] = useState(propCollapsed); 
    useEffect(() => setCollapsed(collapsed), [collapsed]);
    useEffect(() => setLocalCollapsed(propCollapsed), [propCollapsed]);

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

    // Desktop sidebar with hover functionality
    return (
        <div
            className={`fixed left-0 top-0 h-full bg-gray-900/95 backdrop-blur-sm border-r border-gray-800/50 flex-col transition-all duration-500 ease-in-out hidden md:flex ${
                collapsed && !isHovered ? "w-16" : "w-64"
            } shadow-xl group z-50`}
            onMouseEnter={() => {
                setIsHovered(true);
                setLocalCollapsed(false);
            }}
            onMouseLeave={() => {
                setIsHovered(false);
                setLocalCollapsed(true);
            }}
        >
            <div className="p-4 flex items-center justify-center overflow-hidden">
                <div className={`transition-all duration-500 ease-in-out ${collapsed && !isHovered ? "w-16" : "w-full"}`}>
                    <div className="w-16 h-16 relative transform hover:scale-105 transition-transform duration-300">
                        <Image src="/images/StarkmateLogo.png" alt="StarkMate" fill className="object-contain drop-shadow-lg" />
                    </div>
                </div>
            </div>

            <nav className="flex-1 px-2 overflow-hidden">
                <SidebarItem icon={<ChessIcon />} label="Play" collapsed={collapsed && !isHovered} active />
                <SidebarItem icon={<WatchIcon />} label="Watch" collapsed={collapsed && !isHovered} />
                <SidebarItem icon={<NewsIcon />} label="News" collapsed={collapsed && !isHovered} />
                <SidebarItem icon={<UserIcon />} label="Profile" collapsed={collapsed && !isHovered} />
                <SidebarItem icon={<SettingsIcon />} label="Settings" collapsed={collapsed && !isHovered} />
                <SidebarItem icon={<SupportIcon />} label="Support" collapsed={collapsed && !isHovered} />
            </nav>

            <div className={`p-4 space-y-3 overflow-hidden transition-all duration-500 ease-in-out ${
                collapsed && !isHovered ? "opacity-0" : "opacity-100"
            }`}>
                <Button className="w-full bg-gradient-to-r from-teal-500 to-blue-700 hover:from-teal-600 hover:to-blue-800 text-white shadow-lg hover:shadow-teal-500/20 transition-all duration-300 rounded-lg">
                    <div className="flex items-center">
                        <WalletIcon className="transform group-hover:scale-110 transition-transform duration-300" />
                        <span className={`ml-2 transition-opacity duration-500 ${collapsed && !isHovered ? "opacity-0" : "opacity-100"}`}>
                            Connect Wallet
                        </span>
                    </div>
                </Button>
                <div className={`space-y-2 transition-all duration-500 ${collapsed && !isHovered ? "scale-0" : "scale-100"}`}>
                    <Button className="w-full bg-teal-600 hover:bg-teal-700 shadow-lg hover:shadow-teal-600/20 transition-all duration-300 rounded-lg">
                        Sign Up
                    </Button>
                    <Button variant="outline" className="w-full border-gray-700 text-gray-300 hover:bg-gray-800/50 hover:border-teal-500/50 transition-all duration-300 rounded-lg">
                        Log In
                    </Button>
                </div>
            </div>
        </div>
    )
}

// Update SidebarItem component
interface SidebarItemProps {
    icon: React.ReactNode
    label: string
    collapsed?: boolean
    active?: boolean
}

// Keep only this version of SidebarItem and remove the duplicate
function SidebarItem({ icon, label, collapsed, active }: SidebarItemProps) {
    return (
        <Link
            href="#"
            className={`flex items-center p-3 ${
                collapsed ? "justify-center" : "px-4"
            } hover:bg-gray-800/50 transition-all duration-300 rounded-lg mb-1 group overflow-hidden ${
                active ? "bg-gray-800/50 shadow-lg" : ""
            }`}
        >
            <span className={`${
                active ? "text-teal-400" : "text-gray-300"
            } transition-all duration-300 group-hover:text-teal-400 transform group-hover:scale-110 min-w-[24px]`}>
                {icon}
            </span>
            <span className={`ml-3 text-sm font-medium text-gray-300 group-hover:text-white transition-all duration-500 ${
                collapsed ? "opacity-0 w-0" : "opacity-100 w-auto"
            } whitespace-nowrap`}>
                {label}
            </span>
        </Link>
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

// Keep MobileSidebarItem as is
function MobileSidebarItem({ icon, label, active }: SidebarItemProps) {
    return (
        <Link
            href="#"
            className={`flex items-center p-3 px-4 hover:bg-gray-800/50 transition-all duration-300 rounded-lg mb-1 group ${
                active ? "bg-gray-800/50 shadow-lg" : ""
            }`}
        >
            <span
                className={`${
                    active ? "text-teal-400" : "text-gray-300"
                } transition-all duration-300 group-hover:text-teal-400 transform group-hover:scale-110`}
            >
                {icon}
            </span>
            <span className="ml-3 text-sm font-medium text-gray-300 group-hover:text-white transition-colors duration-300">
                {label}
            </span>
        </Link>
    )
}
