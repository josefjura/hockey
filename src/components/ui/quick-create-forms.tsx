"use client"

import { useState } from 'react'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { Users, Activity, Trophy, Check, AlertCircle } from 'lucide-react'
import { useQuery } from '@tanstack/react-query'
import { countryQueries } from '@/queries/countries'
import { useCreateTeam } from '@/queries/teams'
import { useCreatePlayer } from '@/queries/players'
import { useCreateEvent } from '@/queries/events'
import { getCountryFlag } from '@/utils/countryFlag'
import Image from 'next/image'

// Validation schemas
const quickTeamSchema = z.object({
  name: z.string().min(1, 'Team name is required'),
  country_id: z.string().min(1, 'Country is required'),
})

const quickPlayerSchema = z.object({
  name: z.string().min(1, 'Player name is required'),
  country_id: z.string().min(1, 'Country is required'),
})

const quickEventSchema = z.object({
  name: z.string().min(1, 'Event name is required'),
  country_id: z.string().min(1, 'Host country is required'),
})

type QuickTeamData = z.infer<typeof quickTeamSchema>
type QuickPlayerData = z.infer<typeof quickPlayerSchema>
type QuickEventData = z.infer<typeof quickEventSchema>

interface QuickCreateTeamProps {
  onSuccess?: () => void
}

export function QuickCreateTeam({ onSuccess }: QuickCreateTeamProps) {
  const [isExpanded, setIsExpanded] = useState(false)
  const { data: countries = [] } = useQuery(countryQueries.all())
  const createTeamMutation = useCreateTeam()

  const { register, handleSubmit, reset, formState: { errors, isValid } } = useForm<QuickTeamData>({
    resolver: zodResolver(quickTeamSchema),
    defaultValues: { name: '', country_id: '' },
  })

  const onSubmit = async (data: QuickTeamData) => {
    try {
      await createTeamMutation.mutateAsync({
        name: data.name.trim(),
        country_id: data.country_id,
      })
      reset()
      setIsExpanded(false)
      onSuccess?.()
    } catch (error) {
      // Error handled by mutation
    }
  }

  if (!isExpanded) {
    return (
      <button 
        onClick={() => setIsExpanded(true)}
        className="flex items-center p-4 text-left border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors w-full"
      >
        <Users className="h-6 w-6 text-blue-500 mr-3" />
        <div>
          <div className="font-medium text-gray-900">Quick Add Team</div>
          <div className="text-sm text-gray-500">Create a new team quickly</div>
        </div>
      </button>
    )
  }

  return (
    <div className="border border-blue-200 rounded-lg p-4 bg-blue-50">
      <div className="flex items-center mb-3">
        <Users className="h-5 w-5 text-blue-500 mr-2" />
        <h4 className="font-medium text-gray-900">Quick Add Team</h4>
      </div>
      
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-3">
        <div>
          <input
            type="text"
            {...register('name')}
            placeholder="Team name"
            className={`w-full px-3 py-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 placeholder-gray-500 ${
              errors.name ? 'border-red-300' : 'border-gray-300'
            }`}
            disabled={createTeamMutation.isPending}
          />
          {errors.name && <p className="text-xs text-red-600 mt-1">{errors.name.message}</p>}
        </div>

        <div>
          <select
            {...register('country_id')}
            className={`w-full px-3 py-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 bg-white ${
              errors.country_id ? 'border-red-300' : 'border-gray-300'
            }`}
            disabled={createTeamMutation.isPending}
          >
            <option value="" className="text-gray-500">Select country</option>
            {countries.map((country) => (
              <option key={country.id} value={country.id} className="text-gray-900">
                {country.name}
              </option>
            ))}
          </select>
          {errors.country_id && <p className="text-xs text-red-600 mt-1">{errors.country_id.message}</p>}
        </div>

        <div className="flex items-center space-x-2">
          <button
            type="submit"
            disabled={!isValid || createTeamMutation.isPending}
            className="flex-1 px-3 py-2 bg-blue-600 text-white text-sm font-medium rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {createTeamMutation.isPending ? 'Creating...' : 'Create Team'}
          </button>
          <button
            type="button"
            onClick={() => {
              reset()
              setIsExpanded(false)
            }}
            className="px-3 py-2 text-sm text-gray-900 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            Cancel
          </button>
        </div>
      </form>
    </div>
  )
}

