"use client"

import { Link, usePathname } from "@/i18n/navigation"
import { signOut } from "next-auth/react"
import { useTranslations } from 'next-intl'
import { 
  LayoutDashboard, 
  Users, 
  UserCheck,
  Settings, 
  Trophy,
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
  const t = useTranslations('Navigation')
  const [collapsed, setCollapsed] = useState(false)

  const navigation = [
    { 
      name: t('dashboard'), 
      href: '/dashboard', 
      icon: LayoutDashboard,
      description: t('dashboardDescription')
    },
    { 
      name: t('teams'), 
      href: '/teams', 
      icon: Users,
      description: t('teamsDescription')
    },
    { 
      name: t('players'), 
      href: '/players', 
      icon: UserCheck,
      description: t('playersDescription')
    },
    { 
      name: t('events'), 
      href: '/events', 
      icon: Trophy,
      description: t('eventsDescription')
    },
    { 
      name: t('seasons'), 
      href: '/seasons', 
      icon: Calendar,
      description: t('seasonsDescription')
    },
    { 
      name: t('reports'), 
      href: '/reports', 
      icon: FileText,
      description: t('reportsDescription')
    },
    { 
      name: t('management'), 
      href: '/management', 
      icon: Settings,
      description: t('managementDescription')
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
              {t('signedInAs')}
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
          title={collapsed ? t('signOut') : undefined}
        >
          <LogOut className="h-5 w-5 flex-shrink-0" />
          {!collapsed && <span className="ml-3">{t('signOut')}</span>}
        </button>
      </div>
    </div>
  )
}
