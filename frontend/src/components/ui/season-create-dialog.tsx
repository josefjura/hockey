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
import { eventQueries } from '@/queries/events'
import { useCreateSeason } from '@/queries/seasons'

// Form validation schema
const seasonCreateSchema = z.object({
  year: z.number().min(1900, 'Year must be at least 1900').max(2100, 'Year must be at most 2100'),
  display_name: z.string().optional(),
  event_id: z.string().min(1, 'Event is required'),
})

type SeasonCreateFormData = z.infer<typeof seasonCreateSchema>

interface SeasonCreateDialogProps {
  isOpen: boolean
  onClose: () => void
}

export default function SeasonCreateDialog({ isOpen, onClose }: SeasonCreateDialogProps) {
  const { data: session } = useSession()
  const { data: events = [], isLoading: eventsLoading } = useQuery(eventQueries.all(session?.accessToken))
  const createSeasonMutation = useCreateSeason()
  
  const {
    register,
    handleSubmit,
    watch,
    reset,
    setFocus,
    formState: { errors, isValid },
  } = useForm<SeasonCreateFormData>({
    resolver: zodResolver(seasonCreateSchema),
    defaultValues: {
      year: new Date().getFullYear(),
      display_name: '',
      event_id: '',
    },
  })
  
  const watchedValues = watch()

  useEffect(() => {
    if (isOpen) {
      setFocus('year')
    }
  }, [isOpen, setFocus])

  useHotkeys('escape', onClose, { enabled: isOpen })

  const onSubmit = async (data: SeasonCreateFormData) => {
    try {
      await createSeasonMutation.mutateAsync({
        seasonData: {
          year: data.year,
          display_name: data.display_name || null,
          event_id: data.event_id,
        },
        accessToken: session?.accessToken
      })

      reset()
      onClose()
    } catch (error) {
      console.error('Failed to create season:', error)
    }
  }

  const handleClose = () => {
    reset()
    onClose()
  }

  return (
    <Dialog open={isOpen} onClose={handleClose} className="relative z-50">
      <div className="fixed inset-0 bg-black/25" />
      
      <div className="fixed inset-0 overflow-y-auto">
        <div className="flex min-h-full items-center justify-center p-4 text-center">
          <DialogPanel className="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all">
            <div className="flex items-center justify-between mb-4">
              <DialogTitle as="h3" className="text-lg font-medium leading-6 text-gray-900">
                Create New Season
              </DialogTitle>
              <button
                onClick={handleClose}
                className="rounded-md p-1 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <X className="h-5 w-5" />
              </button>
            </div>

            <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
              {/* Year */}
              <div>
                <label htmlFor="year" className="block text-sm font-medium text-gray-700 mb-1">
                  Year *
                </label>
                <input
                  {...register('year', { valueAsNumber: true })}
                  type="number"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  placeholder="e.g., 2024"
                />
                {errors.year && (
                  <p className="mt-1 text-sm text-red-600">{errors.year.message}</p>
                )}
              </div>

              {/* Display Name */}
              <div>
                <label htmlFor="display_name" className="block text-sm font-medium text-gray-700 mb-1">
                  Display Name
                </label>
                <input
                  {...register('display_name')}
                  type="text"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  placeholder="e.g., 2024 Championship Season"
                />
                {errors.display_name && (
                  <p className="mt-1 text-sm text-red-600">{errors.display_name.message}</p>
                )}
              </div>

              {/* Event */}
              <div>
                <label htmlFor="event_id" className="block text-sm font-medium text-gray-700 mb-1">
                  Event *
                </label>
                <select
                  {...register('event_id')}
                  className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                  disabled={eventsLoading}
                >
                  <option value="">
                    {eventsLoading ? 'Loading events...' : 'Select an event'}
                  </option>
                  {events.map((event) => (
                    <option key={event.id} value={event.id}>
                      {event.name}
                    </option>
                  ))}
                </select>
                {errors.event_id && (
                  <p className="mt-1 text-sm text-red-600">{errors.event_id.message}</p>
                )}
              </div>

              {/* Form Actions */}
              <div className="flex items-center justify-end space-x-3 pt-4">
                <button
                  type="button"
                  onClick={handleClose}
                  className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  disabled={!isValid || createSeasonMutation.isPending}
                  className="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {createSeasonMutation.isPending ? 'Creating...' : 'Create Season'}
                </button>
              </div>
            </form>

            {/* Preview */}
            {watchedValues.year && watchedValues.event_id && (
              <div className="mt-4 p-3 bg-blue-50 rounded-md">
                <h4 className="text-sm font-medium text-blue-900 mb-1">Preview:</h4>
                <p className="text-sm text-blue-700">
                  Season {watchedValues.year}
                  {watchedValues.display_name && ` "${watchedValues.display_name}"`}
                  {events.find(e => e.id === watchedValues.event_id) && 
                    ` for ${events.find(e => e.id === watchedValues.event_id)?.name}`}
                </p>
              </div>
            )}
          </DialogPanel>
        </div>
      </div>
    </Dialog>
  )
}