export function QuickCreatePlayer({ onSuccess }: QuickCreateTeamProps) {
  const [isExpanded, setIsExpanded] = useState(false)
  const { data: countries = [] } = useQuery(countryQueries.all())
  const createPlayerMutation = useCreatePlayer()

  const { register, handleSubmit, reset, formState: { errors, isValid } } = useForm<QuickPlayerData>({
    resolver: zodResolver(quickPlayerSchema),
    defaultValues: { name: '', country_id: '' },
  })

  const onSubmit = async (data: QuickPlayerData) => {
    try {
      await createPlayerMutation.mutateAsync({
        name: data.name.trim(),
        country_id: data.country_id,
      })
      reset()
      setIsExpanded(false)
      onSuccess?.()
    } catch (error) {
      // Error handled by mutation
    }
  }

  if (!isExpanded) {
    return (
      <button 
        onClick={() => setIsExpanded(true)}
        className="flex items-center p-4 text-left border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors w-full"
      >
        <Activity className="h-6 w-6 text-green-500 mr-3" />
        <div>
          <div className="font-medium text-gray-900">Quick Add Player</div>
          <div className="text-sm text-gray-500">Create a new player quickly</div>
        </div>
      </button>
    )
  }

  return (
    <div className="border border-green-200 rounded-lg p-4 bg-green-50">
      <div className="flex items-center mb-3">
        <Activity className="h-5 w-5 text-green-500 mr-2" />
        <h4 className="font-medium text-gray-900">Quick Add Player</h4>
      </div>
      
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-3">
        <div>
          <input
            type="text"
            {...register('name')}
            placeholder="Player name"
            className={`w-full px-3 py-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-green-500 text-gray-900 placeholder-gray-500 ${
              errors.name ? 'border-red-300' : 'border-gray-300'
            }`}
            disabled={createPlayerMutation.isPending}
          />
          {errors.name && <p className="text-xs text-red-600 mt-1">{errors.name.message}</p>}
        </div>

        <div>
          <select
            {...register('country_id')}
            className={`w-full px-3 py-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-green-500 text-gray-900 bg-white ${
              errors.country_id ? 'border-red-300' : 'border-gray-300'
            }`}
            disabled={createPlayerMutation.isPending}
          >
            <option value="" className="text-gray-500">Select country</option>
            {countries.map((country) => (
              <option key={country.id} value={country.id} className="text-gray-900">
                {country.name}
              </option>
            ))}
          </select>
          {errors.country_id && <p className="text-xs text-red-600 mt-1">{errors.country_id.message}</p>}
        </div>

        <div className="flex items-center space-x-2">
          <button
            type="submit"
            disabled={!isValid || createPlayerMutation.isPending}
            className="flex-1 px-3 py-2 bg-green-600 text-white text-sm font-medium rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {createPlayerMutation.isPending ? 'Creating...' : 'Create Player'}
          </button>
          <button
            type="button"
            onClick={() => {
              reset()
              setIsExpanded(false)
            }}
            className="px-3 py-2 text-sm text-gray-900 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500"
          >
            Cancel
          </button>
        </div>
      </form>
    </div>
  )
}

export function QuickCreateEvent({ onSuccess }: QuickCreateTeamProps) {
  const [isExpanded, setIsExpanded] = useState(false)
  const { data: countries = [] } = useQuery(countryQueries.all())
  const createEventMutation = useCreateEvent()

  const { register, handleSubmit, reset, formState: { errors, isValid } } = useForm<QuickEventData>({
    resolver: zodResolver(quickEventSchema),
    defaultValues: { name: '', country_id: '' },
  })

  const onSubmit = async (data: QuickEventData) => {
    try {
      await createEventMutation.mutateAsync({
        name: data.name.trim(),
        country_id: data.country_id,
      })
      reset()
      setIsExpanded(false)
      onSuccess?.()
    } catch (error) {
      // Error handled by mutation
    }
  }

  if (!isExpanded) {
    return (
      <button 
        onClick={() => setIsExpanded(true)}
        className="flex items-center p-4 text-left border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors w-full"
      >
        <Trophy className="h-6 w-6 text-yellow-500 mr-3" />
        <div>
          <div className="font-medium text-gray-900">Quick Create Event</div>
          <div className="text-sm text-gray-500">Create a new event quickly</div>
        </div>
      </button>
    )
  }

  return (
    <div className="border border-yellow-200 rounded-lg p-4 bg-yellow-50">
      <div className="flex items-center mb-3">
        <Trophy className="h-5 w-5 text-yellow-500 mr-2" />
        <h4 className="font-medium text-gray-900">Quick Create Event</h4>
      </div>
      
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-3">
        <div>
          <input
            type="text"
            {...register('name')}
            placeholder="Event name"
            className={`w-full px-3 py-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:border-yellow-500 text-gray-900 placeholder-gray-500 ${
              errors.name ? 'border-red-300' : 'border-gray-300'
            }`}
            disabled={createEventMutation.isPending}
          />
          {errors.name && <p className="text-xs text-red-600 mt-1">{errors.name.message}</p>}
        </div>

        <div>
          <select
            {...register('country_id')}
            className={`w-full px-3 py-2 text-sm border rounded-md focus:outline-none focus:ring-2 focus:ring-yellow-500 focus:border-yellow-500 text-gray-900 bg-white ${
              errors.country_id ? 'border-red-300' : 'border-gray-300'
            }`}
            disabled={createEventMutation.isPending}
          >
            <option value="">Select host country</option>
            {countries.map((country) => (
              <option key={country.id} value={country.id}>
                {country.name}
              </option>
            ))}
          </select>
          {errors.country_id && <p className="text-xs text-red-600 mt-1">{errors.country_id.message}</p>}
        </div>

        <div className="flex items-center space-x-2">
          <button
            type="submit"
            disabled={!isValid || createEventMutation.isPending}
            className="flex-1 px-3 py-2 bg-yellow-600 text-white text-sm font-medium rounded-md hover:bg-yellow-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-yellow-500 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {createEventMutation.isPending ? 'Creating...' : 'Create Event'}
          </button>
          <button
            type="button"
            onClick={() => {
              reset()
              setIsExpanded(false)
            }}
            className="px-3 py-2 text-sm text-gray-900 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-yellow-500"
          >
            Cancel
          </button>
        </div>
      </form>
    </div>
  )
}