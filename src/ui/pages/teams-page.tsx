"use client"

import { useState, useEffect, Suspense } from 'react'
import { Users, Search } from 'lucide-react'
import { useTranslations } from 'next-intl'
import type { Team } from '@/types/team'
import { useSuspenseQuery } from '@tanstack/react-query'
import { teamQueries } from '@/queries/teams'
import { useDebounce } from '@/hooks/useDebounce'

function TeamsList({ searchTerm, page }: { 
    searchTerm: string
    page: number 
}) {
    const t = useTranslations('Teams')
    
    const { data } = useSuspenseQuery(teamQueries.list(searchTerm, page))

    return (
        <div className="space-y-4">
            <div className="text-sm text-gray-600">
                Found {data.total} teams
            </div>
            
            <div className="grid gap-4">
                {data.items.map((team: Team) => (
                    <div key={team.id} className="p-4 border border-gray-200 rounded-lg hover:bg-gray-50">
                        <div className="flex items-center space-x-3">
                            <Users className="h-5 w-5 text-blue-500" />
                            <div>
                                <h3 className="font-medium text-gray-900">
                                    {team.name || 'Unnamed Team'}
                                </h3>
                                <p className="text-sm text-gray-500">
                                    Country ID: {team.country_id}
                                </p>
                            </div>
                        </div>
                    </div>
                ))}
                
                {data.items.length === 0 && (
                    <div className="text-center py-8 text-gray-500">
                        No teams found
                    </div>
                )}
            </div>
        </div>
    )
}

export default function TeamsPage() {
    const t = useTranslations('Teams')
    const [searchTerm, setSearchTerm] = useState('')
    const [page, setPage] = useState(0)
    const debouncedSearchTerm = useDebounce(searchTerm, 300)

    // Reset page when search term changes
    useEffect(() => {
        setPage(0)
    }, [debouncedSearchTerm])

    return (
        <div className="space-y-6">
            {/* Header */}
            <div>
                <div className="flex items-center space-x-3 mb-2">
                    <Users className="h-8 w-8 text-blue-500" />
                    <h1 className="text-3xl font-bold text-gray-900">{t('title')}</h1>
                </div>
                <p className="text-gray-600">{t('description')}</p>
            </div>

            {/* Teams Section */}
            <div className="bg-white rounded-lg shadow-sm border border-gray-200">
                {/* Search */}
                <div className="px-6 py-4 border-b border-gray-200">
                    <div className="relative">
                        <Search className="h-5 w-5 absolute left-3 top-3 text-gray-400" />
                        <input
                            type="text"
                            placeholder={t('searchPlaceholder')}
                            className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                            value={searchTerm}
                            onChange={(e) => setSearchTerm(e.target.value)}
                        />
                    </div>
                </div>

                <div className="p-6">
                    <Suspense fallback={
                        <div className="space-y-4">
                            {Array.from({ length: 5 }).map((_, i) => (
                                <div key={i} className="p-4 border border-gray-200 rounded-lg animate-pulse">
                                    <div className="flex items-center space-x-3">
                                        <div className="h-5 w-5 bg-gray-200 rounded"></div>
                                        <div className="space-y-2">
                                            <div className="h-4 w-32 bg-gray-200 rounded"></div>
                                            <div className="h-3 w-24 bg-gray-200 rounded"></div>
                                        </div>
                                    </div>
                                </div>
                            ))}
                        </div>
                    }>
                        <TeamsList 
                            searchTerm={debouncedSearchTerm}
                            page={page}
                        />
                    </Suspense>
                </div>
            </div>
        </div>
    )
}