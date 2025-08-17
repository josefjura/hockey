"use client"

import { useState, useEffect, Suspense } from 'react'
import { X, Target, Plus } from 'lucide-react'
import { useQuery } from '@tanstack/react-query'
import { useIdentifyGoal } from '@/queries/matches'
import QuickAddPlayerDialog from './quick-add-player-dialog'
import type { CreateScoreEventRequest } from '@/types/match'

interface Player {
    id: number
    name: string
    country_name?: string
}

interface IdentifyGoalDialogProps {
    isOpen: boolean
    onClose: () => void
    matchId: string
    seasonId: string
    homeTeamId: string
    homeTeamName: string
    awayTeamId: string
    awayTeamName: string
    homeUnidentified: number
    awayUnidentified: number
}

function IdentifyGoalForm({ 
    onClose, 
    matchId, 
    seasonId,
    homeTeamId, 
    homeTeamName, 
    awayTeamId, 
    awayTeamName,
    homeUnidentified,
    awayUnidentified
}: { 
    onClose: () => void
    matchId: string
    seasonId: string
    homeTeamId: string
    homeTeamName: string
    awayTeamId: string
    awayTeamName: string
    homeUnidentified: number
    awayUnidentified: number
}) {
    const [formData, setFormData] = useState<CreateScoreEventRequest>({
        team_id: '',
        scorer_id: '',
        assist1_id: '',
        assist2_id: '',
        period: 1,
        time_minutes: 0,
        time_seconds: 0,
        goal_type: 'even_strength',
    })
    
    const [errors, setErrors] = useState<Record<string, string>>({})
    const [isQuickAddOpen, setIsQuickAddOpen] = useState(false)
    
    const identifyGoalMutation = useIdentifyGoal()

    // Get roster data for the selected team
    const { data: rosterPlayers } = useQuery({
        queryKey: ['season', seasonId, 'team', formData.team_id, 'players'],
        queryFn: async () => {
            if (!formData.team_id || !seasonId) return []
            
            const response = await fetch(`${process.env.BACKEND_URL || 'http://localhost:8080'}/season/${seasonId}/team/${formData.team_id}/players`)
            if (!response.ok) {
                if (response.status === 404) {
                    return [] // No players in roster yet
                }
                throw new Error('Failed to fetch roster players')
            }
            return response.json()
        },
        enabled: !!formData.team_id && !!seasonId,
    })

    // Reset form when component mounts
    useEffect(() => {
        setFormData({
            team_id: '',
            scorer_id: '',
            assist1_id: '',
            assist2_id: '',
            period: 1,
            time_minutes: 0,
            time_seconds: 0,
            goal_type: 'even_strength',
        })
        setErrors({})
    }, [])

    const handleQuickAddPlayer = () => {
        if (!formData.team_id) {
            alert('Please select a team first')
            return
        }
        setIsQuickAddOpen(true)
    }

    const validateForm = (): boolean => {
        const newErrors: Record<string, string> = {}

        if (!formData.team_id) {
            newErrors.team_id = 'Team is required'
        }

        // Check if the selected team has unidentified goals
        if (formData.team_id === homeTeamId && homeUnidentified === 0) {
            newErrors.team_id = 'Home team has no unidentified goals to convert'
        }
        if (formData.team_id === awayTeamId && awayUnidentified === 0) {
            newErrors.team_id = 'Away team has no unidentified goals to convert'
        }

        if (formData.period && (formData.period < 1 || formData.period > 5)) {
            newErrors.period = 'Period must be between 1-5'
        }
        if (formData.time_minutes && (formData.time_minutes < 0 || formData.time_minutes > 60)) {
            newErrors.time_minutes = 'Minutes must be between 0-60'
        }
        if (formData.time_seconds && (formData.time_seconds < 0 || formData.time_seconds > 59)) {
            newErrors.time_seconds = 'Seconds must be between 0-59'
        }

        setErrors(newErrors)
        return Object.keys(newErrors).length === 0
    }

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        
        if (!validateForm()) {
            return
        }

        try {
            await identifyGoalMutation.mutateAsync({
                matchId,
                ...formData,
                scorer_id: formData.scorer_id || undefined,
                assist1_id: formData.assist1_id || undefined,
                assist2_id: formData.assist2_id || undefined,
                period: formData.period || undefined,
                time_minutes: formData.time_minutes || undefined,
                time_seconds: formData.time_seconds || undefined,
                goal_type: formData.goal_type || undefined,
            })
            onClose()
        } catch (error) {
            console.error('Failed to identify goal:', error)
        }
    }

    const handleChange = (field: keyof CreateScoreEventRequest, value: string | number) => {
        setFormData(prev => ({
            ...prev,
            [field]: value
        }))
        
        // Clear error when user starts typing
        if (errors[field]) {
            setErrors(prev => ({
                ...prev,
                [field]: ''
            }))
        }
    }
    
    // Only show teams that have unidentified goals
    const availableTeams = []
    if (homeUnidentified > 0) {
        availableTeams.push({ id: homeTeamId, name: homeTeamName, unidentified: homeUnidentified })
    }
    if (awayUnidentified > 0) {
        availableTeams.push({ id: awayTeamId, name: awayTeamName, unidentified: awayUnidentified })
    }

    // Get players for the selected team from the roster
    const availablePlayers: Player[] = rosterPlayers || []

    if (availableTeams.length === 0) {
        return (
            <div className="p-6 text-center">
                <Target className="h-12 w-12 mx-auto text-gray-400 mb-4" />
                <h3 className="text-lg font-medium text-gray-900 mb-2">No Goals to Identify</h3>
                <p className="text-gray-500 mb-4">
                    All goals in this match have already been identified with detailed information.
                </p>
                <button
                    onClick={onClose}
                    className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
                >
                    Close
                </button>
            </div>
        )
    }

    return (
        <form onSubmit={handleSubmit} className="p-6 space-y-4">
            {/* Info Banner */}
            <div className="bg-orange-50 border border-orange-200 rounded-md p-3">
                <div className="flex">
                    <Target className="h-5 w-5 text-orange-400 mr-2 mt-0.5" />
                    <div>
                        <h3 className="text-sm font-medium text-orange-800">
                            Converting Unidentified Goal
                        </h3>
                        <p className="text-sm text-orange-700 mt-1">
                            This will convert one unidentified goal into a detailed score event. 
                            The total score will remain the same.
                        </p>
                    </div>
                </div>
            </div>

            {/* Team Selection */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                    Which team scored this goal? *
                </label>
                <select
                    value={formData.team_id}
                    onChange={(e) => handleChange('team_id', e.target.value)}
                    className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 ${
                        errors.team_id ? 'border-red-300' : 'border-gray-300'
                    }`}
                >
                    <option value="">Select team</option>
                    {availableTeams.map((team) => (
                        <option key={team.id} value={team.id}>
                            {team.name} ({team.unidentified} unidentified goal{team.unidentified > 1 ? 's' : ''})
                        </option>
                    ))}
                </select>
                {errors.team_id && (
                    <p className="mt-1 text-xs text-red-600">{errors.team_id}</p>
                )}
            </div>

            {/* Goal Time */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                    When was this goal scored?
                </label>
                <div className="grid grid-cols-3 gap-3">
                    <div>
                        <label className="block text-xs text-gray-600 mb-1">Period</label>
                        <select
                            value={formData.period || ''}
                            onChange={(e) => handleChange('period', parseInt(e.target.value) || 1)}
                            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                        >
                            <option value={1}>P1</option>
                            <option value={2}>P2</option>
                            <option value={3}>P3</option>
                            <option value={4}>OT</option>
                            <option value={5}>SO</option>
                        </select>
                    </div>
                    <div>
                        <label className="block text-xs text-gray-600 mb-1">Minutes</label>
                        <input
                            type="number"
                            min="0"
                            max="60"
                            value={formData.time_minutes || ''}
                            onChange={(e) => handleChange('time_minutes', parseInt(e.target.value) || 0)}
                            className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 ${
                                errors.time_minutes ? 'border-red-300' : 'border-gray-300'
                            }`}
                        />
                        {errors.time_minutes && (
                            <p className="mt-1 text-xs text-red-600">{errors.time_minutes}</p>
                        )}
                    </div>
                    <div>
                        <label className="block text-xs text-gray-600 mb-1">Seconds</label>
                        <input
                            type="number"
                            min="0"
                            max="59"
                            value={formData.time_seconds || ''}
                            onChange={(e) => handleChange('time_seconds', parseInt(e.target.value) || 0)}
                            className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 ${
                                errors.time_seconds ? 'border-red-300' : 'border-gray-300'
                            }`}
                        />
                        {errors.time_seconds && (
                            <p className="mt-1 text-xs text-red-600">{errors.time_seconds}</p>
                        )}
                    </div>
                </div>
            </div>

            {/* Goal Type */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                    Goal Type
                </label>
                <select
                    value={formData.goal_type || ''}
                    onChange={(e) => handleChange('goal_type', e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                >
                    <option value="even_strength">Even Strength</option>
                    <option value="power_play">Power Play</option>
                    <option value="short_handed">Short Handed</option>
                    <option value="penalty_shot">Penalty Shot</option>
                    <option value="empty_net">Empty Net</option>
                </select>
            </div>

            {/* Scorer */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                    Who scored this goal? (optional)
                </label>
                <select
                    value={formData.scorer_id || ''}
                    onChange={(e) => handleChange('scorer_id', e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                >
                    <option value="">Unknown/No scorer specified</option>
                    {availablePlayers.map((player) => (
                        <option key={player.id} value={player.id}>
                            {player.name}
                        </option>
                    ))}
                </select>
                {formData.team_id && (
                    <button
                        type="button"
                        onClick={handleQuickAddPlayer}
                        className="mt-2 inline-flex items-center text-sm text-blue-600 hover:text-blue-800 transition-colors"
                    >
                        <Plus className="h-4 w-4 mr-1" />
                        Quick add player to roster
                    </button>
                )}
            </div>

            {/* Assists */}
            <div className="grid grid-cols-2 gap-4">
                <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                        First Assist (optional)
                    </label>
                    <select
                        value={formData.assist1_id || ''}
                        onChange={(e) => handleChange('assist1_id', e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                    >
                        <option value="">No assist</option>
                        {availablePlayers.map((player) => (
                            <option key={player.id} value={player.id}>
                                {player.name}
                            </option>
                        ))}
                    </select>
                    {formData.team_id && (
                        <button
                            type="button"
                            onClick={handleQuickAddPlayer}
                            className="mt-1 inline-flex items-center text-xs text-blue-600 hover:text-blue-800 transition-colors"
                        >
                            <Plus className="h-3 w-3 mr-1" />
                            Quick add
                        </button>
                    )}
                </div>
                <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                        Second Assist (optional)
                    </label>
                    <select
                        value={formData.assist2_id || ''}
                        onChange={(e) => handleChange('assist2_id', e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                    >
                        <option value="">No assist</option>
                        {availablePlayers.map((player) => (
                            <option key={player.id} value={player.id}>
                                {player.name}
                            </option>
                        ))}
                    </select>
                    {formData.team_id && (
                        <button
                            type="button"
                            onClick={handleQuickAddPlayer}
                            className="mt-1 inline-flex items-center text-xs text-blue-600 hover:text-blue-800 transition-colors"
                        >
                            <Plus className="h-3 w-3 mr-1" />
                            Quick add
                        </button>
                    )}
                </div>
            </div>

            {/* Form Actions */}
            <div className="flex justify-end space-x-3 pt-4">
                <button
                    type="button"
                    onClick={onClose}
                    className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
                >
                    Cancel
                </button>
                <button
                    type="submit"
                    disabled={identifyGoalMutation.isPending}
                    className="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-orange-600 border border-transparent rounded-md hover:bg-orange-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-orange-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                    {identifyGoalMutation.isPending ? (
                        <>
                            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                            Identifying...
                        </>
                    ) : (
                        <>
                            <Target className="h-4 w-4 mr-2" />
                            Identify Goal
                        </>
                    )}
                </button>
            </div>

            {/* Quick Add Player Dialog */}
            <QuickAddPlayerDialog
                isOpen={isQuickAddOpen}
                onClose={() => setIsQuickAddOpen(false)}
                seasonId={seasonId}
                teamId={formData.team_id}
                teamName={formData.team_id === homeTeamId ? homeTeamName : awayTeamName}
            />
        </form>
    )
}

export default function IdentifyGoalDialog({ 
    isOpen, 
    onClose, 
    matchId, 
    seasonId,
    homeTeamId, 
    homeTeamName, 
    awayTeamId, 
    awayTeamName,
    homeUnidentified,
    awayUnidentified
}: IdentifyGoalDialogProps) {
    if (!isOpen) {
        return null
    }

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-lg shadow-xl max-w-lg w-full max-h-[90vh] overflow-y-auto">
                <div className="flex items-center justify-between p-6 border-b border-gray-200">
                    <h2 className="text-lg font-semibold text-gray-900">Identify Goal</h2>
                    <button
                        onClick={onClose}
                        className="text-gray-400 hover:text-gray-600 transition-colors"
                    >
                        <X className="h-5 w-5" />
                    </button>
                </div>

                <Suspense fallback={
                    <div className="p-6">
                        <div className="animate-pulse space-y-4">
                            <div className="h-4 bg-gray-200 rounded w-1/4"></div>
                            <div className="h-10 bg-gray-200 rounded"></div>
                            <div className="h-4 bg-gray-200 rounded w-1/4"></div>
                            <div className="h-10 bg-gray-200 rounded"></div>
                        </div>
                    </div>
                }>
                    <IdentifyGoalForm 
                        onClose={onClose}
                        matchId={matchId}
                        seasonId={seasonId}
                        homeTeamId={homeTeamId}
                        homeTeamName={homeTeamName}
                        awayTeamId={awayTeamId}
                        awayTeamName={awayTeamName}
                        homeUnidentified={homeUnidentified}
                        awayUnidentified={awayUnidentified}
                    />
                </Suspense>
            </div>
        </div>
    )
}