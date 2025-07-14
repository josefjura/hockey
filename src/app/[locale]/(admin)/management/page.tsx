"use client"

import { useState } from 'react'
import { Plus, Edit2, Trash2, Globe, Search, Settings } from 'lucide-react'
import { useTranslations } from 'next-intl'
import countries from '@/utils/countries.json'
import { getCountryFlag } from '@/utils/countryFlag'
import Image from 'next/image'

// Mock data for demonstration - you can replace this with actual API calls
const mockCountries = countries.map((country, index) => ({
	id: index + 1,
	name: country.name,
	code: country.code,
	active: true,
}))

const pageSize = 10;

export default function Management() {
	const t = useTranslations('Management')
	const [searchTerm, setSearchTerm] = useState('')
	const [page, setPage] = useState(0)
	const [showAddForm, setShowAddForm] = useState(false)
	const [countries, setCountries] = useState(
		mockCountries.sort((a, b) => a.name.localeCompare(b.name))
	)
	const [newCountry, setNewCountry] = useState({
		name: '',
		code: '',
		active: true,
	})

	const filteredCountries = countries.filter((country) =>
		country.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
		country.code.toLowerCase().includes(searchTerm.toLowerCase())
	).slice(page * pageSize, (page + 1) * pageSize) // Limit to pageSize results per page

	const handleAddCountry = () => {
		if (newCountry.name && newCountry.code) {
			setCountries([
				...countries,
				{
					id: Date.now(),
					...newCountry,
				},
			])
			setNewCountry({ name: '', code: '', active: true })
			setShowAddForm(false)
		}
	}

	const handleDeleteCountry = (id: number) => {
		setCountries(countries.filter((country) => country.id !== id))
	}

	const toggleCountryStatus = (id: number) => {
		setCountries(
			countries.map((country) =>
				country.id === id ? { ...country, active: !country.active } : country
			)
		)
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
					<div className="flex items-center justify-between">
						<div className="flex items-center space-x-3">
							<Globe className="h-6 w-6 text-blue-500" />							
							<h2 className="text-xl font-semibold text-gray-900">
								{t('countries.title')}
							</h2>
						</div>
						<button
							onClick={() => setShowAddForm(true)}
							className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
						>
							<Plus className="h-4 w-4 mr-2" />
							{t('countries.addCountry')}
						</button>
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

				{/* Add Country Form */}
				{showAddForm && (
					<div className="px-6 py-4 border-b border-gray-200 bg-gray-50">
						<div className="grid grid-cols-1 md:grid-cols-3 gap-4">
							<div>
								<label className="block text-sm font-medium text-gray-700 mb-1">
									{t('countries.form.countryName')}
								</label>
								<input
									type="text"
									placeholder={t('countries.form.countryNamePlaceholder')}
									className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
									value={newCountry.name}
									onChange={(e) =>
										setNewCountry({ ...newCountry, name: e.target.value })
									}
								/>
							</div>
							<div>
								<label className="block text-sm font-medium text-gray-700 mb-1">
									{t('countries.form.countryCode')}
								</label>
								<input
									type="text"
									placeholder={t('countries.form.countryCodePlaceholder')}
									className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
									value={newCountry.code}
									onChange={(e) =>
										setNewCountry({
											...newCountry,
											code: e.target.value.toUpperCase(),
										})
									}
									maxLength={3}
								/>
							</div>
							<div className="flex items-end space-x-2">
								<button
									onClick={handleAddCountry}
									className="px-4 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-green-500"
								>
									{t('countries.form.save')}
								</button>
								<button
									onClick={() => {
										setShowAddForm(false)
										setNewCountry({ name: '', code: '', active: true })
									}}
									className="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500"
								>
									{t('countries.form.cancel')}
								</button>
							</div>
						</div>
					</div>
				)}

				{/* Countries Table */}
				<div className="overflow-x-auto">
					<table className="min-w-full divide-y divide-gray-200">
						<thead className="bg-gray-50">
							<tr>
								<th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
									{t('countries.table.country')}
								</th>
								<th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
									{t('countries.table.code')}
								</th>
								<th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
									{t('countries.table.status')}
								</th>
								<th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
									{t('countries.table.actions')}
								</th>
							</tr>
						</thead>
						<tbody className="bg-white divide-y divide-gray-200">
							{filteredCountries.map((country) => (
								<tr key={country.id} className="hover:bg-gray-50">
									<td className="px-6 py-4 whitespace-nowrap">
										<div className="flex items-center">
											<Image width={32} height={25} src={getCountryFlag(country.code)} alt={country.code} className='mr-2 shadow-sm shadow-black' />
											<div className="text-sm font-medium text-gray-900">
												{country.name}
											</div>
										</div>
									</td>
									<td className="px-6 py-4 whitespace-nowrap">
										<span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
											{country.code}
										</span>
									</td>
									<td className="px-6 py-4 whitespace-nowrap">
										<button
											onClick={() => toggleCountryStatus(country.id)}
											className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
												country.active
													? 'bg-green-100 text-green-800'
													: 'bg-red-100 text-red-800'
											}`}
										>
											{country.active ? t('countries.status.active') : t('countries.status.inactive')}
										</button>
									</td>
									<td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
										<div className="flex items-center justify-end space-x-2">
											<button
												className="text-blue-600 hover:text-blue-900 p-1 rounded-md hover:bg-blue-50"
												title={t('countries.actions.edit')}
											>
												<Edit2 className="h-4 w-4" />
											</button>
											<button
												onClick={() => handleDeleteCountry(country.id)}
												className="text-red-600 hover:text-red-900 p-1 rounded-md hover:bg-red-50"
												title={t('countries.actions.delete')}
											>
												<Trash2 className="h-4 w-4" />
											</button>
										</div>
									</td>
								</tr>
							))}
						</tbody>
					</table>
				</div>

				{/* Empty State */}
				{filteredCountries.length === 0 && (
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
						{!searchTerm && (
							<button
								onClick={() => setShowAddForm(true)}
								className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700"
							>
								<Plus className="h-4 w-4 mr-2" />
								{t('countries.addCountry')}
							</button>
						)}
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
