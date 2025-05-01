# Social Sign-On (SSO) Integration with Google & Apple

This document outlines the implementation of Social Sign-On authentication using NextAuth.js in StarkMate.

## Features

- Google Authentication
- Apple Authentication
- Wallet-based Authentication
- Unified authentication flow
- TypeScript support
- Integration with Next.js App Router

## Setup Instructions

### 1. Environment Variables

Create a `.env.local` file in the `frontend` directory with the following variables:

```
# NextAuth.js Secret - Generate with `openssl rand -base64 32`
NEXTAUTH_SECRET=your_secret_key_here

# Base URL for NextAuth.js
NEXTAUTH_URL=http://localhost:3000

# Google OAuth
GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret

# Apple OAuth
APPLE_CLIENT_ID=your_apple_client_id
APPLE_ID=your_apple_id
APPLE_TEAM_ID=your_apple_team_id
APPLE_PRIVATE_KEY=your_apple_private_key
APPLE_KEY_ID=your_apple_key_id
```

### 2. Google OAuth Setup

1. Go to the [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Navigate to "APIs & Services" > "Credentials"
4. Click "Create Credentials" > "OAuth client ID"
5. Set up the OAuth consent screen if prompted
6. For Application type, select "Web application"
7. Add authorized JavaScript origins: `http://localhost:3000`
8. Add authorized redirect URIs: `http://localhost:3000/api/auth/callback/google`
9. Click "Create" and note your Client ID and Client Secret

### 3. Apple OAuth Setup

1. Go to the [Apple Developer Portal](https://developer.apple.com/)
2. Navigate to "Certificates, Identifiers & Profiles"
3. Register a new App ID if you don't have one already
4. Enable "Sign In with Apple" for your App ID
5. Create a Services ID for your website
6. Configure the "Sign In with Apple" section for your Services ID
   - Add domains and return URLs (e.g., `localhost` and `http://localhost:3000/api/auth/callback/apple`)
7. Create a private key for "Sign In with Apple"
8. Note down the required credentials:
   - `APPLE_CLIENT_ID`: Your Services ID (e.g., `com.example.web`)
   - `APPLE_ID`: Your Apple Developer Account ID
   - `APPLE_TEAM_ID`: Your Team ID from the developer account
   - `APPLE_KEY_ID`: The ID of the private key you created
   - `APPLE_PRIVATE_KEY`: The contents of the .p8 file you downloaded

## File Structure

```
app/
├── api/
│   └── auth/
│       └── [...nextauth]/
│           └── route.ts       # NextAuth.js API route handler
├── auth/
│   ├── signin/
│   │   └── page.tsx           # Sign-in page
│   ├── signout/
│   │   └── page.tsx           # Sign-out page
│   └── error/
│       └── page.tsx           # Auth error page
├── layout.tsx                 # Root layout with AuthProvider
lib/
├── auth.d.ts                  # TypeScript definitions
└── auth-provider.tsx          # Authentication context provider
hook/
└── use-auth.tsx               # Authentication hook
components/
└── AuthStatus.tsx             # Auth status UI component
```

## Usage

### Authentication Hook

```tsx
import { useAuth } from "@/hook/use-auth";

// In your component
const { 
  isAuthenticated, 
  isLoading, 
  user, 
  loginWithGoogle, 
  loginWithApple, 
  loginWithWallet,
  logout 
} = useAuth();

// Use these functions and states in your UI
```

### Protected Routes

To protect routes, you can create a middleware.ts file in the root of your project:

```tsx
import { NextRequest, NextResponse } from "next/server";
import { getToken } from "next-auth/jwt";

export async function middleware(req: NextRequest) {
  const token = await getToken({ req });
  
  // If the user is not authenticated and trying to access a protected route
  if (!token && req.nextUrl.pathname.startsWith("/protected")) {
    return NextResponse.redirect(new URL("/auth/signin", req.url));
  }
  
  return NextResponse.next();
}

export const config = {
  matcher: ["/protected/:path*"],
};
```

## Production Deployment

When deploying to production:

1. Update the `NEXTAUTH_URL` environment variable to your production domain
2. Add your production domain to the allowed redirect URIs in Google and Apple developer consoles
3. Generate a strong `NEXTAUTH_SECRET` for production
4. Consider using a database adapter for NextAuth.js to persist sessions 