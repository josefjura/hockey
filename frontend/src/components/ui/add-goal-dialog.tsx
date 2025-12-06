"use client"

import { useState, useEffect, Suspense } from 'react'
import { X, Plus } from 'lucide-react'
import { useQuery } from '@tanstack/react-query'
import { useSession } from 'next-auth/react'
import { useCreateScoreEvent, matchQueries } from '@/queries/matches'
import QuickAddPlayerDialog from '@/components/ui/quick-add-player-dialog'
import type { CreateScoreEventRequest } from '@/types/match'

interface AddGoalDialogProps {
    isOpen: boolean
    onClose: () => void
    matchId: string
    seasonId: string
    homeTeamId: string
    homeTeamName: string
    awayTeamId: string
    awayTeamName: string
}

function AddGoalForm({ 
    onClose, 
    matchId, 
    seasonId,
    homeTeamId, 
    homeTeamName, 
    awayTeamId, 
    awayTeamName 
}: { 
    onClose: () => void
    matchId: string
    seasonId: string
    homeTeamId: string
    homeTeamName: string
    awayTeamId: string
    awayTeamName: string
}) {
    const { data: session } = useSession()
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
    const createScoreEventMutation = useCreateScoreEvent()
    
    // Get roster players for the selected team
    const { data: rosterPlayers } = useQuery(
        matchQueries.rosterPlayers(seasonId, formData.team_id)
    )

    // Debug logging
    useEffect(() => {
        console.log('AddGoalDialog - seasonId:', seasonId, 'teamId:', formData.team_id);
    }, [seasonId, formData.team_id])

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

    const validateForm = (): boolean => {
        const newErrors: Record<string, string> = {}

        if (!formData.team_id) {
            newErrors.team_id = 'Team is required'
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
            await createScoreEventMutation.mutateAsync({
                matchId,
                data: {
                    ...formData,
                    scorer_id: formData.scorer_id || undefined,
                    assist1_id: formData.assist1_id || undefined,
                    assist2_id: formData.assist2_id || undefined,
                    period: formData.period || undefined,
                    time_minutes: formData.time_minutes || undefined,
                    time_seconds: formData.time_seconds || undefined,
                    goal_type: formData.goal_type || undefined,
                },
                accessToken: session?.accessToken,
            })
            onClose()
        } catch (error) {
            console.error('Failed to add goal:', error)
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

    const matchTeams = [
        { id: homeTeamId, name: homeTeamName },
        { id: awayTeamId, name: awayTeamName }
    ]

    // Get current team name for quick add dialog
    const selectedTeam = matchTeams.find(team => team.id === formData.team_id)
    const selectedTeamName = selectedTeam?.name || ''

    return (
        <form onSubmit={handleSubmit} className="p-6 space-y-4">
            {/* Team Selection */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                    Scoring Team *
                </label>
                <select
                    value={formData.team_id}
                    onChange={(e) => handleChange('team_id', e.target.value)}
                    className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 ${
                        errors.team_id ? 'border-red-300' : 'border-gray-300'
                    }`}
                >
                    <option value="">Select team</option>
                    {matchTeams.map((team) => (
                        <option key={team.id} value={team.id}>
                            {team.name}
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
                    Goal Time
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
                    Scorer (optional)
                </label>
                <select
                    value={formData.scorer_id || ''}
                    onChange={(e) => handleChange('scorer_id', e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                >
                    <option value="">Unknown/No scorer specified</option>
                    {rosterPlayers?.map((player) => (
                        <option key={player.id} value={player.id}>
                            {player.name} ({player.nationality})
                        </option>
                    ))}
                </select>
                {formData.team_id && (
                    <button
                        type="button"
                        onClick={() => setIsQuickAddOpen(true)}
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
                        {rosterPlayers?.map((player) => (
                            <option key={player.id} value={player.id}>
                                {player.name} ({player.nationality})
                            </option>
                        ))}
                    </select>
                    {formData.team_id && (
                        <button
                            type="button"
                            onClick={() => setIsQuickAddOpen(true)}
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
                        {rosterPlayers?.map((player) => (
                            <option key={player.id} value={player.id}>
                                {player.name} ({player.nationality})
                            </option>
                        ))}
                    </select>
                    {formData.team_id && (
                        <button
                            type="button"
                            onClick={() => setIsQuickAddOpen(true)}
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
                    disabled={createScoreEventMutation.isPending}
                    className="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-green-600 border border-transparent rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                    {createScoreEventMutation.isPending ? (
                        <>
                            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                            Adding...
                        </>
                    ) : (
                        <>
                            <Plus className="h-4 w-4 mr-2" />
                            Add Goal
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
                teamName={selectedTeamName}
            />
        </form>
    )
}

export default function AddGoalDialog({ 
    isOpen, 
    onClose, 
    matchId, 
    seasonId,
    homeTeamId, 
    homeTeamName, 
    awayTeamId, 
    awayTeamName 
}: AddGoalDialogProps) {
    if (!isOpen) {
        return null
    }

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-lg shadow-xl max-w-lg w-full max-h-[90vh] overflow-y-auto">
                <div className="flex items-center justify-between p-6 border-b border-gray-200">
                    <h2 className="text-lg font-semibold text-gray-900">Add Goal</h2>
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
                    <AddGoalForm 
                        onClose={onClose}
                        matchId={matchId}
                        seasonId={seasonId}
                        homeTeamId={homeTeamId}
                        homeTeamName={homeTeamName}
                        awayTeamId={awayTeamId}
                        awayTeamName={awayTeamName}
                    />
                </Suspense>
            </div>
        </div>
    )
}