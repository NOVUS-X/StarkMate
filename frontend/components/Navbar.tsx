"use client";
import { useState, useEffect } from "react";
import Link from "next/link";
import { Button } from '@/components/ui'
// Define types for props
interface NavLinkProps {
  href: string;
  label: string;
}

interface DropdownItemProps {
  href: string;
  label: string;
}

interface MobileNavLinkProps {
  href: string;
  label: string;
  indented?: boolean;
}

const Navbar = () => {
  const [isScrolled, setIsScrolled] = useState(false);
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [walletConnected, setWalletConnected] = useState(false);
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);

  useEffect(() => {
    const handleScroll = () => {
      if (window.scrollY > 10) {
        setIsScrolled(true);
      } else {
        setIsScrolled(false);
      }
    };

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, []);

  const toggleMobileMenu = () => {
    setIsMobileMenuOpen(!isMobileMenuOpen);
  };

  const toggleWalletConnection = () => {
    setWalletConnected(!walletConnected);
  };

  const toggleDropdown = () => {
    setIsDropdownOpen(!isDropdownOpen);
  };

  return (
    <nav
      className={`fixed w-full z-50 transition-all duration-300 ${isScrolled
        ? "bg-gray-900/90 backdrop-blur-md border-b border-cyan-500/20 py-2"
        : "bg-transparent py-4"
        }`}
    >
      <div className="container mx-auto px-4">
        <div className="flex items-center justify-between">
          {/* Logo */}
          <Link href="/" className="flex items-center space-x-2">
            <div className="relative w-8 h-8 overflow-hidden">
              <div className="absolute inset-0 bg-gradient-to-br from-cyan-500 to-purple-500 rounded-md"></div>
              <div className="absolute inset-1 bg-gray-900 rounded-sm flex items-center justify-center">
                <span className="text-cyan-500 font-bold text-lg">C</span>
              </div>
              <div className="absolute top-0 left-0 w-full h-full opacity-50 bg-gradient-to-t from-transparent to-white/30"></div>
            </div>
            <span className="text-white font-bold text-xl tracking-tight">
              Stark<span className="text-cyan-500">Mate</span>
            </span>
          </Link>

          {/* Desktop Navigation */}
          <div className="hidden md:flex items-center space-x-1">
            <NavLink href="/play" label="Play" />
            <NavLink href="/leaderboard" label="Leaderboard" />
            <NavLink href="/nft-gallery" label="NFT Gallery" />

            {/* Games Dropdown */}
            <div className="relative">
              <button
                onClick={toggleDropdown}
                className="px-4 py-2 text-gray-300 hover:text-white transition-colors flex items-center space-x-1 group"
              >
                <span>Game Modes</span>
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className={`h-4 w-4 transition-transform ${isDropdownOpen ? "rotate-180" : ""
                    }`}
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M19 9l-7 7-7-7"
                  />
                </svg>
                {/* Hover indicator */}
                <span className="absolute bottom-0 left-0 w-0 h-0.5 bg-cyan-500 group-hover:w-full transition-all duration-300"></span>
              </button>

              {/* Dropdown Menu */}
              {isDropdownOpen && (
                <div className="absolute top-full right-0 mt-1 w-48 rounded-md overflow-hidden">
                  <div className="bg-gray-900 border border-gray-800 backdrop-blur-lg shadow-lg shadow-cyan-500/20 animate-fadeIn">
                    <div className="absolute inset-0 bg-cyan-500/5"></div>
                    <div className="relative">
                      <DropdownItem href="/game/classic" label="Classic" />
                      <DropdownItem href="/game/blitz" label="Blitz" />
                      <DropdownItem href="/game/bullet" label="Bullet" />
                      <DropdownItem
                        href="/game/tournaments"
                        label="Tournaments"
                      />
                    </div>
                  </div>
                </div>
              )}
            </div>

            <NavLink href="/dao" label="Governance" />
          </div>

          {/* Connect Wallet Button */}
          <div className="hidden md:block w-[160px]">
            <Button
              size="sm"
              onClick={toggleWalletConnection}
              className="w-full"
            >
              <div className="relative w-4 h-4 mr-1">
                {walletConnected ? (
                  <div className="absolute inset-0 mr-2 flex items-center justify-center">
                    <div className="w-2 h-2 rounded-full bg-green-400 animate-pulse"></div>
                  </div>
                ) : (
                  <svg
                    xmlns="http://www.w3.org/2000/svg"
                    className="h-4 w-4"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2"
                    />
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M20 9v6a2 2 0 01-2 2H9a2 2 0 01-2-2V9a2 2 0 012-2h9a2 2 0 012 2z"
                    />
                  </svg>
                )}
              </div>
              <span className="text-xs">{walletConnected ? "0xF3...7D21" : "Connect Wallet"}</span>
            </Button>
          </div>

          {/* Mobile Menu Button */}
          <div className="md:hidden">
            <button
              onClick={toggleMobileMenu}
              className="text-gray-300 hover:text-white focus:outline-none"
            >
              <div className="w-6 h-6 relative">
                <span
                  className={`absolute h-0.5 w-full bg-current transform transition-all duration-300 ${isMobileMenuOpen ? "rotate-45 top-3" : "top-1"
                    }`}
                ></span>
                <span
                  className={`absolute h-0.5 w-full bg-current transform transition-all duration-300 ${isMobileMenuOpen ? "opacity-0" : "top-3"
                    }`}
                ></span>
                <span
                  className={`absolute h-0.5 w-full bg-current transform transition-all duration-300 ${isMobileMenuOpen ? "-rotate-45 top-3" : "top-5"
                    }`}
                ></span>
              </div>
            </button>
          </div>
        </div>
      </div>

      {/* Mobile Menu */}
      <div
        className={`md:hidden transition-all duration-300 overflow-hidden ${isMobileMenuOpen ? "max-h-96" : "max-h-0"
          }`}
      >
        <div className="container mx-auto px-4 py-2">
          <div className="bg-gray-900/90 backdrop-blur-md border border-gray-800 rounded-xl overflow-hidden">
            <div className="py-2">
              <MobileNavLink href="/play" label="Play" />
              <MobileNavLink href="/leaderboard" label="Leaderboard" />
              <MobileNavLink href="/nft-gallery" label="NFT Gallery" />
              <div className="px-4 py-2 text-gray-400 text-sm font-medium">
                Game Modes
              </div>
              <MobileNavLink href="/game/classic" label="Classic" indented />
              <MobileNavLink href="/game/blitz" label="Blitz" indented />
              <MobileNavLink href="/game/bullet" label="Bullet" indented />
              <MobileNavLink
                href="/game/tournaments"
                label="Tournaments"
                indented
              />
              <MobileNavLink href="/dao" label="Governance" />

              <div className="w-[200px]">
                <Button
                  variant="primary"
                  size="lg"
                  onClick={toggleWalletConnection}
                >
                  <div className="relative w-4 h-4">
                    {walletConnected ? (
                      <div className="absolute inset-0 flex items-center justify-center">
                        <div className="w-2 h-2 rounded-full bg-green-400 animate-pulse"></div>
                      </div>
                    ) : (
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        className="h-4 w-4"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M17 9V7a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2"
                        />
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M20 9v6a2 2 0 01-2 2H9a2 2 0 01-2-2V9a2 2 0 012-2h9a2 2 0 012 2z"
                        />
                      </svg>
                    )}
                  </div>
                  <span>
                    {walletConnected ? "0xF3...7D21" : "Connect Wallet"}
                  </span>
                </Button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </nav>
  );
};

// Desktop Nav Link
const NavLink: React.FC<NavLinkProps> = ({ href, label }) => (
  <Link
    href={href}
    className="px-4 py-2 text-gray-300 hover:text-white transition-colors relative group"
  >
    {label}
    {/* Hover indicator */}
    <span className="absolute bottom-0 left-0 w-0 h-0.5 bg-cyan-500 group-hover:w-full transition-all duration-300"></span>
  </Link>
);

// Dropdown Item
const DropdownItem: React.FC<DropdownItemProps> = ({ href, label }) => (
  <Link
    href={href}
    className="block px-4 py-2 text-gray-300 hover:text-white hover:bg-gray-800 transition-colors text-sm"
  >
    {label}
  </Link>
);

// Mobile Nav Link
const MobileNavLink: React.FC<MobileNavLinkProps> = ({
  href,
  label,
  indented = false,
}) => (
  <Link
    href={href}
    className={`block px-4 py-2 text-gray-300 hover:text-white transition-colors hover:bg-gray-800 ${indented ? "pl-8 text-sm" : ""
      }`}
  >
    {label}
  </Link>
);

export default Navbar;
