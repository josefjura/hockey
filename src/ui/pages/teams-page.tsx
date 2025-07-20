"use client"

import { useState, useEffect, Suspense } from 'react'
import { Users, Search } from 'lucide-react'
import { useTranslations } from 'next-intl'
import type { Team } from '@/types/team'
import { useSuspenseQuery } from '@tanstack/react-query'
import { teamQueries } from '@/queries/teams'
import { useDebounce } from '@/hooks/useDebounce'
import TeamsTable from '@/components/ui/teams-table'

function TeamsTableWrapper({ searchTerm, page }: { 
    searchTerm: string
    page: number 
}) {
    const { data } = useSuspenseQuery(teamQueries.list(searchTerm, page))

    return (
        <div className="space-y-4">
            <div className="text-sm text-gray-600">
                Found {data.total} teams
            </div>
            
            <TeamsTable data={data.items} />
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
                        <TeamsTable data={[]} loading={true} />
                    }>
                        <TeamsTableWrapper 
                            searchTerm={debouncedSearchTerm}
                            page={page}
                        />
                    </Suspense>
                </div>
            </div>
        </div>
    )
}