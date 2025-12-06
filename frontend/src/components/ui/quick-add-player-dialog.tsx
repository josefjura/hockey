"use client"

import { useState, useEffect } from 'react'
import { X, Plus, Search, User } from 'lucide-react'
import { useSession } from 'next-auth/react'
import { useAddPlayerToRoster, searchPlayers } from '@/queries/roster'

interface QuickAddPlayerDialogProps {
    isOpen: boolean
    onClose: () => void
    seasonId: string
    teamId: string
    teamName: string
}

function QuickAddPlayerForm({
    onClose,
    seasonId,
    teamId,
    teamName
}: {
    onClose: () => void
    seasonId: string
    teamId: string
    teamName: string
}) {
    const { data: session } = useSession()
    const [searchTerm, setSearchTerm] = useState('')
    const [isCreatingNew, setIsCreatingNew] = useState(false)
    const [searchResults, setSearchResults] = useState<Array<{id: number, name: string, nationality: string}>>([])
    const [selectedPlayer, setSelectedPlayer] = useState<{id: number, name: string, nationality: string} | null>(null)
    const [isSearching, setIsSearching] = useState(false)
    const [newPlayerData, setNewPlayerData] = useState({
        name: '',
        nationality: 'Czech Republic' // Default
    })

    const addPlayerMutation = useAddPlayerToRoster()

    // Search for players when search term changes with debounce
    useEffect(() => {
        if (searchTerm.trim().length <= 2) {
            setSearchResults([])
            return
        }

        const timeoutId = setTimeout(async () => {
            try {
                setIsSearching(true)
                const results = await searchPlayers(searchTerm)
                setSearchResults(results)
            } catch (error) {
                console.error('Search failed:', error)
                setSearchResults([])
            } finally {
                setIsSearching(false)
            }
        }, 300) // 300ms debounce

        return () => clearTimeout(timeoutId)
    }, [searchTerm]) // Only depend on searchTerm

    const handleQuickAdd = async () => {
        try {
            if (selectedPlayer) {
                // Add existing player
                await addPlayerMutation.mutateAsync({
                    teamSeasonId: teamId,
                    playerData: { id: selectedPlayer.id },
                    accessToken: session?.accessToken
                })
            } else if (isCreatingNew && newPlayerData.name.trim()) {
                // Create new player and add to roster
                await addPlayerMutation.mutateAsync({
                    teamSeasonId: teamId,
                    playerData: {
                        name: newPlayerData.name,
                        nationality: newPlayerData.nationality
                    },
                    accessToken: session?.accessToken
                })
            } else {
                return // Invalid state
            }
            
            onClose()
        } catch (error) {
            console.error('Failed to add player to roster:', error)
        }
    }

    const handlePlayerSelect = (player: {id: number, name: string, nationality: string}) => {
        setSelectedPlayer(player)
        setSearchTerm(`${player.name} (${player.nationality})`)
        setSearchResults([])
    }

    const clearSelection = () => {
        setSelectedPlayer(null)
        setSearchTerm('')
        setSearchResults([])
    }

    return (
        <div className="p-6 space-y-4">
            {/* Info Banner */}
            <div className="bg-blue-50 border border-blue-200 rounded-md p-3">
                <div className="flex">
                    <User className="h-5 w-5 text-blue-400 mr-2 mt-0.5" />
                    <div>
                        <h3 className="text-sm font-medium text-blue-800">
                            Add Player to {teamName} Roster
                        </h3>
                        <p className="text-sm text-blue-700 mt-1">
                            This will add the player to the team roster for this season.
                        </p>
                    </div>
                </div>
            </div>

            {!isCreatingNew ? (
                /* Search Mode */
                <div className="space-y-3">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">
                            Search Existing Players
                        </label>
                        <div className="relative">
                            <Search className="absolute left-3 top-2.5 h-4 w-4 text-gray-400" />
                            <input
                                type="text"
                                value={searchTerm}
                                onChange={(e) => setSearchTerm(e.target.value)}
                                placeholder="Type player name..."
                                className="w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                            />
                            {selectedPlayer && (
                                <button
                                    type="button"
                                    onClick={clearSelection}
                                    className="absolute right-3 top-2.5 text-gray-400 hover:text-gray-600"
                                >
                                    <X className="h-4 w-4" />
                                </button>
                            )}
                        </div>
                        
                        {/* Search Results */}
                        {searchResults.length > 0 && !selectedPlayer && (
                            <div className="mt-1 border border-gray-300 rounded-md bg-white max-h-40 overflow-y-auto">
                                {searchResults.map((player) => (
                                    <button
                                        key={player.id}
                                        type="button"
                                        onClick={() => handlePlayerSelect(player)}
                                        className="w-full px-3 py-2 text-left hover:bg-gray-50 text-sm border-b border-gray-100 last:border-b-0"
                                    >
                                        <div className="font-medium text-gray-900">{player.name}</div>
                                        <div className="text-gray-500">{player.nationality}</div>
                                    </button>
                                ))}
                            </div>
                        )}
                        
                        {isSearching && (
                            <p className="mt-1 text-xs text-gray-500">
                                Searching...
                            </p>
                        )}
                        
                        {searchTerm.trim().length > 0 && searchTerm.trim().length <= 2 && (
                            <p className="mt-1 text-xs text-gray-500">
                                Type at least 3 characters to search
                            </p>
                        )}
                    </div>
                    
                    <div className="text-center py-4">
                        <p className="text-sm text-gray-500 mb-3">Or</p>
                        <button
                            type="button"
                            onClick={() => setIsCreatingNew(true)}
                            className="inline-flex items-center px-3 py-2 text-sm font-medium text-blue-600 bg-blue-50 border border-blue-200 rounded-md hover:bg-blue-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 transition-colors"
                        >
                            <Plus className="h-4 w-4 mr-2" />
                            Create New Player
                        </button>
                    </div>
                </div>
            ) : (
                /* Create New Mode */
                <div className="space-y-3">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">
                            Player Name *
                        </label>
                        <input
                            type="text"
                            value={newPlayerData.name}
                            onChange={(e) => setNewPlayerData(prev => ({ ...prev, name: e.target.value }))}
                            placeholder="e.g., Jaromir Jagr"
                            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                        />
                    </div>
                    
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">
                            Nationality *
                        </label>
                        <input
                            type="text"
                            value={newPlayerData.nationality}
                            onChange={(e) => setNewPlayerData(prev => ({ ...prev, nationality: e.target.value }))}
                            placeholder="e.g., Czech Republic"
                            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900"
                        />
                    </div>

                    <button
                        type="button"
                        onClick={() => setIsCreatingNew(false)}
                        className="text-sm text-gray-600 hover:text-gray-800 transition-colors"
                    >
                        ← Back to search
                    </button>
                </div>
            )}

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
                    type="button"
                    onClick={handleQuickAdd}
                    disabled={
                        addPlayerMutation.isPending || 
                        (!selectedPlayer && (!isCreatingNew || !newPlayerData.name.trim()))
                    }
                    className="inline-flex items-center px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                    {addPlayerMutation.isPending ? (
                        <>
                            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
                            Adding...
                        </>
                    ) : (
                        <>
                            <Plus className="h-4 w-4 mr-2" />
                            Add to Roster
                        </>
                    )}
                </button>
            </div>
        </div>
    )
}

export default function QuickAddPlayerDialog({ 
    isOpen, 
    onClose, 
    seasonId, 
    teamId, 
    teamName 
}: QuickAddPlayerDialogProps) {
    if (!isOpen) {
        return null
    }

    return (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-[60]">
            <div className="bg-white rounded-lg shadow-xl max-w-md w-full max-h-[90vh] overflow-y-auto">
                <div className="flex items-center justify-between p-6 border-b border-gray-200">
                    <h2 className="text-lg font-semibold text-gray-900">Quick Add Player</h2>
                    <button
                        onClick={onClose}
                        className="text-gray-400 hover:text-gray-600 transition-colors"
                    >
                        <X className="h-5 w-5" />
                    </button>
                </div>

                <QuickAddPlayerForm 
                    onClose={onClose}
                    seasonId={seasonId}
                    teamId={teamId}
                    teamName={teamName}
                />
            </div>
        </div>
    )
}