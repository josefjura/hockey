"use client"

import { useState, useEffect, Suspense } from 'react'
import { Gamepad2, Search, Plus } from 'lucide-react'
import { useTranslations } from 'next-intl'
import { useSuspenseQuery, useQuery } from '@tanstack/react-query'
import { useRouter } from '@/i18n/navigation'
import { matchQueries } from '@/queries/matches'
import { teamQueries } from '@/queries/teams'
import { seasonQueries } from '@/queries/seasons'
import { useDebounce } from '@/hooks/useDebounce'
import MatchesTable from '@/components/ui/matches-table'
import MatchCreateDialog from '@/components/ui/match-create-dialog'
import MatchEditDialog from '@/components/ui/match-edit-dialog'
import ErrorBoundary from '@/components/error-boundary'
import QueryErrorBoundary from '@/components/query-error-boundary'
import type { Match } from '@/types/match'

function MatchesTableWrapper({ 
    filters, 
    page, 
    pageSize, 
    onPageChange, 
    onEdit,
    onViewDetails
}: { 
    filters: {
        searchTerm?: string;
        seasonId?: string;
        teamId?: string;
        status?: string;
        dateFrom?: string;
        dateTo?: string;
    }
    page: number
    pageSize: number
    onPageChange: (page: number) => void
    onEdit: (match: Match) => void
    onViewDetails: (match: Match) => void
}) {
    const { data } = useSuspenseQuery(matchQueries.list(filters, page, pageSize))

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
                Found {data.total} matches
            </div>
            
            <ErrorBoundary
                fallback={
                    <div className="p-4 text-center text-red-600 bg-red-50 rounded-md">
                        Error loading matches table. Please refresh the page.
                    </div>
                }
            >
                <MatchesTable 
                    data={data.items || []}
                    totalItems={data.total || 0}
                    currentPage={data.page || 1}
                    pageSize={data.page_size || pageSize}
                    totalPages={data.total_pages || 1}
                    hasNext={data.has_next || false}
                    hasPrevious={data.has_previous || false}
                    onPageChange={onPageChange}
                    onEdit={onEdit}
                    onViewDetails={onViewDetails}
                />
            </ErrorBoundary>
        </div>
    )
}

