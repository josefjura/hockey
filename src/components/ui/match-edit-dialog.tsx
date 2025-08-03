"use client"

import { useState, useEffect } from 'react'
import { X, Save } from 'lucide-react'
import { useUpdateMatch } from '@/queries/matches'
import type { Match, UpdateMatchRequest } from '@/types/match'

interface MatchEditDialogProps {
    isOpen: boolean
    onClose: () => void
    match: Match | null
}

export default function MatchEditDialog({ isOpen, onClose, match }: MatchEditDialogProps) {
    const [formData, setFormData] = useState<UpdateMatchRequest>({})
    const [errors, setErrors] = useState<Record<string, string>>({})
    const updateMatchMutation = useUpdateMatch()

    // Initialize form data when dialog opens or match changes
    useEffect(() => {
        if (isOpen && match) {
            // Convert match date to datetime-local format if it exists
            let matchDate = ''
            if (match.match_date) {
                try {
                    const date = new Date(match.match_date)
                    matchDate = date.toISOString().slice(0, 16) // Format for datetime-local
                } catch {
                    matchDate = match.match_date
                }
            }

            setFormData({
                season_id: match.season_id,
                home_team_id: match.home_team_id,
                away_team_id: match.away_team_id,
                home_score_unidentified: match.home_score_unidentified,
                away_score_unidentified: match.away_score_unidentified,
                match_date: matchDate,
                status: match.status,
                venue: match.venue || '',
            })
            setErrors({})
        }
    }, [isOpen, match])

    const validateForm = (): boolean => {
        const newErrors: Record<string, string> = {}

        if (formData.season_id && !formData.season_id) {
            newErrors.season_id = 'Season is required'
        }
        if (formData.home_team_id && !formData.home_team_id) {
            newErrors.home_team_id = 'Home team is required'
        }
        if (formData.away_team_id && !formData.away_team_id) {
            newErrors.away_team_id = 'Away team is required'
        }
        if (formData.home_team_id && formData.away_team_id && formData.home_team_id === formData.away_team_id) {
            newErrors.away_team_id = 'Away team must be different from home team'
        }
        if (formData.home_score_unidentified !== undefined && formData.home_score_unidentified < 0) {
            newErrors.home_score_unidentified = 'Score cannot be negative'
        }
        if (formData.away_score_unidentified !== undefined && formData.away_score_unidentified < 0) {
            newErrors.away_score_unidentified = 'Score cannot be negative'
        }

        setErrors(newErrors)
        return Object.keys(newErrors).length === 0
    }

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault()
        
        if (!match || !validateForm()) {
            return
        }

        try {
            // Only send fields that have values
            const updateData: UpdateMatchRequest = {}
            
            Object.entries(formData).forEach(([key, value]) => {
                if (value !== undefined && value !== '' && value !== null) {
                    if (key === 'match_date' && value) {
                        // Convert datetime-local to ISO string
                        updateData[key as keyof UpdateMatchRequest] = new Date(value as string).toISOString()
                    } else {
                        (updateData as Record<string, unknown>)[key] = value
                    }
                }
            })

            await updateMatchMutation.mutateAsync({
                id: match.id,
                ...updateData,
            })
            onClose()
        } catch (error) {
            console.error('Failed to update match:', error)
        }
    }

    const handleChange = (field: keyof UpdateMatchRequest, value: string | number) => {
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

    if (!isOpen || !match) {
        return null
    }

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
            <div className="bg-white rounded-lg shadow-xl max-w-md w-full max-h-[90vh] overflow-y-auto">
                <div className="flex items-center justify-between p-6 border-b border-gray-200">
                    <h2 className="text-lg font-semibold text-gray-900">Edit Match</h2>
                    <button
                        onClick={onClose}
                        className="text-gray-400 hover:text-gray-600 transition-colors"
                    >
                        <X className="h-5 w-5" />
                    </button>
                </div>

                <form onSubmit={handleSubmit} className="p-6 space-y-4">
                    {/* Match Info Display */}
                    <div className="bg-gray-50 p-4 rounded-md">
                        <h3 className="text-sm font-medium text-gray-700 mb-2">Current Match</h3>
                        <p className="text-sm text-gray-900">
                            {match.home_team_name} vs {match.away_team_name}
                        </p>
                        <p className="text-xs text-gray-500">
                            Season: {match.season_name} | ID: {match.id}
                        </p>
                    </div>

                    {/* Season Selection - Read Only for now */}
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">
                            Season
                        </label>
                        <input
                            type="text"
                            value={match.season_name}
                            readOnly
                            className="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-500"
                        />
                        <p className="mt-1 text-xs text-gray-500">Season cannot be changed after creation</p>
                    </div>

                    {/* Team Selection - Read Only for now */}
                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">
                                Home Team
                            </label>
                            <input
                                type="text"
                                value={match.home_team_name}
                                readOnly
                                className="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-500"
                            />
                        </div>
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">
                                Away Team
                            </label>
                            <input
                                type="text"
                                value={match.away_team_name}
                                readOnly
                                className="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50 text-gray-500"
                            />
                        </div>
                    </div>
                    <p className="text-xs text-gray-500">Teams cannot be changed after creation</p>

                    {/* Score Fields */}
                    <div className="grid grid-cols-2 gap-4">
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">
                                Home Score (Unidentified)
                            </label>
                            <input
                                type="number"
                                min="0"
                                value={formData.home_score_unidentified ?? ''}
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
                            <label className="block text-sm font-medium text-gray-700 mb-1">
                                Away Score (Unidentified)
                            </label>
                            <input
                                type="number"
                                min="0"
                                value={formData.away_score_unidentified ?? ''}
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

                    {/* Match Date */}
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">
                            Match Date & Time
                        </label>
                        <input
                            type="datetime-local"
                            value={formData.match_date || ''}
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
                            value={formData.status || ''}
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
                            value={formData.venue || ''}
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
                            disabled={updateMatchMutation.isPending}
                            className="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                        >
                            {updateMatchMutation.isPending ? (
                                <>
                                    <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                                    Saving...
                                </>
                            ) : (
                                <>
                                    <Save className="h-4 w-4 mr-2" />
                                    Save Changes
                                </>
                            )}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    )
}