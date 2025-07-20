"use client"

import React from 'react'
import { Dialog, DialogPanel, DialogTitle } from '@headlessui/react'
import { X } from 'lucide-react'
import { useQuery } from '@tanstack/react-query'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { countryQueries, useCreateTeam } from '@/queries/teams'
import { getCountryFlag } from '@/utils/countryFlag'
import Image from 'next/image'

// Form validation schema  
const teamCreateSchema = z.object({
  name: z.string().optional(),
  country_id: z.string().min(1, 'Country is required'),
})

type TeamCreateFormData = z.infer<typeof teamCreateSchema>

interface TeamCreateDialogProps {
  isOpen: boolean
  onClose: () => void
}

export default function TeamCreateDialog({ isOpen, onClose }: TeamCreateDialogProps) {
  const { data: countries = [], isLoading: countriesLoading } = useQuery(countryQueries.list())
  const createTeamMutation = useCreateTeam()
  
  const {
    register,
    handleSubmit,
    watch,
    reset,
    formState: { errors, isValid },
  } = useForm<TeamCreateFormData>({
    resolver: zodResolver(teamCreateSchema),
    defaultValues: {
      name: '',
      country_id: '',
    },
  })
  
  const watchedValues = watch()

  const onSubmit = async (data: TeamCreateFormData) => {
    try {
      await createTeamMutation.mutateAsync({
        name: data.name?.trim() || null,
        country_id: data.country_id,
      })
      
      // Reset form and close dialog
      reset()
      onClose()
    } catch {
      // Error is handled by the mutation
    }
  }

  const handleClose = () => {
    if (!createTeamMutation.isPending) {
      reset()
      onClose()
    }
  }

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
              Create New Team
            </DialogTitle>
            <button
              onClick={handleClose}
              disabled={createTeamMutation.isPending}
              className="text-gray-400 hover:text-gray-600 disabled:opacity-50"
            >
              <X className="h-5 w-5" />
            </button>
          </div>

          {/* Form */}
          <form onSubmit={handleSubmit(onSubmit)} className="p-6 space-y-4">
            {/* Team Name */}
            <div>
              <label htmlFor="name" className="block text-sm font-medium text-gray-700 mb-1">
                Team Name
              </label>
              <input
                type="text"
                id="name"
                {...register('name')}
                placeholder="Enter team name (optional for national teams)"
                className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                disabled={createTeamMutation.isPending}
              />
              <p className="mt-1 text-xs text-gray-500">
                Leave empty for national teams
              </p>
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
                  className={`w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 ${
                    errors.country_id ? 'border-red-300' : 'border-gray-300'
                  }`}
                  disabled={createTeamMutation.isPending}
                >
                  <option value="">Select a country</option>
                  {countries.map((country) => (
                    <option key={country.id} value={country.id}>
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
                        {watchedValues.name?.trim() || 'National Team'} - {selectedCountry.name}
                      </span>
                    </div>
                  )
                })()}
              </div>
            )}

            {/* Actions */}
            <div className="flex justify-end space-x-3 pt-4">
              <button
                type="button"
                onClick={handleClose}
                disabled={createTeamMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={!isValid || createTeamMutation.isPending}
                className="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {createTeamMutation.isPending ? 'Creating...' : 'Create Team'}
              </button>
            </div>
          </form>
        </DialogPanel>
      </div>
    </Dialog>
  )
}