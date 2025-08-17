"use client"

import { useState, Suspense } from 'react'
import { ArrowLeft, Plus, Target, Edit, Trash2, Clock, MapPin, Trophy } from 'lucide-react'
import { useRouter } from '@/i18n/navigation'
import { useSuspenseQuery } from '@tanstack/react-query'
import { matchQueries } from '@/queries/matches'
import { useDeleteScoreEvent } from '@/queries/matches'
import Badge from '@/components/ui/badge'
import AddGoalDialog from '@/components/ui/add-goal-dialog'
import IdentifyGoalDialog from '@/components/ui/identify-goal-dialog'
import MatchEditDialog from '@/components/ui/match-edit-dialog'
import ErrorBoundary from '@/components/error-boundary'
import QueryErrorBoundary from '@/components/query-error-boundary'
import type { Match, MatchWithStats } from '@/types/match'

interface MatchDetailsPageProps {
    matchId: string
}

function MatchScoreboard({ match, stats }: { match: Match; stats: MatchWithStats }) {
    const getStatusDisplay = (status: string) => {
        switch (status.toLowerCase()) {
            case 'scheduled':
                return 'SCHEDULED'
            case 'in_progress':
                return 'LIVE'
            case 'finished':
                return 'FINAL'
            case 'cancelled':
                return 'CANCELLED'
            default:
                return status.toUpperCase()
        }
    }

    const getStatusColor = (status: string) => {
        switch (status.toLowerCase()) {
            case 'scheduled':
                return 'blue'
            case 'in_progress':
                return 'yellow'
            case 'finished':
                return 'green'
            case 'cancelled':
                return 'red'
            default:
                return 'gray'
        }
    }

    const formatDate = (dateString: string | null) => {
        if (!dateString) return null
        
        try {
            const date = new Date(dateString)
            return new Intl.DateTimeFormat('en-US', {
                month: 'short',
                day: 'numeric',
                hour: '2-digit',
                minute: '2-digit'
            }).format(date)
        } catch {
            return dateString
        }
    }

    return (
        <div className="bg-gradient-to-r from-blue-600 to-blue-800 text-white rounded-lg p-6 shadow-lg">
            {/* Top info */}
            <div className="flex justify-between items-center mb-4">
                <div className="flex items-center space-x-2 text-blue-100">
                    <Trophy className="h-4 w-4" />
                    <span className="text-sm">{match.season_name}</span>
                </div>
                <div className="flex items-center space-x-4 text-blue-100 text-sm">
                    {match.match_date && (
                        <div className="flex items-center space-x-1">
                            <Clock className="h-4 w-4" />
                            <span>{formatDate(match.match_date)}</span>
                        </div>
                    )}
                    {match.venue && (
                        <div className="flex items-center space-x-1">
                            <MapPin className="h-4 w-4" />
                            <span>{match.venue}</span>
                        </div>
                    )}
                </div>
            </div>

            {/* Main scoreboard */}
            <div className="flex items-center justify-center space-x-8">
                {/* Home Team */}
                <div className="text-center flex-1">
                    <div className="text-2xl font-bold mb-2">{match.home_team_name}</div>
                    <div className="text-6xl font-bold">{stats.home_total_score}</div>
                    <div className="text-sm text-blue-200 mt-1">
                        {stats.home_detailed_goals} detailed • {match.home_score_unidentified} unidentified
                    </div>
                </div>

                {/* VS and Status */}
                <div className="text-center">
                    <div className="text-xl text-blue-200 mb-2">VS</div>
                    <Badge color={getStatusColor(match.status)} className="text-lg px-4 py-2">
                        {getStatusDisplay(match.status)}
                    </Badge>
                </div>

                {/* Away Team */}
                <div className="text-center flex-1">
                    <div className="text-2xl font-bold mb-2">{match.away_team_name}</div>
                    <div className="text-6xl font-bold">{stats.away_total_score}</div>
                    <div className="text-sm text-blue-200 mt-1">
                        {stats.away_detailed_goals} detailed • {match.away_score_unidentified} unidentified
                    </div>
                </div>
            </div>
        </div>
    )
}

