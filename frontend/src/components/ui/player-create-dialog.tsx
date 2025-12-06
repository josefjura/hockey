"use client"

import React, { useEffect } from 'react'
import { Dialog, DialogPanel, DialogTitle } from '@headlessui/react'
import { X } from 'lucide-react'
import { useSession } from 'next-auth/react'
import { useQuery } from '@tanstack/react-query'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { useHotkeys } from 'react-hotkeys-hook'
import { countryQueries } from '@/queries/countries'
import { useCreatePlayer } from '@/queries/players'
import { getCountryFlag } from '@/utils/countryFlag'
import Image from 'next/image'

// Form validation schema
const playerCreateSchema = z.object({
  name: z.string().min(1, 'Player name is required'),
  country_id: z.string().min(1, 'Country is required'),
})

type PlayerCreateFormData = z.infer<typeof playerCreateSchema>

interface PlayerCreateDialogProps {
  isOpen: boolean
  onClose: () => void
}

export default function PlayerCreateDialog({ isOpen, onClose }: PlayerCreateDialogProps) {
  console.log('[PlayerCreateDialog] Render - isOpen:', isOpen)

  const { data: session } = useSession()
  const { data: countries = [], isLoading: countriesLoading } = useQuery(countryQueries.all(session?.accessToken))
  const createPlayerMutation = useCreatePlayer()
  
  const {
    register,
    handleSubmit,
    watch,
    reset,
		setFocus,
    formState: { errors, isValid },
  } = useForm<PlayerCreateFormData>({
    resolver: zodResolver(playerCreateSchema),
    defaultValues: {
      name: '',
      country_id: '',
    },
  })
  
  const watchedValues = watch()
  console.log('[PlayerCreateDialog] watchedValues:', watchedValues)

	useEffect(() => {
		console.log('[PlayerCreateDialog] useEffect triggered - isOpen:', isOpen)
		if (isOpen) {
			// Add a small delay to ensure the dialog is fully rendered
			setTimeout(() => {
				console.log('[PlayerCreateDialog] Setting focus to name')
				setFocus('name')
			}, 100)
		}
	}, [isOpen, setFocus])

  const onSubmit = async (data: PlayerCreateFormData) => {
    console.log('[PlayerCreateDialog] onSubmit called with data:', data)
    try {
      await createPlayerMutation.mutateAsync({
        playerData: {
          name: data.name.trim(),
          country_id: data.country_id,
        },
        accessToken: session?.accessToken
      })

      // Reset form and close dialog
      console.log('[PlayerCreateDialog] Success - resetting and closing')
      reset()
      onClose()
    } catch (error) {
      console.log('[PlayerCreateDialog] Error in onSubmit:', error)
      // Error is handled by the mutation
    }
  }

  const handleClose = () => {
    console.log('[PlayerCreateDialog] handleClose called - isPending:', createPlayerMutation.isPending)
    if (!createPlayerMutation.isPending) {
      console.log('[PlayerCreateDialog] Closing dialog - resetting form')
      reset()
      onClose()
    }
  }

  // Keyboard shortcuts
  useHotkeys(
    'shift+enter',
    (e) => {
      e.preventDefault()
      if (isValid && !createPlayerMutation.isPending) {
        handleSubmit(onSubmit)()
      }
    },
    {
      enabled: isOpen, // Only active when dialog is open
      enableOnFormTags: ['input', 'select'], // Allow in form fields
    }
  )

  useHotkeys(
    'escape',
    () => {
      handleClose()
    },
    {
      enabled: isOpen && !createPlayerMutation.isPending,
    }
  )

  return (
    <Dialog open={isOpen} onClose={handleClose} className="relative z-50">
      {/* Backdrop */}
      <div className="fixed inset-0 bg-black bg-opacity-25" aria-hidden="true" />

      {/* Dialog container */}
      <div className="fixed inset-0 flex items-center justify-center p-4">
        <DialogPanel className="w-full max-w-md bg-white rounded-lg shadow-xl">
          {/* Header */}
          <div className="flex items-center justify-between p-6 border-b border-gray-200">
            <DialogTitle as="h3" className="text-lg font-semibold text-gray-900">
              Create New Player
            </DialogTitle>
            <button
              onClick={handleClose}
              disabled={createPlayerMutation.isPending}
              className="text-gray-400 hover:text-gray-600 disabled:opacity-50"
            >
              <X className="h-5 w-5" />
            </button>
          </div>

          {/* Form */}
          <form onSubmit={handleSubmit(onSubmit)} className="p-6 space-y-4">
            {/* Player Name */}
            <div>
              <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-1">
                Player Name <span className="text-red-500">*</span>
              </label>
              <input
                type="text"
                id="name"
                {...register('name')}
                placeholder="Enter player name"
                className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 placeholder-gray-400 ${
                  errors.name ? 'border-red-300' : 'border-gray-300'
                }`}
                disabled={createPlayerMutation.isPending}
              />
              {errors.name && (
                <p className="mt-1 text-sm text-red-600">{errors.name.message}</p>
              )}
            </div>

            {/* Country Selection */}
            <div>
              <label htmlFor="country" className="block text-sm font-medium text-gray-700 mb-1">
                Country <span className="text-red-500">*</span>
              </label>
              {countriesLoading ? (
                <div className="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50">
                  Loading countries...
                </div>
              ) : (
                <select
                  id="country"
                  {...register('country_id')}
                  className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 bg-white ${
                    errors.country_id ? 'border-red-300' : 'border-gray-300'
                  }`}
                  style={{ color: '#111827' }}
                  disabled={createPlayerMutation.isPending}
                >
                  <option value="" className="text-gray-500">Select a country</option>
                  {countries.map((country) => (
                    <option key={country.id} value={country.id} className="text-gray-900">
                      {country.name}
                    </option>
                  ))}
                </select>
              )}
              {errors.country_id && (
                <p className="mt-1 text-sm text-red-600">{errors.country_id.message}</p>
              )}
            </div>

            {/* Selected Country Preview */}
            {watchedValues.country_id && watchedValues.name && (
              <div className="p-3 bg-gray-50 rounded-md">
                {(() => {
                  const selectedCountry = countries.find(c => c.id === watchedValues.country_id)
                  if (!selectedCountry) return null
                  
                  return (
                    <div className="flex items-center space-x-2">
                      <Image 
                        width={24} 
                        height={18} 
                        src={getCountryFlag(selectedCountry.iso2_code ?? '', false)} 
                        alt={selectedCountry.iso2_code ?? ''} 
                        className='shadow-sm shadow-black' 
                      />
                      <span className="text-sm text-gray-700">
                        {watchedValues.name.trim()} - {selectedCountry.name}
                      </span>
                    </div>
                  )
                })()}
              </div>
            )}

            {/* Keyboard shortcuts hint */}
            <div className="text-xs text-gray-500 border-t border-gray-100 pt-3">
              <div className="flex justify-between items-center">
                <span>Keyboard shortcuts:</span>
                <div className="flex space-x-3">
                  <span><kbd className="px-1.5 py-0.5 text-xs bg-gray-100 border border-gray-300 rounded">Shift</kbd> + <kbd className="px-1.5 py-0.5 text-xs bg-gray-100 border border-gray-300 rounded">Enter</kbd> to submit</span>
                  <span><kbd className="px-1.5 py-0.5 text-xs bg-gray-100 border border-gray-300 rounded">Esc</kbd> to cancel</span>
                </div>
              </div>
            </div>

            {/* Actions */}
            <div className="flex justify-end space-x-3 pt-4">
              <button
                type="button"
                onClick={handleClose}
                disabled={createPlayerMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={!isValid || createPlayerMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {createPlayerMutation.isPending ? 'Creating...' : 'Create Player'}
              </button>
            </div>
          </form>
        </DialogPanel>
      </div>
    </Dialog>
  )
}