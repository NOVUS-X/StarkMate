"use client";

import { useSearchParams } from "next/navigation";
import Link from "next/link";

export default function ErrorPage() {
  const searchParams = useSearchParams();
  const error = searchParams.get("error");

  const errorMessages: { [key: string]: string } = {
    default: "An error occurred during authentication.",
    accessdenied: "Access denied. You may have rejected the authorization request.",
    verification: "The verification process failed.",
    configuration: "There is a problem with the server configuration.",
  };

  const errorMessage = error && errorMessages[error] ? errorMessages[error] : errorMessages.default;

  return (
    <div className="flex min-h-screen flex-col items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div className="w-full max-w-md space-y-8 text-center">
        <h1 className="text-3xl font-bold text-red-500">Authentication Error</h1>
        <div className="mt-2 text-lg">{errorMessage}</div>
        <div className="mt-6">
          <Link 
            href="/auth/signin"
            className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
          >
            Try Again
          </Link>
        </div>
      </div>
    </div>
  );
} 