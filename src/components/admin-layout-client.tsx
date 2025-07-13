'use client';

import { ReactNode, useState } from 'react';
import { Menu, X, Database, Users, Trophy, Calendar, BarChart3, Home, Type, Wrench } from 'lucide-react';
import { signOut } from 'next-auth/react';
import { usePathname } from 'next/navigation';

interface AdminLayoutClientProps  {
  children: ReactNode;
	userName: string;
}

// Reusable Layout Component
function AdminLayoutClient({ children,  userName }: AdminLayoutClientProps ) {
  const [sidebarOpen, setSidebarOpen] = useState(false);
  const currentPage = usePathname().split('/')[2] || 'dashboard'; // Get the current page from the URL
  const navigation = [
    { name: 'Dashboard', href: '/dashboard', icon: Home, current: currentPage === 'dashboard' },
    { name: 'Teams', href: '/teams', icon: Users, current: currentPage === 'teams' },
    { name: 'Players', href: '/players', icon: Database, current: currentPage === 'players' },
    { name: 'Games', href: '/games', icon: Calendar, current: currentPage === 'games' },
    { name: 'Events', href: '/events', icon: Type, current: currentPage === 'events' },
    { name: 'Standings', href: '/standings', icon: Trophy, current: currentPage === 'standings' },
    { name: 'Statistics', href: '/stats', icon: BarChart3, current: currentPage === 'stats' },
    { name: 'Management', href: '/management', icon: Wrench, current: currentPage === 'management' },
  ];

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Mobile sidebar overlay */}
      {sidebarOpen && (
        <div className="fixed inset-0 z-40 lg:hidden">
          <div className="fixed inset-0 bg-gray-600 bg-opacity-75" onClick={() => setSidebarOpen(false)} />
        </div>
      )}

      {/* Sidebar */}
      <div className={`fixed inset-y-0 left-0 z-50 w-64 bg-slate-900 transform transition-transform duration-300 ease-in-out ${sidebarOpen ? 'translate-x-0' : '-translate-x-full'} lg:translate-x-0`}>
        <div className="flex items-center justify-between h-16 px-4 bg-slate-800">
          <div className="flex items-center space-x-2">
            <div className="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center">
              <span className="text-white text-lg">🏒</span>
            </div>
            <span className="text-white font-semibold text-lg">Hockey DB {currentPage}</span>
          </div>
          <button
            onClick={() => setSidebarOpen(false)}
            className="lg:hidden text-gray-400 hover:text-white"
          >
            <X className="w-6 h-6" />
          </button>
        </div>
        
        <nav className="mt-8">
          <div className="px-2 space-y-1">
            {navigation.map((item) => (
              <a
                key={item.name}
                href={item.href}
                className={`${
                  item.current
                    ? 'bg-slate-800 text-white border-r-2 border-blue-500'
                    : 'text-gray-300 hover:bg-slate-700 hover:text-white'
                } group flex items-center px-3 py-2 text-sm font-medium rounded-l-md transition-colors`}
              >
                <item.icon className="mr-3 h-5 w-5" />
                {item.name}
              </a>
            ))}
          </div>
        </nav>
      </div>

      {/* Main content */}
      <div className="lg:pl-64">
        {/* Top bar */}
        <div className="sticky top-0 z-10 flex h-16 bg-white shadow-sm">
          <button
            onClick={() => setSidebarOpen(true)}
            className="px-4 text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-blue-500 lg:hidden"
          >
            <Menu className="h-6 w-6" />
          </button>
          <div className="flex flex-1 justify-between px-4 lg:px-6">
            <div className="flex flex-1 items-center">
              <h1 className="text-xl font-semibold text-gray-900">Hockey League Manager</h1>
            </div>
            <div className="flex items-center space-x-4">
              <span className="text-sm text-gray-500">Welcome, {userName}</span>
							<button
								onClick={() => signOut()}
								className="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg font-medium transition-colors text-sm flex items-center space-x-2"
							>
								Sign Out
							</button>
            </div>
          </div>
        </div>

        {/* Page content */}
        <main className="flex-1">
          {children}
        </main>
      </div>
    </div>
  );
}

export default AdminLayoutClient;