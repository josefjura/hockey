"use client"

import { useState } from 'react'
import { Globe, Search, Settings } from 'lucide-react'
import { useTranslations } from 'next-intl'
import type { Country } from '@/types/country'
import { getCountryFlag } from '@/utils/countryFlag'
import Image from 'next/image'
import Pager from '@/components/pager'
import { useQuery } from '@tanstack/react-query'
import { fetchCountryList } from '@/queries/countries'

const pageSize = 10;

export default function Management() {
	const t = useTranslations('Management')
	const [searchTerm, setSearchTerm] = useState('')
	const [page, setPage] = useState(0)

	// const allFilteredCountries = typedCountries.filter((country) =>
	// 	country.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
	// 	country.code.toLowerCase().includes(searchTerm.toLowerCase())
	// )

	const {data} = useQuery<Country[]>({
		queryKey: ['countries', searchTerm, page],
		queryFn: fetchCountryList,
		select: (data) => data.filter((country: Country) =>
			country.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
			country.ioc_code?.toLowerCase().includes(searchTerm.toLowerCase())
		).slice(page * pageSize, (page + 1) * pageSize)
	})		

	const handlePageChange = (newPage: number) => {
		setPage(newPage)
	}

	if (!data) {
		return <div className="text-center py-12">{t('Loading')}</div>
	}

	return (
		<div className="space-y-6">
			{/* Header */}
			<div>
				<div className="flex items-center space-x-3 mb-2">
					<Settings className="h-8 w-8 text-blue-500" />
					<h1 className="text-3xl font-bold text-gray-900">{t('title')}</h1>
				</div>
				<p className="text-gray-600">
					{t('description')}
				</p>
			</div>

			{/* Countries Section */}
			<div className="bg-white rounded-lg shadow-sm border border-gray-200">
				<div className="px-6 py-4 border-b border-gray-200">
					<div className="flex items-center space-x-3">
						<Globe className="h-6 w-6 text-blue-500" />							
						<h2 className="text-xl font-semibold text-gray-900">
							{t('countries.title')}
						</h2>
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
							onChange={(e) => {
								setSearchTerm(e.target.value)
								setPage(0) // Reset to first page when searching
							}}
						/>
					</div>
				</div>

				{/* Countries Table */}
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
							{data.map((country) => (
								<tr key={country.id} className="hover:bg-gray-50">
									<td className="px-6 py-4 whitespace-nowrap">
										<div className="flex items-center">
											<Image width={32} height={25} src={getCountryFlag(country.iso2_code ?? "UNKNOWN", country.ioc_code ?? "UNKNOWN", country.is_historical ?? false)} alt={country.iso2_code ?? "Unknown"} className='mr-2 shadow-sm shadow-black' />
											<div className="text-sm font-medium text-gray-900">
												{country.name}
											</div>
										</div>
									</td>
									<td className="px-6 py-4 whitespace-nowrap">
										<span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
											{country.ioc_code ?? 'N/A'}
										</span>
									</td>
									<td className="px-6 py-4 whitespace-nowrap">
										<span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
											{country.iso2_code ?? 'N/A'}
										</span>
									</td>
									<td className="px-6 py-4 whitespace-nowrap">
										<span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
											country.iihf 
												? 'bg-green-100 text-green-800'
												: 'bg-gray-100 text-gray-800'
										}`}>
											{country.iihf ? 'Yes' : 'No'}
										</span>
									</td>
									<td className="px-6 py-4 whitespace-nowrap">
										<span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
											{country.enabled ? 'Yes' : 'No'}
										</span>
									</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>				

				{/* Pagination */}
				{data && data.length > 0 && (
					<Pager
						currentPage={page}
						totalPages={Math.ceil(data.length / pageSize)}
						onPageChange={handlePageChange}
						pageSize={pageSize}
						totalItems={data.length}
					/>
				)}
				{/* Empty State */}
				{data.length === 0 && (
					<div className="px-6 py-12 text-center">
						<Globe className="h-12 w-12 text-gray-400 mx-auto mb-4" />
						<h3 className="text-lg font-medium text-gray-900 mb-2">
							{t('countries.empty.title')}
						</h3>
						<p className="text-gray-500 mb-4">
							{searchTerm
								? t('countries.empty.searchMessage')
								: t('countries.empty.emptyMessage')}
						</p>
					</div>
				)}
			</div>

			{/* Future Management Sections Placeholder */}
			<div className="mt-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
				<div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
					<h3 className="text-lg font-semibold text-gray-900 mb-2">
						{t('sections.systemSettings.title')}
					</h3>
					<p className="text-gray-600 text-sm mb-4">
						{t('sections.systemSettings.description')}
					</p>
					<button className="text-blue-600 hover:text-blue-800 text-sm font-medium">
						{t('sections.comingSoon')}
					</button>
				</div>

				<div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
					<h3 className="text-lg font-semibold text-gray-900 mb-2">
						{t('sections.userManagement.title')}
					</h3>
					<p className="text-gray-600 text-sm mb-4">
						{t('sections.userManagement.description')}
					</p>
					<button className="text-blue-600 hover:text-blue-800 text-sm font-medium">
						{t('sections.comingSoon')}
					</button>
				</div>

				<div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
					<h3 className="text-lg font-semibold text-gray-900 mb-2">
						{t('sections.dataImportExport.title')}
					</h3>
					<p className="text-gray-600 text-sm mb-4">
						{t('sections.dataImportExport.description')}
					</p>
					<button className="text-blue-600 hover:text-blue-800 text-sm font-medium">
						{t('sections.comingSoon')}
					</button>
				</div>
			</div>
		</div>
	)
}
