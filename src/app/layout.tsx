// app/layout.tsx - Root Layout
import type { Metadata } from "next";
import { ReactNode } from "react";

export const metadata: Metadata = {
  title: "Hockey Database - League Management System",
  description: "Complete hockey league management with team tracking, player statistics, and game analytics",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: ReactNode;
}>) {
  return children;
}