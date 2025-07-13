"use client"

import Link from "next/link"
import { usePathname } from "next/navigation"
import { signOut } from "next-auth/react"
import { 
  LayoutDashboard, 
  Users, 
  Settings, 
  Trophy,
  Globe,
  Calendar,
  FileText,
  LogOut,
  ChevronLeft,
  ChevronRight
} from "lucide-react"
import { useState } from "react"
import type { User } from "@/types/auth"

interface AdminSidebarProps {
  user: User
}

export default function AdminSidebar({ user }: AdminSidebarProps) {
  const pathname = usePathname()
  const [collapsed, setCollapsed] = useState(false)

  const navigation = [
    { 
      name: 'Dashboard', 
      href: '/dashboard', 
      icon: LayoutDashboard,
      description: 'Overview and stats'
    },
    { 
      name: 'Teams', 
      href: '/teams', 
      icon: Users,
      description: 'Manage teams'
    },
    { 
      name: 'Players', 
      href: '/players', 
      icon: Users,
      description: 'Player database'
    },
    { 
      name: 'Events', 
      href: '/events', 
      icon: Trophy,
      description: 'Tournaments & leagues'
    },
    { 
      name: 'Seasons', 
      href: '/seasons', 
      icon: Calendar,
      description: 'Season management'
    },
    { 
      name: 'Countries', 
      href: '/countries', 
      icon: Globe,
      description: 'Country settings'
    },
    { 
      name: 'Reports', 
      href: '/reports', 
      icon: FileText,
      description: 'Data analytics'
    },
    { 
      name: 'Management', 
      href: '/management', 
      icon: Settings,
      description: 'System settings'
    },
  ]

  return (
    <div className={`bg-slate-800 text-white transition-all duration-300 ${collapsed ? 'w-16' : 'w-64'} flex flex-col`}>
      {/* Logo and collapse button */}
      <div className="flex items-center justify-between p-4 border-b border-slate-700">
        {!collapsed && (
          <div className="flex items-center space-x-3">
            <div className="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center">
              <span className="text-white text-lg">🏒</span>
            </div>
            <span className="font-bold text-lg">Hockey DB</span>
          </div>
        )}
        <button
          onClick={() => setCollapsed(!collapsed)}
          className="p-1 rounded-md hover:bg-slate-700 transition-colors"
        >
          {collapsed ? (
            <ChevronRight className="h-4 w-4" />
          ) : (
            <ChevronLeft className="h-4 w-4" />
          )}
        </button>
      </div>

      {/* Navigation */}
      <nav className="flex-1 px-3 py-4 space-y-1">
        {navigation.map((item) => {
          const Icon = item.icon
          const isActive = pathname === item.href
          
          return (
            <Link
              key={item.name}
              href={item.href}
              className={`group flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors ${
                isActive
                  ? 'bg-blue-600 text-white'
                  : 'text-slate-300 hover:bg-slate-700 hover:text-white'
              }`}
              title={collapsed ? item.name : undefined}
            >
              <Icon className="h-5 w-5 flex-shrink-0" />
              {!collapsed && (
                <div className="ml-3 flex-1">
                  <div className="text-sm font-medium">{item.name}</div>
                  <div className="text-xs text-slate-400 group-hover:text-slate-300">
                    {item.description}
                  </div>
                </div>
              )}
            </Link>
          )
        })}
      </nav>

      {/* User section and logout */}
      <div className="border-t border-slate-700 p-3">
        {!collapsed && (
          <div className="mb-3 px-3 py-2">
            <div className="text-xs text-slate-400 uppercase tracking-wider font-semibold">
              Signed in as
            </div>
            <div className="text-sm text-white font-medium mt-1 truncate">
              {user?.email}
            </div>
          </div>
        )}
        
        <button
          onClick={() => signOut()}
          className={`group flex items-center w-full px-3 py-2 text-sm font-medium rounded-md text-slate-300 hover:bg-red-600 hover:text-white transition-colors ${
            collapsed ? 'justify-center' : ''
          }`}
          title={collapsed ? 'Sign Out' : undefined}
        >
          <LogOut className="h-5 w-5 flex-shrink-0" />
          {!collapsed && <span className="ml-3">Sign Out</span>}
        </button>
      </div>
    </div>
  )
}
