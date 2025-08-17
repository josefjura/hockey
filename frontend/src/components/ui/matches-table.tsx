"use client"

import { Edit, Trash2, Trophy, Calendar, MapPin, Users, Gamepad2, Eye } from 'lucide-react'
import { useDeleteMatch } from '@/queries/matches'
import TableSkeleton from './table-skeleton'
import Badge from './badge'
import Pager from './pager'
import type { Match } from '@/types/match'

interface MatchesTableProps {
    data: Match[]
    loading?: boolean
    totalItems?: number
    currentPage: number
    pageSize?: number
    totalPages: number
    hasNext: boolean
    hasPrevious: boolean
    onPageChange: (page: number) => void
    onEdit: (match: Match) => void
    onViewDetails: (match: Match) => void
}

export default function MatchesTable({
    data,
    loading = false,
    currentPage,
    totalPages,
    onPageChange,
    onEdit,
    onViewDetails,
}: MatchesTableProps) {
    const deleteMatchMutation = useDeleteMatch()

    const handleDelete = async (id: string) => {
        if (confirm('Are you sure you want to delete this match?')) {
            deleteMatchMutation.mutate(id)
        }
    }

    const formatDate = (dateString: string | null) => {
        if (!dateString) return '-'
        
        try {
            const date = new Date(dateString)
            return new Intl.DateTimeFormat('en-US', {
                year: 'numeric',
                month: 'short',
                day: 'numeric',
                hour: '2-digit',
                minute: '2-digit'
            }).format(date)
        } catch {
            return dateString
        }
    }

    const getStatusBadgeVariant = (status: string) => {
        switch (status.toLowerCase()) {
            case 'scheduled':
                return 'info'
            case 'in_progress':
                return 'warning'
            case 'finished':
                return 'success'
            case 'cancelled':
                return 'error'
            default:
                return 'default'
        }
    }

    const getStatusDisplay = (status: string) => {
        switch (status.toLowerCase()) {
            case 'scheduled':
                return 'Scheduled'
            case 'in_progress':
                return 'In Progress'
            case 'finished':
                return 'Finished'
            case 'cancelled':
                return 'Cancelled'
            default:
                return status
        }
    }

    if (loading) {
        return <TableSkeleton rows={5} columns={6} />
    }

    if (data.length === 0) {
        return (
            <div className="text-center py-12">
                <Gamepad2 className="mx-auto h-12 w-12 text-gray-400" />
                <h3 className="mt-2 text-sm font-medium text-gray-900">No matches found</h3>
                <p className="mt-1 text-sm text-gray-500">
                    Try adjusting your search or filter criteria.
                </p>
            </div>
        )
    }

    return (
        <div className="space-y-4">
            <div className="overflow-hidden shadow ring-1 ring-black ring-opacity-5 rounded-lg">
                <table className="min-w-full divide-y divide-gray-300">
                    <thead className="bg-gray-50">
                        <tr>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Match
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Score
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Season
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Date
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Status
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Venue
                            </th>
                            <th className="relative px-6 py-3">
                                <span className="sr-only">Actions</span>
                            </th>
                        </tr>
                    </thead>
                    <tbody className="bg-white divide-y divide-gray-200">
                        {data.map((match) => (
                            <tr key={match.id} className="hover:bg-gray-50">
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <div className="flex items-center space-x-3">
                                        <div className="flex-shrink-0">
                                            <div className="h-8 w-8 bg-blue-100 rounded-full flex items-center justify-center">
                                                <Users className="h-4 w-4 text-blue-600" />
                                            </div>
                                        </div>
                                        <div>
                                            <div className="text-sm font-medium text-gray-900">
                                                {match.home_team_name} vs {match.away_team_name}
                                            </div>
                                            <div className="text-sm text-gray-500">
                                                ID: {match.id}
                                            </div>
                                        </div>
                                    </div>
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <div className="text-sm font-medium text-gray-900">
                                        {match.home_score_total} - {match.away_score_total}
                                    </div>
                                    {(match.home_score_unidentified > 0 || match.away_score_unidentified > 0) && (
                                        <div className="text-xs text-gray-500">
                                            ({match.home_score_unidentified} - {match.away_score_unidentified} unidentified)
                                        </div>
                                    )}
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <div className="flex items-center space-x-1">
                                        <Trophy className="h-4 w-4 text-gray-400" />
                                        <span className="text-sm text-gray-900">{match.season_name}</span>
                                    </div>
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <div className="flex items-center space-x-1">
                                        <Calendar className="h-4 w-4 text-gray-400" />
                                        <span className="text-sm text-gray-900">
                                            {formatDate(match.match_date)}
                                        </span>
                                    </div>
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <Badge variant={getStatusBadgeVariant(match.status)}>
                                        {getStatusDisplay(match.status)}
                                    </Badge>
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    {match.venue ? (
                                        <div className="flex items-center space-x-1">
                                            <MapPin className="h-4 w-4 text-gray-400" />
                                            <span className="text-sm text-gray-900">{match.venue}</span>
                                        </div>
                                    ) : (
                                        <span className="text-sm text-gray-500">-</span>
                                    )}
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                                    <div className="flex items-center justify-end space-x-2">
                                        <button
                                            onClick={() => onViewDetails(match)}
                                            className="text-green-600 hover:text-green-900 p-1 rounded-md hover:bg-green-50 transition-colors"
                                            title="View match details"
                                        >
                                            <Eye className="h-4 w-4" />
                                        </button>
                                        <button
                                            onClick={() => onEdit(match)}
                                            className="text-blue-600 hover:text-blue-900 p-1 rounded-md hover:bg-blue-50 transition-colors"
                                            title="Edit match"
                                        >
                                            <Edit className="h-4 w-4" />
                                        </button>
                                        <button
                                            onClick={() => handleDelete(match.id)}
                                            disabled={deleteMatchMutation.isPending}
                                            className="text-red-600 hover:text-red-900 p-1 rounded-md hover:bg-red-50 transition-colors disabled:opacity-50"
                                            title="Delete match"
                                        >
                                            <Trash2 className="h-4 w-4" />
                                        </button>
                                    </div>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>

            <Pager
                currentPage={currentPage}
                totalPages={totalPages}
                onPageChange={onPageChange}
            />
        </div>
    )
}