export default function MatchesPage() {
    const t = useTranslations('Matches')
    const router = useRouter()
    const [searchTerm, setSearchTerm] = useState('')
    const [seasonFilter, setSeasonFilter] = useState('')
    const [teamFilter, setTeamFilter] = useState('')
    const [statusFilter, setStatusFilter] = useState('')
    const [page, setPage] = useState(0)
    const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false)
    const [isEditDialogOpen, setIsEditDialogOpen] = useState(false)
    const [selectedMatch, setSelectedMatch] = useState<Match | null>(null)
    const debouncedSearchTerm = useDebounce(searchTerm, 300)
    const pageSize = 20 // Consistent page size

    // Load filter data
    const { data: teamsData } = useQuery(teamQueries.all())
    const { data: seasonsData } = useQuery(seasonQueries.all())

    // Build filters object
    const filters = {
        searchTerm: debouncedSearchTerm || undefined,
        seasonId: seasonFilter || undefined,
        teamId: teamFilter || undefined,
        status: statusFilter || undefined,
    }

    // Reset page when filters change
    useEffect(() => {
        setPage(0)
    }, [debouncedSearchTerm, seasonFilter, teamFilter, statusFilter])

    // Handle page changes (convert from 1-based backend to 0-based frontend)
    const handlePageChange = (newPage: number) => {
        setPage(newPage - 1) // Convert 1-based to 0-based
    }

    const handleEdit = (match: Match) => {
        setSelectedMatch(match)
        setIsEditDialogOpen(true)
    }

    const handleViewDetails = (match: Match) => {
        router.push(`/matches/${match.id}`)
    }

    const handleCloseEdit = () => {
        setIsEditDialogOpen(false)
        setSelectedMatch(null)
    }

    return (
        <div className="space-y-6">
            {/* Header */}
            <div>
                <div className="flex items-center space-x-3 mb-2">
                    <Gamepad2 className="h-8 w-8 text-blue-500" />
                    <h1 className="text-3xl font-bold text-gray-900">{t('title')}</h1>
                </div>
                <p className="text-gray-600">{t('description')}</p>
            </div>

            {/* Matches Section */}
            <div className="bg-white rounded-lg shadow-sm border border-gray-200">
                {/* Header with Create Button and Filters */}
                <div className="px-6 py-4 border-b border-gray-200 space-y-4">
                    {/* Create Button */}
                    <div className="flex justify-between items-center">
                        <h2 className="text-lg font-semibold text-gray-900">All Matches</h2>
                        <button
                            onClick={() => setIsCreateDialogOpen(true)}
                            className="inline-flex items-center px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
                        >
                            <Plus className="h-4 w-4 mr-2" />
                            Create Match
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

                    {/* Filters */}
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                        {/* Season Filter */}
                        <div>
                            <label className="block text-xs font-medium text-gray-700 mb-1">
                                Season
                            </label>
                            <select
                                value={seasonFilter}
                                onChange={(e) => setSeasonFilter(e.target.value)}
                                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-sm text-gray-900"
                            >
                                <option value="">{t('filters.allSeasons')}</option>
                                {seasonsData?.map((season) => (
                                    <option key={season.id} value={season.id}>
                                        {season.name || season.year} ({season.event_name})
                                    </option>
                                ))}
                            </select>
                        </div>

                        {/* Team Filter */}
                        <div>
                            <label className="block text-xs font-medium text-gray-700 mb-1">
                                Team
                            </label>
                            <select
                                value={teamFilter}
                                onChange={(e) => setTeamFilter(e.target.value)}
                                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-sm text-gray-900"
                            >
                                <option value="">{t('filters.allTeams')}</option>
                                {teamsData?.map((team) => (
                                    <option key={team.id} value={team.id}>
                                        {team.name || 'Team'}
                                    </option>
                                ))}
                            </select>
                        </div>

                        {/* Status Filter */}
                        <div>
                            <label className="block text-xs font-medium text-gray-700 mb-1">
                                Status
                            </label>
                            <select
                                value={statusFilter}
                                onChange={(e) => setStatusFilter(e.target.value)}
                                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-sm text-gray-900"
                            >
                                <option value="">{t('filters.allStatuses')}</option>
                                <option value="scheduled">{t('filters.status.scheduled')}</option>
                                <option value="in_progress">{t('filters.status.in_progress')}</option>
                                <option value="finished">{t('filters.status.finished')}</option>
                                <option value="cancelled">{t('filters.status.cancelled')}</option>
                            </select>
                        </div>
                    </div>
                </div>

                <div className="p-6">
                    <QueryErrorBoundary
                        fallback={
                            <div className="p-4 text-center text-red-600 bg-red-50 rounded-md">
                                Failed to load matches. This might be due to a backend API issue.
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
                            <MatchesTable 
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
                                onViewDetails={() => {}}
                            />
                        }>
                            <MatchesTableWrapper 
                                filters={filters}
                                page={page}
                                pageSize={pageSize}
                                onPageChange={handlePageChange}
                                onEdit={handleEdit}
                                onViewDetails={handleViewDetails}
                            />
                        </Suspense>
                    </QueryErrorBoundary>
                </div>
            </div>

            {/* Create Dialog */}
            <MatchCreateDialog 
                isOpen={isCreateDialogOpen} 
                onClose={() => setIsCreateDialogOpen(false)} 
            />

            {/* Edit Dialog */}
            <MatchEditDialog 
                isOpen={isEditDialogOpen} 
                onClose={handleCloseEdit}
                match={selectedMatch}
            />
        </div>
    )
}