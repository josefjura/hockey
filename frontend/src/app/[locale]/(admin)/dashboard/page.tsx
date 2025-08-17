"use client"

import { useTranslations } from 'next-intl'
import { BarChart3, Users, Trophy, Calendar, TrendingUp, Activity } from 'lucide-react'
import { QuickCreateTeam, QuickCreatePlayer, QuickCreateEvent } from '@/components/ui/quick-create-forms'

export default function Dashboard() {
  const t = useTranslations('Dashboard')

  // Mock data for demonstration
  const stats = [
    {
      name: t('stats.totalTeams'),
      value: '24',
      change: '+12%',
      changeType: 'increase' as const,
      icon: Users,
    },
    {
      name: t('stats.activePlayers'),
      value: '486',
      change: '+8%',
      changeType: 'increase' as const,
      icon: Activity,
    },
    {
      name: t('stats.ongoingSeasons'),
      value: '3',
      change: '0%',
      changeType: 'neutral' as const,
      icon: Calendar,
    },
    {
      name: t('stats.totalEvents'),
      value: '156',
      change: '+23%',
      changeType: 'increase' as const,
      icon: Trophy,
    },
  ]

  const recentActivity = [
    { id: 1, action: t('activity.teamRegistered'), target: 'HC Sparta Praha', time: '2 hours ago' },
    { id: 2, action: t('activity.playerTransfer'), target: 'Jan Novák → HC Slavia', time: '4 hours ago' },
    { id: 3, action: t('activity.seasonStarted'), target: 'Winter League 2024/25', time: '1 day ago' },
    { id: 4, action: t('activity.eventCreated'), target: 'Championship Finals', time: '2 days ago' },
  ]

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold text-gray-900">{t('title')}</h1>
        <p className="text-gray-600 mt-2">
          {t('subtitle')}
        </p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {stats.map((stat) => (
          <div
            key={stat.name}
            className="bg-white rounded-lg shadow-sm border border-gray-200 p-6"
          >
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <stat.icon className="h-8 w-8 text-blue-500" />
              </div>
              <div className="ml-4 w-0 flex-1">
                <dl>
                  <dt className="text-sm font-medium text-gray-500 truncate">
                    {stat.name}
                  </dt>
                  <dd className="flex items-baseline">
                    <div className="text-2xl font-semibold text-gray-900">
                      {stat.value}
                    </div>
                    <div
                      className={`ml-2 flex items-baseline text-sm font-semibold ${
                        stat.changeType === 'increase'
                          ? 'text-green-600'
                          : stat.changeType === 'neutral'
                          ? 'text-red-600'
                          : 'text-gray-500'
                      }`}
                    >
                      {stat.changeType === 'increase' && (
                        <TrendingUp className="h-4 w-4 mr-1" />
                      )}
                      {stat.change}
                    </div>
                  </dd>
                </dl>
              </div>
            </div>
          </div>
        ))}
      </div>

      {/* Content Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Recent Activity */}
        <div className="bg-white rounded-lg shadow-sm border border-gray-200">
          <div className="px-6 py-4 border-b border-gray-200">
            <h3 className="text-lg font-semibold text-gray-900">
              {t('recentActivity.title')}
            </h3>
          </div>
          <div className="p-6">
            <div className="space-y-4">
              {recentActivity.map((activity) => (
                <div key={activity.id} className="flex items-start space-x-3">
                  <div className="flex-shrink-0">
                    <div className="h-2 w-2 bg-blue-500 rounded-full mt-2"></div>
                  </div>
                  <div className="flex-1 min-w-0">
                    <p className="text-sm text-gray-900">
                      <span className="font-medium">{activity.action}</span>{' '}
                      <span className="text-gray-600">{activity.target}</span>
                    </p>
                    <p className="text-xs text-gray-500 mt-1">{activity.time}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>

        {/* Quick Actions */}
        <div className="bg-white rounded-lg shadow-sm border border-gray-200">
          <div className="px-6 py-4 border-b border-gray-200">
            <h3 className="text-lg font-semibold text-gray-900">
              {t('quickActions.title')}
            </h3>
          </div>
          <div className="p-6">
            <div className="grid grid-cols-1 gap-4">
              <QuickCreateTeam />
              <QuickCreatePlayer />
              <QuickCreateEvent />
              
              <button className="flex items-center p-4 text-left border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors">
                <BarChart3 className="h-6 w-6 text-purple-500 mr-3" />
                <div>
                  <div className="font-medium text-gray-900">{t('quickActions.viewReports')}</div>
                  <div className="text-sm text-gray-500">{t('quickActions.viewReportsDescription')}</div>
                </div>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
