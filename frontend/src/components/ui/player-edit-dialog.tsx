"use client"

import React, { useEffect } from 'react'
import { Dialog, DialogPanel, DialogTitle } from '@headlessui/react'
import { X } from 'lucide-react'
import { useQuery } from '@tanstack/react-query'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { useHotkeys } from 'react-hotkeys-hook'
import { countryQueries } from '@/queries/countries'
import { useUpdatePlayer } from '@/queries/players'
import type { Player } from '@/types/player'

// Form validation schema  
const playerEditSchema = z.object({
  name: z.string().min(1, 'Player name is required'),
  country_id: z.string().min(1, 'Country is required'),
})

type PlayerEditFormData = z.infer<typeof playerEditSchema>

interface PlayerEditDialogProps {
  isOpen: boolean
  onClose: () => void
  player: Player | null
}

export default function PlayerEditDialog({ isOpen, onClose, player }: PlayerEditDialogProps) {
  const { data: countries = [], isLoading: countriesLoading } = useQuery(countryQueries.all())
  const updatePlayerMutation = useUpdatePlayer()
  
  const {
    register,
    handleSubmit,
    watch,
    reset,
    setFocus,
    formState: { errors, isValid },
  } = useForm<PlayerEditFormData>({
    resolver: zodResolver(playerEditSchema),
    defaultValues: {
      name: player?.name || '',
      country_id: player?.country_id?.toString() || '',
    },
  })
  
  const watchedValues = watch()

  useEffect(() => {
    if (isOpen && player) {
      reset({
        name: player.name,
        country_id: player.country_id.toString(),
      })
      setTimeout(() => setFocus('name'), 100)
    }
  }, [isOpen, player, reset, setFocus])

  useHotkeys('escape', onClose, { enabled: isOpen })
  useHotkeys('enter', () => {
    if (isValid) {
      handleSubmit(onSubmit)()
    }
  }, { enabled: isOpen })

  const onSubmit = async (data: PlayerEditFormData) => {
    if (!player) return

    try {
      await updatePlayerMutation.mutateAsync({
        id: player.id,
        data: {
          name: data.name,
          country_id: data.country_id,
        }
      })
      onClose()
      reset()
    } catch (error) {
      // Error handled by mutation
    }
  }

  const handleClose = () => {
    reset()
    onClose()
  }

  if (!player) return null

  // Generate preview
  const selectedCountry = countries.find(c => c.id.toString() === watchedValues.country_id)
  const previewName = watchedValues.name || player.name
  const previewCountry = selectedCountry?.name || 'Unknown Country'

  return (
    <Dialog open={isOpen} onClose={handleClose} className="relative z-50">
      <div className="fixed inset-0 bg-black/30" aria-hidden="true" />
      <div className="fixed inset-0 flex w-screen items-center justify-center p-4">
        <DialogPanel className="max-w-lg w-full bg-white rounded-xl shadow-2xl">
          <div className="flex items-center justify-between p-6 border-b border-gray-200">
            <DialogTitle className="text-lg font-semibold text-gray-900">
              Edit Player
            </DialogTitle>
            <button
              onClick={handleClose}
              className="text-gray-400 hover:text-gray-600 transition-colors"
            >
              <X className="h-5 w-5" />
            </button>
          </div>

          <form onSubmit={handleSubmit(onSubmit)} className="p-6 space-y-6">
            <div>
              <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-2">
                Player Name <span className="text-red-500">*</span>
              </label>
              <input
                type="text"
                id="name"
                {...register('name')}
                placeholder="Enter player name"
                className={`w-full px-3 py-2 border rounded-lg shadow-sm placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${
                  errors.name ? 'border-red-300' : 'border-gray-300'
                }`}
                disabled={updatePlayerMutation.isPending}
              />
              {errors.name && (
                <p className="mt-1 text-sm text-red-600">{errors.name.message}</p>
              )}
            </div>

            <div>
              <label htmlFor="country_id" className="block text-sm font-medium text-gray-700 mb-2">
                Country <span className="text-red-500">*</span>
              </label>
              <select
                id="country_id"
                {...register('country_id')}
                className={`w-full px-3 py-2 border rounded-lg shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${
                  errors.country_id ? 'border-red-300' : 'border-gray-300'
                }`}
                disabled={updatePlayerMutation.isPending || countriesLoading}
              >
                <option value="">Select a country</option>
                {countries.map((country) => (
                  <option key={country.id} value={country.id}>
                    {country.name}
                  </option>
                ))}
              </select>
              {errors.country_id && (
                <p className="mt-1 text-sm text-red-600">{errors.country_id.message}</p>
              )}
            </div>

            {/* Preview */}
            <div className="bg-gray-50 rounded-lg p-4 border border-gray-200">
              <h4 className="text-sm font-medium text-gray-700 mb-2">Preview</h4>
              <div className="text-sm text-gray-600">
                <p><strong>Player:</strong> {previewName}</p>
                <p><strong>Country:</strong> {previewCountry}</p>
              </div>
            </div>

            <div className="flex items-center space-x-3 pt-4">
              <button
                type="submit"
                disabled={!isValid || updatePlayerMutation.isPending}
                className="flex-1 bg-blue-600 text-white py-2 px-4 rounded-lg hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                {updatePlayerMutation.isPending ? 'Updating...' : 'Update Player'}
              </button>
              <button
                type="button"
                onClick={handleClose}
                className="px-4 py-2 text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500 transition-colors"
              >
                Cancel
              </button>
            </div>
          </form>
        </DialogPanel>
      </div>
    </Dialog>
  )
}