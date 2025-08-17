"use client"

import { ReactNode } from 'react';
import { useSession } from 'next-auth/react';
import { useRouter } from '@/i18n/navigation';
import { useEffect } from 'react';
import AdminLayoutClient from '@/components/layout/admin-layout-client';

interface LayoutProps {
  children: ReactNode;
}

export default function AdminLayout({ children }: LayoutProps) {  
  const { data: session, status } = useSession()
  const router = useRouter()

  useEffect(() => {
    if (status === 'unauthenticated') {
      router.push('/auth/signin')
    }
  }, [status, router])

  // Show loading while checking auth
  if (status === 'loading') {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-100">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
      </div>
    )
  }

  // Middleware should handle this, but double-check
  if (!session) {
    return null
  }

  return (
    <AdminLayoutClient user={session.user}>
      {children}
    </AdminLayoutClient>
  );
}
