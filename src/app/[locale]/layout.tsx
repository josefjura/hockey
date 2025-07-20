// app/layout.tsx - Root Layout
import AuthProvider from "@/components/shared/auth-provider";
import { routing } from "@/i18n/routing";
import { QueryClientProvider } from "@/providers";
import type { Metadata } from "next";
import { hasLocale, NextIntlClientProvider } from "next-intl";
import { getMessages } from "next-intl/server";
import { Geist, Geist_Mono } from "next/font/google";
import { notFound } from "next/navigation";
import "../globals.css";

export const metadata: Metadata = {
  title: "Hockey Database - League Management System",
  description: "Complete hockey league management with team tracking, player statistics, and game analytics",
};

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});



export default async function RootLayout({
  children,
  params
}: {
  children: React.ReactNode;
  params: Promise<{locale: string}>;
}) {

  // Ensure that the incoming `locale` is valid
  const {locale} = await params;
  if (!hasLocale(routing.locales, locale)) {
    notFound();
  }
  
  // Providing all messages to the client
  // side is the easiest way to get started
  const messages = await getMessages();
return <html lang={locale}>
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
        suppressHydrationWarning={true}
      >
				<QueryClientProvider>
        <NextIntlClientProvider messages={messages}>
          <AuthProvider>						
            {children}						
          </AuthProvider>
        </NextIntlClientProvider>
				</QueryClientProvider>				
      </body>
    </html>
}