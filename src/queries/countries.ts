import { Country } from "@/types/country";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions } from '@tanstack/react-query';

const API_URL = process.env.BACKEND_URL || 'http://localhost:8080';

export const fetchCountryList = async (page: number = 0, searchTerm?: string): Promise<PaginatedResponse<Country>> => {
	const params = new URLSearchParams({
		page: (page + 1).toString(), // Convert from 0-based to 1-based for backend
	});

	if (searchTerm) {
		params.append('name', searchTerm);
	}

	const response = await fetch(`${API_URL}/country?${params}`);
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

export const updateCountryStatus = async (countryId: string, status: boolean) => {
	const response = await fetch(`${API_URL}/country/${countryId}`, {
		method: 'PATCH',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify(status),
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
}

// Simple query configuration for country list (no details needed)
export const countryQueries = {
	list: (searchTerm: string = '', page: number = 0) => 
		queryOptions({
			queryKey: ['countries', searchTerm, page],
			queryFn: () => fetchCountryList(page, searchTerm || undefined),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),
};