function ScoreEventsTimeline({ matchId, homeTeamId }: { 
    matchId: string; 
    homeTeamId: string; 
    awayTeamId: string; 
}) {
    const { data: events } = useSuspenseQuery(matchQueries.scoreEvents(matchId))
    const deleteEventMutation = useDeleteScoreEvent()

    const handleDeleteEvent = async (eventId: string) => {
        if (confirm('Are you sure you want to delete this goal?')) {
            deleteEventMutation.mutate({ matchId, eventId })
        }
    }

    const formatTime = (period: number | null, minutes: number | null, seconds: number | null) => {
        if (!period) return ''
        
        let periodStr = ''
        switch (period) {
            case 1:
            case 2:
            case 3:
                periodStr = `P${period}`
                break
            case 4:
                periodStr = 'OT'
                break
            case 5:
                periodStr = 'SO'
                break
            default:
                periodStr = `P${period}`
        }

        if (minutes !== null && seconds !== null) {
            return `${periodStr} ${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
        } else if (minutes !== null) {
            return `${periodStr} ${minutes}:00`
        }
        
        return periodStr
    }

    return (
        <div className="bg-white rounded-lg shadow-sm border border-gray-200">
            <div className="px-6 py-4 border-b border-gray-200">
                <h3 className="text-lg font-semibold text-gray-900">Score Events</h3>
            </div>
            
            <div className="p-6">
                {events.length === 0 ? (
                    <div className="text-center py-8 text-gray-500">
                        <Target className="h-8 w-8 mx-auto mb-2 text-gray-400" />
                        <p>No goals scored yet</p>
                    </div>
                ) : (
                    <div className="space-y-3">
                        {events.map((event) => (
                            <div key={event.id} className="flex items-center justify-between p-3 bg-gray-50 rounded-md">
                                <div className="flex items-center space-x-4">
                                    <div className="text-sm font-medium text-gray-600 min-w-[60px]">
                                        {formatTime(event.period, event.time_minutes, event.time_seconds)}
                                    </div>
                                    
                                    <div className={`w-3 h-3 rounded-full ${
                                        event.team_id === homeTeamId ? 'bg-blue-500' : 'bg-red-500'
                                    }`} />
                                    
                                    <div className="flex-1">
                                        <div className="text-sm font-medium text-gray-900">
                                            {event.scorer_name || 'Unknown scorer'}
                                            {event.goal_type && (
                                                <span className="ml-2 text-xs text-gray-500">
                                                    ({event.goal_type.replace('_', ' ')})
                                                </span>
                                            )}
                                        </div>
                                        {(event.assist1_name || event.assist2_name) && (
                                            <div className="text-xs text-gray-500">
                                                Assists: {[event.assist1_name, event.assist2_name].filter(Boolean).join(', ')}
                                            </div>
                                        )}
                                    </div>
                                </div>
                                
                                <button
                                    onClick={() => handleDeleteEvent(event.id)}
                                    disabled={deleteEventMutation.isPending}
                                    className="text-red-600 hover:text-red-900 p-1 rounded-md hover:bg-red-50 transition-colors disabled:opacity-50"
                                    title="Delete goal"
                                >
                                    <Trash2 className="h-4 w-4" />
                                </button>
                            </div>
                        ))}
                    </div>
                )}
            </div>
        </div>
    )
}

function MatchDetailsContent({ matchId }: { matchId: string }) {
    const router = useRouter()
    const { data: match } = useSuspenseQuery(matchQueries.byId(matchId))
    const { data: stats } = useSuspenseQuery(matchQueries.withStats(matchId))
    
    const [isAddGoalDialogOpen, setIsAddGoalDialogOpen] = useState(false)
    const [isIdentifyGoalDialogOpen, setIsIdentifyGoalDialogOpen] = useState(false)
    const [isEditMatchDialogOpen, setIsEditMatchDialogOpen] = useState(false)

    return (
        <div className="space-y-6">
            {/* Header with back button */}
            <div className="flex items-center space-x-4">
                <button
                    onClick={() => router.back()}
                    className="flex items-center space-x-2 text-gray-600 hover:text-gray-900 transition-colors"
                >
                    <ArrowLeft className="h-5 w-5" />
                    <span>Back to Matches</span>
                </button>
            </div>

            {/* Hockey Scoreboard */}
            <MatchScoreboard match={match} stats={stats} />

            {/* Action Buttons */}
            <div className="flex flex-wrap gap-3">
                <button 
                    onClick={() => setIsAddGoalDialogOpen(true)}
                    className="inline-flex items-center px-4 py-2 bg-green-600 text-white text-sm font-medium rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 transition-colors"
                >
                    <Plus className="h-4 w-4 mr-2" />
                    Add Goal
                </button>
                
                {(match.home_score_unidentified > 0 || match.away_score_unidentified > 0) && (
                    <button 
                        onClick={() => setIsIdentifyGoalDialogOpen(true)}
                        className="inline-flex items-center px-4 py-2 bg-orange-600 text-white text-sm font-medium rounded-md hover:bg-orange-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-orange-500 transition-colors"
                    >
                        <Target className="h-4 w-4 mr-2" />
                        Identify Goal ({match.home_score_unidentified + match.away_score_unidentified} unidentified)
                    </button>
                )}
                
                <button 
                    onClick={() => setIsEditMatchDialogOpen(true)}
                    className="inline-flex items-center px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
                >
                    <Edit className="h-4 w-4 mr-2" />
                    Edit Match
                </button>
            </div>

            {/* Score Events Timeline */}
            <ErrorBoundary
                fallback={
                    <div className="p-4 text-center text-red-600 bg-red-50 rounded-md">
                        Error loading score events. Please refresh the page.
                    </div>
                }
            >
                <Suspense fallback={
                    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
                        <div className="animate-pulse space-y-3">
                            <div className="h-4 bg-gray-200 rounded w-1/4"></div>
                            <div className="h-8 bg-gray-200 rounded"></div>
                            <div className="h-8 bg-gray-200 rounded"></div>
                        </div>
                    </div>
                }>
                    <ScoreEventsTimeline 
                        matchId={matchId} 
                        homeTeamId={match.home_team_id} 
                        awayTeamId={match.away_team_id} 
                    />
                </Suspense>
            </ErrorBoundary>

            {/* Dialogs */}
            <AddGoalDialog
                isOpen={isAddGoalDialogOpen}
                onClose={() => setIsAddGoalDialogOpen(false)}
                matchId={matchId}
                seasonId={match.season_id}
                homeTeamId={match.home_team_id}
                homeTeamName={match.home_team_name}
                awayTeamId={match.away_team_id}
                awayTeamName={match.away_team_name}
            />

            <IdentifyGoalDialog
                isOpen={isIdentifyGoalDialogOpen}
                onClose={() => setIsIdentifyGoalDialogOpen(false)}
                matchId={matchId}
                seasonId={match.season_id}
                homeTeamId={match.home_team_id}
                homeTeamName={match.home_team_name}
                awayTeamId={match.away_team_id}
                awayTeamName={match.away_team_name}
                homeUnidentified={match.home_score_unidentified}
                awayUnidentified={match.away_score_unidentified}
            />

            <MatchEditDialog
                isOpen={isEditMatchDialogOpen}
                onClose={() => setIsEditMatchDialogOpen(false)}
                match={match}
            />
        </div>
    )
}

export default function MatchDetailsPage({ matchId }: MatchDetailsPageProps) {
    return (
        <QueryErrorBoundary
            fallback={
                <div className="p-4 text-center text-red-600 bg-red-50 rounded-md">
                    Failed to load match details. Please check the match ID and try again.
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
                <div className="space-y-6">
                    <div className="animate-pulse">
                        <div className="h-8 bg-gray-200 rounded w-1/4 mb-6"></div>
                        <div className="h-32 bg-gray-200 rounded mb-6"></div>
                        <div className="h-64 bg-gray-200 rounded"></div>
                    </div>
                </div>
            }>
                <MatchDetailsContent matchId={matchId} />
            </Suspense>
        </QueryErrorBoundary>
    )
}