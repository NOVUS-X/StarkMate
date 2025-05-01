import Link from 'next/link';

export default function Home() {
  return (
    <div className="container py-8">
      <div className="md:flex items-center justify-between mb-8">
        <div>
          <h1 className="text-4xl font-bold mb-2 bg-clip-text text-transparent bg-gradient-to-r from-blue-400 to-blue-600">
            Play Chess. Wager On Starknet
          </h1>
        </div>
      </div>
      
      <div className="mt-8 p-6 border border-gray-300 rounded-lg bg-gray-50 dark:bg-gray-800 dark:border-gray-700">
        <h2 className="text-2xl font-bold mb-4">Authentication Test Links</h2>
        <div className="space-y-4">
          <div>
            <Link 
              href="/auth/signin"
              className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 mr-4"
            >
              Sign In Page
            </Link>
            <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
              Test the sign-in page with Google, Apple, and Wallet options
            </p>
          </div>
          
          <div>
            <Link 
              href="/protected"
              className="inline-flex items-center rounded-md bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700 mr-4"
            >
              Protected Page
            </Link>
            <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
              Access a protected page (should redirect to sign-in if not authenticated)
            </p>
          </div>
          
          <div>
            <Link 
              href="/auth/signout"
              className="inline-flex items-center rounded-md bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700 mr-4"
            >
              Sign Out
            </Link>
            <p className="mt-1 text-sm text-gray-600 dark:text-gray-400">
              Test the sign-out functionality
            </p>
          </div>
        </div>
        
        <div className="mt-6 p-4 bg-gray-100 dark:bg-gray-700 rounded-md">
          <h3 className="font-semibold mb-2">Check Authentication in Console</h3>
          <p className="text-sm text-gray-600 dark:text-gray-400 mb-2">
            Open your browser console and run this code:
          </p>
          <pre className="bg-gray-800 text-gray-100 p-3 rounded text-sm overflow-x-auto">
            {`fetch('/api/auth/test').then(res => res.json()).then(data => console.log(data));`}
          </pre>
        </div>
      </div>
    </div>
  );
}
