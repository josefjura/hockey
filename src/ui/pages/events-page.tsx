"use client"

import { useState, useEffect, Suspense } from 'react'
import { Calendar, Search } from 'lucide-react'
import { useTranslations } from 'next-intl'
import { useSuspenseQuery } from '@tanstack/react-query'
import { eventQueries } from '@/queries/teams'
import { useDebounce } from '@/hooks/useDebounce'
import EventsTable from '@/components/ui/events-table'

function EventsTableWrapper({ searchTerm, page, pageSize, onPageChange }: { 
    searchTerm: string
    page: number
    pageSize: number
    onPageChange: (page: number) => void
}) {
    const { data } = useSuspenseQuery(eventQueries.list(searchTerm, page, pageSize))

    return (
        <div className="space-y-4">
            <div className="text-sm text-gray-600">
                Found {data.total} events
            </div>
            
            <EventsTable 
                data={data.items}
                totalItems={data.total}
                currentPage={data.page}
                pageSize={data.page_size}
                totalPages={data.total_pages}
                hasNext={data.has_next}
                hasPrevious={data.has_previous}
                onPageChange={onPageChange}
            />
        </div>
    )
}

export default function EventsPage() {
    const t = useTranslations('Events')
    const [searchTerm, setSearchTerm] = useState('')
    const [page, setPage] = useState(0)
    const debouncedSearchTerm = useDebounce(searchTerm, 300)
    const pageSize = 20 // Consistent page size

    // Reset page when search term changes
    useEffect(() => {
        setPage(0)
    }, [debouncedSearchTerm])

    // Handle page changes (convert from 1-based backend to 0-based frontend)
    const handlePageChange = (newPage: number) => {
        setPage(newPage - 1) // Convert 1-based to 0-based
    }

    return (
        <div className="space-y-6">
            {/* Header */}
            <div>
                <div className="flex items-center space-x-3 mb-2">
                    <Calendar className="h-8 w-8 text-blue-500" />
                    <h1 className="text-3xl font-bold text-gray-900">{t('title')}</h1>
                </div>
                <p className="text-gray-600">{t('description')}</p>
            </div>

            {/* Events Section */}
            <div className="bg-white rounded-lg shadow-sm border border-gray-200">
                {/* Search */}
                <div className="px-6 py-4 border-b border-gray-200">
                    <div className="relative">
                        <Search className="h-5 w-5 absolute left-3 top-3 text-gray-400" />
                        <input
                            type="text"
                            placeholder={t('searchPlaceholder')}
                            className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 placeholder-gray-400"
                            value={searchTerm}
                            onChange={(e) => setSearchTerm(e.target.value)}
                        />
                    </div>
                </div>

                <div className="p-6">
                    <Suspense fallback={
                        <EventsTable 
                            data={[]} 
                            loading={true}
                            totalItems={0}
                            currentPage={1}
                            pageSize={pageSize}
                            totalPages={1}
                            hasNext={false}
                            hasPrevious={false}
                            onPageChange={() => {}}
                        />
                    }>
                        <EventsTableWrapper 
                            searchTerm={debouncedSearchTerm}
                            page={page}
                            pageSize={pageSize}
                            onPageChange={handlePageChange}
                        />
                    </Suspense>
                </div>
            </div>
        </div>
    )
}