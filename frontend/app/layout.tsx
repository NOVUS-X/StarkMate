import type { Metadata } from "next";
import "./globals.css";
import ClientRoot from "@/components/ClientRoot";
import { AuthProvider } from "@/lib/auth-provider";

export const metadata: Metadata = {
  title: "Starkmate",
  description: "Starkmate — Chess on Starknet",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="antialiased font-rowdies bg-background text-foreground">
        <AuthProvider>
          <ClientRoot>{children}</ClientRoot>
        </AuthProvider>
      </body>
    </html>
  );
}
