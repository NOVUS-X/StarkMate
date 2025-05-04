"use client";

import { useEffect } from "react";
import { useAuth } from "@/hook/use-auth";
import { useRouter } from "next/navigation";

export default function SignOut() {
  const { logout, isAuthenticated, isLoading } = useAuth();
  const router = useRouter();

  useEffect(() => {
    if (!isLoading) {
      if (isAuthenticated) {
        logout();
      } else {
        router.push("/");
      }
    }
  }, [isLoading, isAuthenticated, logout, router]);

  return (
    <div className="flex min-h-screen flex-col items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8 text-center">
        <h1 className="text-3xl font-bold">Signing you out...</h1>
        <div className="mt-4">
          <div className="animate-pulse flex space-x-4 justify-center">
            <div className="w-12 h-12 bg-blue-400 rounded-full"></div>
          </div>
        </div>
      </div>
    </div>
  );
} 