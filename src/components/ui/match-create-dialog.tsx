"use client"

import { useState, useEffect, Suspense } from 'react'
import { X, Plus } from 'lucide-react'
import { useSuspenseQuery } from '@tanstack/react-query'
import { useCreateMatch } from '@/queries/matches'
import { teamQueries } from '@/queries/teams'
import { seasonQueries } from '@/queries/seasons'
import type { CreateMatchRequest } from '@/types/match'

interface MatchCreateDialogProps {
    isOpen: boolean
    onClose: () => void
}

function MatchCreateForm({ onClose }: { onClose: () => void }) {
    const teamsData = useSuspenseQuery(teamQueries.all())
    const seasonsData = useSuspenseQuery(seasonQueries.all())
    
    const [formData, setFormData] = useState<CreateMatchRequest>({
        season_id: '',
        home_team_id: '',
        away_team_id: '',
        home_score_unidentified: 0,
        away_score_unidentified: 0,
        match_date: '',
        status: 'scheduled',
        venue: '',
    })
    
    const [errors, setErrors] = useState<Record<string, string>>({})
    const createMatchMutation = useCreateMatch()

    // Reset form when component mounts
    useEffect(() => {
        setFormData({
            season_id: '',
            home_team_id: '',
            away_team_id: '',
            home_score_unidentified: 0,
            away_score_unidentified: 0,
            match_date: '',
            status: 'scheduled',
            venue: '',
        })
        setErrors({})
    }, [])

    const validateForm = (): boolean => {
        const newErrors: Record<string, string> = {}

        if (!formData.season_id) {
            newErrors.season_id = 'Season is required'
        }
        if (!formData.home_team_id) {
            newErrors.home_team_id = 'Home team is required'
        }
        if (!formData.away_team_id) {
            newErrors.away_team_id = 'Away team is required'
        }
        if (formData.home_team_id === formData.away_team_id) {
            newErrors.away_team_id = 'Away team must be different from home team'
        }
        if (formData.home_score_unidentified < 0) {
            newErrors.home_score_unidentified = 'Score cannot be negative'
        }
        if (formData.away_score_unidentified < 0) {
            newErrors.away_score_unidentified = 'Score cannot be negative'
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
            await createMatchMutation.mutateAsync({
                ...formData,
                match_date: formData.match_date || undefined,
                venue: formData.venue || undefined,
            })
            onClose()
        } catch (error) {
            console.error('Failed to create match:', error)
        }
    }

    const handleChange = (field: keyof CreateMatchRequest, value: string | number) => {
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

    const teams = teamsData || []
    const seasons = seasonsData || []

    return (
        <form onSubmit={handleSubmit} className="p-6 space-y-4">
            {/* Season Selection */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                    Season *
                </label>
                <select
                    value={formData.season_id}
                    onChange={(e) => handleChange('season_id', e.target.value)}
                    className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 ${
                        errors.season_id ? 'border-red-300' : 'border-gray-300'
                    }`}
                >
                    <option value="">Select season</option>
                    {seasons.map((season) => (
                        <option key={season.id} value={season.id}>
                            {season.name || season.year} ({season.event_name})
                        </option>
                    ))}
                </select>
                {errors.season_id && (
                    <p className="mt-1 text-xs text-red-600">{errors.season_id}</p>
                )}
            </div>

            {/* Home Team Selection */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                    Home Team *
                </label>
                <select
                    value={formData.home_team_id}
                    onChange={(e) => handleChange('home_team_id', e.target.value)}
                    className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 ${
                        errors.home_team_id ? 'border-red-300' : 'border-gray-300'
                    }`}
                >
                    <option value="">Select home team</option>
                    {teams.map((team) => (
                        <option key={team.id} value={team.id}>
                            {team.name}
                        </option>
                    ))}
                </select>
                {errors.home_team_id && (
                    <p className="mt-1 text-xs text-red-600">{errors.home_team_id}</p>
                )}
            </div>

            {/* Away Team Selection */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                    Away Team *
                </label>
                <select
                    value={formData.away_team_id}
                    onChange={(e) => handleChange('away_team_id', e.target.value)}
                    className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 ${
                        errors.away_team_id ? 'border-red-300' : 'border-gray-300'
                    }`}
                >
                    <option value="">Select away team</option>
                    {teams.map((team) => (
                        <option key={team.id} value={team.id}>
                            {team.name}
                        </option>
                    ))}
                </select>
                {errors.away_team_id && (
                    <p className="mt-1 text-xs text-red-600">{errors.away_team_id}</p>
                )}
            </div>

            {/* Final Score */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">
                    Final Score (optional)
                </label>
                <div className="grid grid-cols-2 gap-4">
                    <div>
                        <label className="block text-xs text-gray-600 mb-1">
                            {formData.home_team_id ? teams.find(t => t.id === formData.home_team_id)?.name || 'Home Team' : 'Home Team'}
                        </label>
                        <input
                            type="number"
                            min="0"
                            value={formData.home_score_unidentified}
                            onChange={(e) => handleChange('home_score_unidentified', parseInt(e.target.value) || 0)}
                            className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 ${
                                errors.home_score_unidentified ? 'border-red-300' : 'border-gray-300'
                            }`}
                        />
                        {errors.home_score_unidentified && (
                            <p className="mt-1 text-xs text-red-600">{errors.home_score_unidentified}</p>
                        )}
                    </div>
                    <div>
                        <label className="block text-xs text-gray-600 mb-1">
                            {formData.away_team_id ? teams.find(t => t.id === formData.away_team_id)?.name || 'Away Team' : 'Away Team'}
                        </label>
                        <input
                            type="number"
                            min="0"
                            value={formData.away_score_unidentified}
                            onChange={(e) => handleChange('away_score_unidentified', parseInt(e.target.value) || 0)}
                            className={`w-full px-3 py-2 border rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 ${
                                errors.away_score_unidentified ? 'border-red-300' : 'border-gray-300'
                            }`}
                        />
                        {errors.away_score_unidentified && (
                            <p className="mt-1 text-xs text-red-600">{errors.away_score_unidentified}</p>
                        )}
                    </div>
                </div>
                <p className="mt-1 text-xs text-gray-500">
                    💡 Leave at 0-0 for new matches, or enter final scores for completed games. You can add detailed goal information later.
                </p>
            </div>

            {/* Match Date */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                    Match Date & Time
                </label>
                <input
                    type="datetime-local"
                    value={formData.match_date}
                    onChange={(e) => handleChange('match_date', e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                />
            </div>

            {/* Status */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                    Status
                </label>
                <select
                    value={formData.status}
                    onChange={(e) => handleChange('status', e.target.value)}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                >
                    <option value="scheduled">Scheduled</option>
                    <option value="in_progress">In Progress</option>
                    <option value="finished">Finished</option>
                    <option value="cancelled">Cancelled</option>
                </select>
            </div>

            {/* Venue */}
            <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                    Venue
                </label>
                <input
                    type="text"
                    value={formData.venue}
                    onChange={(e) => handleChange('venue', e.target.value)}
                    placeholder="e.g., Arena Name"
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                />
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
                    disabled={createMatchMutation.isPending}
                    className="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                    {createMatchMutation.isPending ? (
                        <>
                            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                            Creating...
                        </>
                    ) : (
                        <>
                            <Plus className="h-4 w-4 mr-2" />
                            Create Match
                        </>
                    )}
                </button>
            </div>
        </form>
    )
}

export default function MatchCreateDialog({ isOpen, onClose }: MatchCreateDialogProps) {
    if (!isOpen) {
        return null
    }

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-lg shadow-xl max-w-md w-full max-h-[90vh] overflow-y-auto">
                <div className="flex items-center justify-between p-6 border-b border-gray-200">
                    <h2 className="text-lg font-semibold text-gray-900">Create New Match</h2>
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
                            <div className="h-4 bg-gray-200 rounded w-1/4"></div>
                            <div className="h-10 bg-gray-200 rounded"></div>
                        </div>
                    </div>
                }>
                    <MatchCreateForm onClose={onClose} />
                </Suspense>
            </div>
        </div>
    )
}