"use client";

import { useAuth } from "@/hook/use-auth";
import Link from "next/link";

export default function ProtectedPage() {
  const { user, isAuthenticated, isLoading } = useAuth();

  if (isLoading) {
    return (
      <div className="flex min-h-screen flex-col items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
        <div className="w-full max-w-md space-y-8 text-center">
          <h1 className="text-3xl font-bold">Loading...</h1>
          <div className="mt-4">
            <div className="animate-pulse flex space-x-4 justify-center">
              <div className="w-12 h-12 bg-blue-400 rounded-full"></div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex min-h-screen flex-col items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8 text-center">
        <h1 className="text-3xl font-bold">Protected Page</h1>
        
        {isAuthenticated ? (
          <div className="mt-6">
            <div className="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded relative mb-6">
              <strong className="font-bold">Authentication Successful!</strong>
              <p className="mt-2">
                You are signed in as:
                <br />
                {user?.name || user?.email || user?.address || "Unknown user"}
              </p>
            </div>
            
            <div className="mt-4">
              <Link 
                href="/"
                className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
              >
                Back to Home
              </Link>
            </div>
          </div>
        ) : (
          <div className="mt-6">
            <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-6">
              <strong className="font-bold">Not Authenticated</strong>
              <p className="mt-2">
                You should have been redirected to the sign-in page.
                If you see this message, it means the middleware isn't working correctly.
              </p>
            </div>
            
            <div className="mt-4">
              <Link 
                href="/auth/signin"
                className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 mr-4"
              >
                Sign In
              </Link>
              
              <Link 
                href="/"
                className="inline-flex items-center rounded-md bg-gray-600 px-4 py-2 text-sm font-medium text-white hover:bg-gray-700"
              >
                Home
              </Link>
            </div>
          </div>
        )}
      </div>
    </div>
  );
} 