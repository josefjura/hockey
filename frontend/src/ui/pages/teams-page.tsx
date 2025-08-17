"use client"

import { useState, useEffect, Suspense } from 'react'
import { Users, Search, Plus } from 'lucide-react'
import { useTranslations } from 'next-intl'
import { useSuspenseQuery } from '@tanstack/react-query'
import { teamQueries } from '@/queries/teams'
import { useDebounce } from '@/hooks/useDebounce'
import TeamsTable from '@/components/ui/teams-table'
import TeamCreateDialog from '@/components/ui/team-create-dialog'
import TeamEditDialog from '@/components/ui/team-edit-dialog'
import ErrorBoundary from '@/components/error-boundary'
import QueryErrorBoundary from '@/components/query-error-boundary'
import type { Team } from '@/types/team'

function TeamsTableWrapper({ searchTerm, page, pageSize, onPageChange, onEdit }: { 
    searchTerm: string
    page: number
    pageSize: number
    onPageChange: (page: number) => void
    onEdit: (team: Team) => void
}) {
    const { data } = useSuspenseQuery(teamQueries.list(searchTerm, page, pageSize))

    // Runtime validation as failsafe
    if (!data || typeof data !== 'object') {
        throw new Error('Invalid data received from API')
    }
    
    if (!Array.isArray(data.items)) {
        throw new Error('API data.items is not an array')
    }

    return (
        <div className="space-y-4">
            <div className="text-sm text-gray-600">
                Found {data.total} teams
            </div>
            
            <ErrorBoundary
                fallback={
                    <div className="p-4 text-center text-red-600 bg-red-50 rounded-md">
                        Error loading teams table. Please refresh the page.
                    </div>
                }
            >
                <TeamsTable 
                    data={data.items || []}
                    totalItems={data.total || 0}
                    currentPage={data.page || 1}
                    pageSize={data.page_size || pageSize}
                    totalPages={data.total_pages || 1}
                    hasNext={data.has_next || false}
                    hasPrevious={data.has_previous || false}
                    onPageChange={onPageChange}
                    onEdit={onEdit}
                />
            </ErrorBoundary>
        </div>
    )
}

export default function TeamsPage() {
    const t = useTranslations('Teams')
    const [searchTerm, setSearchTerm] = useState('')
    const [page, setPage] = useState(0)
    const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false)
    const [isEditDialogOpen, setIsEditDialogOpen] = useState(false)
    const [selectedTeam, setSelectedTeam] = useState<Team | null>(null)
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

    const handleEdit = (team: Team) => {
        setSelectedTeam(team)
        setIsEditDialogOpen(true)
    }

    const handleCloseEdit = () => {
        setIsEditDialogOpen(false)
        setSelectedTeam(null)
    }

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
                {/* Header with Create Button and Search */}
                <div className="px-6 py-4 border-b border-gray-200 space-y-4">
                    {/* Create Button */}
                    <div className="flex justify-between items-center">
                        <h2 className="text-lg font-semibold text-gray-900">All Teams</h2>
                        <button
                            onClick={() => setIsCreateDialogOpen(true)}
                            className="inline-flex items-center px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
                        >
                            <Plus className="h-4 w-4 mr-2" />
                            Create Team
                        </button>
                    </div>
                    {/* Search */}
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
                    <QueryErrorBoundary
                        fallback={
                            <div className="p-4 text-center text-red-600 bg-red-50 rounded-md">
                                Failed to load teams. This might be due to a backend API issue.
                                <br />
                                <button 
                                    onClick={() => window.location.reload()} 
                                    className="mt-2 px-3 py-1 bg-red-600 text-white rounded text-sm hover:bg-red-700"
                                >
                                    Reload Page
                                </button>
                            </div>
                        }
                    >
                        <Suspense fallback={
                            <TeamsTable 
                                data={[]} 
                                loading={true}
                                totalItems={0}
                                currentPage={1}
                                pageSize={pageSize}
                                totalPages={1}
                                hasNext={false}
                                hasPrevious={false}
                                onPageChange={() => {}}
                                onEdit={() => {}}
                            />
                        }>
                            <TeamsTableWrapper 
                                searchTerm={debouncedSearchTerm}
                                page={page}
                                pageSize={pageSize}
                                onPageChange={handlePageChange}
                                onEdit={handleEdit}
                            />
                        </Suspense>
                    </QueryErrorBoundary>
                </div>
            </div>

            {/* Create Dialog */}
            <TeamCreateDialog 
                isOpen={isCreateDialogOpen} 
                onClose={() => setIsCreateDialogOpen(false)} 
            />

            {/* Edit Dialog */}
            <TeamEditDialog 
                isOpen={isEditDialogOpen} 
                onClose={handleCloseEdit}
                team={selectedTeam}
            />
        </div>
    )
}