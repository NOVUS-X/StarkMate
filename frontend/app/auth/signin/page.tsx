"use client";

import React from "react";
import { useAuth } from "@/hook/use-auth";
import { FcGoogle } from "react-icons/fc";
import { FaApple, FaWallet } from "react-icons/fa";

export default function SignIn() {
  const { loginWithGoogle, loginWithApple, loginWithWallet } = useAuth();

  const handleWalletLogin = async () => {
    // This is a placeholder - actual wallet connection logic would be implemented here
    // For now, we're just passing dummy values
    const address = "0x123...456";
    const message = "Sign this message to authenticate with StarkMate";
    const signature = "0xsignature";
    
    await loginWithWallet(address, signature, message);
  };

  return (
    <div className="flex min-h-screen flex-col items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8">
        <div>
          <h2 className="mt-6 text-center text-3xl font-bold tracking-tight">
            Sign in to StarkMate
          </h2>
        </div>
        <div className="mt-8 space-y-4">
          <button
            onClick={() => loginWithGoogle({ callbackUrl: "/" })}
            className="group relative flex w-full justify-center rounded-lg border border-gray-300 bg-white py-2 px-4 text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-cyan-500 focus:ring-offset-2"
          >
            <span className="flex items-center">
              <FcGoogle className="mr-2 h-5 w-5" />
              Sign in with Google
            </span>
          </button>
          
          <button
            onClick={() => loginWithApple({ callbackUrl: "/" })}
            className="group relative flex w-full justify-center rounded-lg border border-gray-800 bg-black py-2 px-4 text-sm font-medium text-white hover:bg-gray-900 focus:outline-none focus:ring-2 focus:ring-cyan-500 focus:ring-offset-2"
          >
            <span className="flex items-center">
              <FaApple className="mr-2 h-5 w-5" />
              Sign in with Apple
            </span>
          </button>
          
          <button
            onClick={handleWalletLogin}
            className="group relative flex w-full justify-center rounded-lg border border-gray-300 bg-gradient-to-r from-blue-400 to-blue-600 py-2 px-4 text-sm font-medium text-white hover:from-blue-500 hover:to-blue-700 focus:outline-none focus:ring-2 focus:ring-cyan-500 focus:ring-offset-2"
          >
            <span className="flex items-center">
              <FaWallet className="mr-2 h-5 w-5" />
              Sign in with Wallet
            </span>
          </button>
        </div>
      </div>
    </div>
  );
} 