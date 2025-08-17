"use client"

import { ReactNode } from 'react';
import AdminSidebar from '@/components/layout/admin-sidebar';
import LocaleSwitcher from '@/components/shared/locale-switcher';
import type { User } from "@/types/auth";

interface AdminLayoutClientProps {
  children: ReactNode;
  user: User;
}

export default function AdminLayoutClient({ children, user }: AdminLayoutClientProps) {
  return (
    <div className="flex h-screen bg-gray-100">
      {/* Sidebar */}
      <AdminSidebar user={user} />
      
      {/* Main content area */}
      <div className="flex-1 flex flex-col overflow-hidden">
        {/* Top header */}
        <header className="bg-white shadow-sm border-b border-gray-200">
          <div className="px-6 py-4">
            <div className="flex items-center justify-between">
              <h1 className="text-2xl font-semibold text-gray-900">
                Hockey Database Management
              </h1>
              <div className="flex items-center space-x-4">
                <LocaleSwitcher />
                <span className="text-sm text-gray-500">
                  Welcome, {user?.email}
                </span>
              </div>
            </div>
          </div>
        </header>
        
        {/* Main content */}
        <main className="flex-1 overflow-y-auto p-6">
          {children}
        </main>
      </div>
    </div>
  );
}