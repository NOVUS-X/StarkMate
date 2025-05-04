# Environment Variables Setup for SSO Integration

## Required Environment Variables

Add the following variables to your `.env.local` file:

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

## Setting Up Google OAuth

1. Go to the [Google Cloud Console](https://console.cloud.google.com/)
2. Create a new project or select an existing one
3. Navigate to "APIs & Services" > "Credentials"
4. Click "Create Credentials" > "OAuth client ID"
5. Set up the OAuth consent screen if prompted
6. For Application type, select "Web application"
7. Add authorized JavaScript origins: `http://localhost:3000`
8. Add authorized redirect URIs: `http://localhost:3000/api/auth/callback/google`
9. Click "Create" and note your Client ID and Client Secret

## Setting Up Apple OAuth

1. Go to the [Apple Developer Portal](https://developer.apple.com/)
2. Navigate to "Certificates, Identifiers & Profiles"
3. Register a new App ID if you don't have one already
4. Enable "Sign In with Apple" for your App ID
5. Create a Services ID for your website
6. Configure the "Sign In with Apple" section for your Services ID
   - Add domains and return URLs (e.g., `localhost` and `http://localhost:3000/api/auth/callback/apple`)
7. Create a private key for "Sign In with Apple"
8. Note down:
   - `APPLE_CLIENT_ID`: Your Services ID (e.g., `com.example.web`)
   - `APPLE_ID`: Your Apple Developer Account ID
   - `APPLE_TEAM_ID`: Your Team ID from the developer account
   - `APPLE_KEY_ID`: The ID of the private key you created
   - `APPLE_PRIVATE_KEY`: The contents of the .p8 file you downloaded (multiline, handle with care in .env file) 