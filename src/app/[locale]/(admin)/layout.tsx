import { ReactNode } from 'react';
import { auth } from '@/auth';
import { redirect } from 'next/navigation';
import AdminSidebar from '@/components/admin-sidebar';
import LocaleSwitcher from '@/components/locale-switcher';
import { SessionProvider } from 'next-auth/react';

interface LayoutProps {
  children: ReactNode;
  params: Promise<{locale: string}>;
}

export default async function AdminLayout({ children, params }: LayoutProps) {  
  const session = await auth()
  const {locale} = await params;

  if (!session) {
    redirect(`/${locale}/auth/signin`);
  }

  return (
    <SessionProvider session={session}>
      <div className="flex h-screen bg-gray-100">
        {/* Sidebar */}
        <AdminSidebar user={session.user} />
        
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
                    Welcome, {session.user?.email}
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
    </SessionProvider>
  );
}
