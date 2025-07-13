"use client"

import { useState } from 'react'
import { Plus, Edit2, Trash2, Globe, Search, Settings } from 'lucide-react'

// Mock data for demonstration - you can replace this with actual API calls
const mockCountries = [
	{ id: 1, name: 'Czech Republic', code: 'CZ', active: true },
	{ id: 2, name: 'Slovakia', code: 'SK', active: true },
	{ id: 3, name: 'Finland', code: 'FI', active: true },
	{ id: 4, name: 'Sweden', code: 'SE', active: true },
	{ id: 5, name: 'Norway', code: 'NO', active: false },
]

export default function Management() {
	const [searchTerm, setSearchTerm] = useState('')
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
	)

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
					<h1 className="text-3xl font-bold text-gray-900">Management</h1>
				</div>
				<p className="text-gray-600">
					Configure and manage system settings, countries, and administrative
					options.
				</p>
			</div>

			{/* Countries Section */}
			<div className="bg-white rounded-lg shadow-sm border border-gray-200">
				<div className="px-6 py-4 border-b border-gray-200">
					<div className="flex items-center justify-between">
						<div className="flex items-center space-x-3">
							<Globe className="h-6 w-6 text-blue-500" />
							<h2 className="text-xl font-semibold text-gray-900">
								Countries
							</h2>
						</div>
						<button
							onClick={() => setShowAddForm(true)}
							className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
						>
							<Plus className="h-4 w-4 mr-2" />
							Add Country
						</button>
					</div>
				</div>

				{/* Search */}
				<div className="px-6 py-4 border-b border-gray-200">
					<div className="relative">
						<Search className="h-5 w-5 absolute left-3 top-3 text-gray-400" />
						<input
							type="text"
							placeholder="Search countries..."
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
									Country Name
								</label>
								<input
									type="text"
									placeholder="e.g., Czech Republic"
									className="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
									value={newCountry.name}
									onChange={(e) =>
										setNewCountry({ ...newCountry, name: e.target.value })
									}
								/>
							</div>
							<div>
								<label className="block text-sm font-medium text-gray-700 mb-1">
									Country Code
								</label>
								<input
									type="text"
									placeholder="e.g., CZ"
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
									Save
								</button>
								<button
									onClick={() => {
										setShowAddForm(false)
										setNewCountry({ name: '', code: '', active: true })
									}}
									className="px-4 py-2 bg-gray-600 text-white rounded-md hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500"
								>
									Cancel
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
									Country
								</th>
								<th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
									Code
								</th>
								<th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
									Status
								</th>
								<th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
									Actions
								</th>
							</tr>
						</thead>
						<tbody className="bg-white divide-y divide-gray-200">
							{filteredCountries.map((country) => (
								<tr key={country.id} className="hover:bg-gray-50">
									<td className="px-6 py-4 whitespace-nowrap">
										<div className="flex items-center">
											<Globe className="h-5 w-5 text-gray-400 mr-3" />
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
											{country.active ? 'Active' : 'Inactive'}
										</button>
									</td>
									<td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
										<div className="flex items-center justify-end space-x-2">
											<button
												className="text-blue-600 hover:text-blue-900 p-1 rounded-md hover:bg-blue-50"
												title="Edit country"
											>
												<Edit2 className="h-4 w-4" />
											</button>
											<button
												onClick={() => handleDeleteCountry(country.id)}
												className="text-red-600 hover:text-red-900 p-1 rounded-md hover:bg-red-50"
												title="Delete country"
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
							No countries found
						</h3>
						<p className="text-gray-500 mb-4">
							{searchTerm
								? 'Try adjusting your search terms.'
								: 'Get started by adding your first country.'}
						</p>
						{!searchTerm && (
							<button
								onClick={() => setShowAddForm(true)}
								className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700"
							>
								<Plus className="h-4 w-4 mr-2" />
								Add Country
							</button>
						)}
					</div>
				)}
			</div>

			{/* Future Management Sections Placeholder */}
			<div className="mt-8 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
				<div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
					<h3 className="text-lg font-semibold text-gray-900 mb-2">
						System Settings
					</h3>
					<p className="text-gray-600 text-sm mb-4">
						Configure global system preferences and options.
					</p>
					<button className="text-blue-600 hover:text-blue-800 text-sm font-medium">
						Coming soon...
					</button>
				</div>

				<div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
					<h3 className="text-lg font-semibold text-gray-900 mb-2">
						User Management
					</h3>
					<p className="text-gray-600 text-sm mb-4">
						Manage user accounts and permissions.
					</p>
					<button className="text-blue-600 hover:text-blue-800 text-sm font-medium">
						Coming soon...
					</button>
				</div>

				<div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
					<h3 className="text-lg font-semibold text-gray-900 mb-2">
						Data Import/Export
					</h3>
					<p className="text-gray-600 text-sm mb-4">
						Import and export hockey data in various formats.
					</p>
					<button className="text-blue-600 hover:text-blue-800 text-sm font-medium">
						Coming soon...
					</button>
				</div>
			</div>
		</div>
	)
}