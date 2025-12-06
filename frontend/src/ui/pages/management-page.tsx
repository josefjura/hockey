"use client"

import { useState, useEffect, Suspense } from 'react'
import { Globe, Search, Settings } from 'lucide-react'
import { useTranslations } from 'next-intl'
import { useSession } from 'next-auth/react'
import type { Country } from '@/types/country'
import { getCountryFlag } from '@/utils/countryFlag'
import Image from 'next/image'
import Pager from '@/components/ui/pager'
import Badge from '@/components/ui/badge'
import TableSkeleton from '@/components/ui/table-skeleton'
import { useSuspenseQuery } from '@tanstack/react-query'
import { countryQueries, useUpdateCountryStatus } from '@/queries/countries'
import { useDebounce } from '@/hooks/useDebounce'
import ErrorBoundary from '@/components/error-boundary'
import QueryErrorBoundary from '@/components/query-error-boundary'

function CountriesTable({ searchTerm, page, onPageChange }: {
    searchTerm: string
    page: number
    onPageChange: (page: number) => void
}) {
    const t = useTranslations('Management')
    const { data: session } = useSession()

    const { data } = useSuspenseQuery(countryQueries.list(searchTerm, page, session?.accessToken))
    const updateCountryStatus = useUpdateCountryStatus()

    // Runtime validation as failsafe
    if (!data || typeof data !== 'object') {
        throw new Error('Invalid data received from API')
    }
    
    if (!Array.isArray(data.items)) {
        throw new Error('API data.items is not an array')
    }

    return (
        <>
            {/* Table */}
            <div className="overflow-x-auto">
                <table className="min-w-full divide-y divide-gray-200">
                    <thead className="bg-gray-50">
                        <tr>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                {t('countries.table.country')}
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                {t('countries.table.iocCode')}
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                {t('countries.table.isoCode')}
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                {t('countries.table.iihfMember')}
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                {t('countries.table.enabled')}
                            </th>
                            <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                Actions
                            </th>
                        </tr>
                    </thead>
                    <tbody className="bg-white divide-y divide-gray-200">
                        {data.items.map((country: Country) => (
                            <tr key={country.id} className="hover:bg-gray-50">
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <div className="flex items-center">
                                        <Image 
                                            width={32} 
                                            height={25} 
                                            src={getCountryFlag(country.iso2_code ?? "UNKNOWN", country.is_historical ?? false)} 
                                            alt={country.iso2_code ?? "Unknown"} 
                                            className='mr-2 shadow-sm shadow-black' 
                                        />
                                        <div className="text-sm font-medium text-gray-900">
                                            {country.name}
                                        </div>
                                    </div>
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <Badge>{country.ioc_code ?? 'N/A'}</Badge>
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <Badge>{country.iso2_code ?? 'N/A'}</Badge>
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <Badge variant={country.iihf ? 'success' : 'default'}>
                                        {country.iihf ? 'Yes' : 'No'}
                                    </Badge>
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <div className="flex items-center">
                                        <div className={`h-2 w-2 rounded-full mr-2 ${
                                            country.enabled ? 'bg-green-500' : 'bg-gray-400'
                                        }`}></div>
                                        <span className={`text-sm font-medium ${
                                            country.enabled ? 'text-green-700' : 'text-gray-500'
                                        }`}>
                                            {country.enabled ? 'Enabled' : 'Disabled'}
                                        </span>
                                    </div>
                                </td>
                                <td className="px-6 py-4 whitespace-nowrap">
                                    <button
                                        onClick={() => updateCountryStatus.mutate({
                                            countryId: country.id.toString(),
                                            enabled: !country.enabled,
                                            accessToken: session?.accessToken
                                        })}
                                        disabled={updateCountryStatus.isPending}
                                        className={`px-3 py-1 text-xs rounded-md font-medium transition-colors ${
                                            country.enabled
                                                ? 'bg-red-100 text-red-700 hover:bg-red-200 disabled:opacity-50'
                                                : 'bg-green-100 text-green-700 hover:bg-green-200 disabled:opacity-50'
                                        }`}
                                    >
                                        {updateCountryStatus.isPending ? 'Updating...' : (country.enabled ? 'Disable' : 'Enable')}
                                    </button>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>

            {/* Pagination */}
            {data && data.total_pages > 1 && (
                <Pager
                    currentPage={page}
                    totalPages={data.total_pages}
                    onPageChange={onPageChange}
                    pageSize={data.page_size}
                    totalItems={data.total}
                />
            )}
        </>
    )
}

export default function ManagementPage() {
    const t = useTranslations('Management')
    const [searchTerm, setSearchTerm] = useState('')
    const [page, setPage] = useState(0)
    const debouncedSearchTerm = useDebounce(searchTerm, 300)

    // Reset page when search term changes
    useEffect(() => {
        setPage(0)
    }, [debouncedSearchTerm])

    return (
        <div className="space-y-6">
            {/* Header */}
            <div>
                <div className="flex items-center space-x-3 mb-2">
                    <Settings className="h-8 w-8 text-blue-500" />
                    <h1 className="text-3xl font-bold text-gray-900">{t('title')}</h1>
                </div>
                <p className="text-gray-600">{t('description')}</p>
            </div>

            {/* Countries Section */}
            <div className="bg-white rounded-lg shadow-sm border border-gray-200">
                <div className="px-6 py-4 border-b border-gray-200">
                    <div className="flex items-center space-x-3">
                        <Globe className="h-6 w-6 text-blue-500" />
                        <h2 className="text-xl font-semibold text-gray-900">{t('countries.title')}</h2>
                    </div>
                </div>

                {/* Search */}
                <div className="px-6 py-4 border-b border-gray-200">
                    <div className="relative">
                        <Search className="h-5 w-5 absolute left-3 top-3 text-gray-400" />
                        <input
                            type="text"
                            placeholder={t('countries.searchPlaceholder')}
                            className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 text-gray-900 placeholder-gray-400"
                            value={searchTerm}
                            onChange={(e) => setSearchTerm(e.target.value)}
                        />
                    </div>
                </div>

                <QueryErrorBoundary
                    fallback={
                        <div className="p-4 text-center text-red-600 bg-red-50 rounded-md">
                            Failed to load countries. This might be due to a backend API issue.
                            <br />
                            <button 
                                onClick={() => window.location.reload()} 
                                className="mt-2 px-3 py-1 bg-red-600 text-white rounded text-sm hover:bg-red-700"
                            >
                                Reload Page
                            </button>
                        </div>
                    }
                >
                    <Suspense fallback={
                        <TableSkeleton 
                            rows={15} 
                            columns={6} 
                            headers={[
                                t('countries.table.country'),
                                t('countries.table.iocCode'),
                                t('countries.table.isoCode'),
                                t('countries.table.iihfMember'),
                                t('countries.table.enabled'),
                                'Actions'
                            ]}
                        />
                    }>
                        <ErrorBoundary
                            fallback={
                                <div className="p-4 text-center text-red-600 bg-red-50 rounded-md">
                                    Error loading countries table. Please refresh the page.
                                </div>
                            }
                        >
                            <CountriesTable 
                                searchTerm={debouncedSearchTerm}
                                page={page}
                                onPageChange={setPage}
                            />
                        </ErrorBoundary>
                    </Suspense>
                </QueryErrorBoundary>
            </div>
        </div>
    )
}