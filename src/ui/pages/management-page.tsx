"use client"

import { useState, useEffect, Suspense } from 'react'
import { Globe, Search, Settings } from 'lucide-react'
import { useTranslations } from 'next-intl'
import type { Country } from '@/types/country'
import { getCountryFlag } from '@/utils/countryFlag'
import Image from 'next/image'
import Pager from '@/components/ui/pager'
import Badge from '@/components/ui/badge'
import TableSkeleton from '@/components/ui/table-skeleton'
import { useSuspenseQuery } from '@tanstack/react-query'
import { fetchCountryList } from '@/queries/countries'
import { useDebounce } from '@/hooks/useDebounce'

function CountriesTable({ searchTerm, page, onPageChange }: { 
    searchTerm: string
    page: number 
    onPageChange: (page: number) => void 
}) {
    const t = useTranslations('Management')
    
    const { data } = useSuspenseQuery({
        queryKey: ['countries', searchTerm, page],
        queryFn: () => fetchCountryList(page, searchTerm || undefined),
        staleTime: 5 * 60 * 1000,
    })

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
                                            src={getCountryFlag(country.iso2_code ?? "UNKNOWN", country.ioc_code ?? "UNKNOWN", country.is_historical ?? false)} 
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
                                    <Badge>{country.enabled ? 'Yes' : 'No'}</Badge>
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
                            className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                            value={searchTerm}
                            onChange={(e) => setSearchTerm(e.target.value)}
                        />
                    </div>
                </div>

                <Suspense fallback={
                    <TableSkeleton 
                        rows={15} 
                        columns={5} 
                        headers={[
                            t('countries.table.country'),
                            t('countries.table.iocCode'),
                            t('countries.table.isoCode'),
                            t('countries.table.iihfMember'),
                            t('countries.table.enabled')
                        ]}
                    />
                }>
                    <CountriesTable 
                        searchTerm={debouncedSearchTerm}
                        page={page}
                        onPageChange={setPage}
                    />
                </Suspense>
            </div>
        </div>
    )
}