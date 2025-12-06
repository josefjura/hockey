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
import { useCreateEvent } from '@/queries/events'
import { getCountryFlag } from '@/utils/countryFlag'
import Image from 'next/image'

// Form validation schema  
const eventCreateSchema = z.object({
  name: z.string().min(1, 'Event name is required'),
  country_id: z.string().optional(),
})

type EventCreateFormData = z.infer<typeof eventCreateSchema>

interface EventCreateDialogProps {
  isOpen: boolean
  onClose: () => void
}

export default function EventCreateDialog({ isOpen, onClose }: EventCreateDialogProps) {
  const { data: session } = useSession()
  const { data: countries = [], isLoading: countriesLoading } = useQuery(countryQueries.all(session?.accessToken))
  const createEventMutation = useCreateEvent()

  const {
    register,
    handleSubmit,
    watch,
    reset,
		setFocus,
    formState: { errors, isValid },
  } = useForm<EventCreateFormData>({
    resolver: zodResolver(eventCreateSchema),
    defaultValues: {
      name: '',
      country_id: '',
    },
  })

  const watchedValues = watch()

	useEffect(() => {
		if (isOpen) {
			// Add a small delay to ensure the dialog is fully rendered
			setTimeout(() => {
				setFocus('name')
			}, 100)
		}
	}, [isOpen, setFocus])

  const onSubmit = async (data: EventCreateFormData) => {
    try {
      await createEventMutation.mutateAsync({
        data: {
          name: data.name.trim(),
          country_id: data.country_id || null,
        },
        accessToken: session?.accessToken
      })

      // Reset form and close dialog
      reset()
      onClose()
    } catch {
      // Error is handled by the mutation
    }
  }

  const handleClose = () => {
    if (!createEventMutation.isPending) {
      reset()
      onClose()
    }
  }

  // Keyboard shortcuts
  useHotkeys(
    'shift+enter',
    (e) => {
      e.preventDefault()
      if (isValid && !createEventMutation.isPending) {
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
      enabled: isOpen && !createEventMutation.isPending,
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
              Create New Event
            </DialogTitle>
            <button
              onClick={handleClose}
              disabled={createEventMutation.isPending}
              className="text-gray-400 hover:text-gray-600 disabled:opacity-50"
            >
              <X className="h-5 w-5" />
            </button>
          </div>

          {/* Form */}
          <form onSubmit={handleSubmit(onSubmit)} className="p-6 space-y-4">
            {/* Event Name */}
            <div>
              <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-1">
                Event Name <span className="text-red-500">*</span>
              </label>
              <input
                type="text"
                id="name"
                {...register('name')}
                placeholder="Enter event name"
                className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 placeholder-gray-400 ${
                  errors.name ? 'border-red-300' : 'border-gray-300'
                }`}
                disabled={createEventMutation.isPending}
              />
              {errors.name && (
                <p className="mt-1 text-sm text-red-600">{errors.name.message}</p>
              )}
            </div>

            {/* Country Selection */}
            <div>
              <label htmlFor="country" className="block text-sm font-medium text-gray-700 mb-1">
                Country
              </label>
              {countriesLoading ? (
                <div className="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50">
                  Loading countries...
                </div>
              ) : (
                <select
                  id="country"
                  {...register('country_id')}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 bg-white"
                  style={{ color: '#111827' }}
                  disabled={createEventMutation.isPending}
                >
                  <option value="" className="text-gray-500">Select a country (optional)</option>
                  {countries.map((country) => (
                    <option key={country.id} value={country.id} className="text-gray-900">
                      {country.name}
                    </option>
                  ))}
                </select>
              )}
              <p className="mt-1 text-xs text-gray-500">
                Leave empty for international events
              </p>
            </div>

            {/* Selected Country Preview */}
            {watchedValues.country_id && (
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
                        {watchedValues.name?.trim() || 'Event'} - {selectedCountry.name}
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
                disabled={createEventMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={!isValid || createEventMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {createEventMutation.isPending ? 'Creating...' : 'Create Event'}
              </button>
            </div>
          </form>
        </DialogPanel>
      </div>
    </Dialog>
  )
}
