import { Country } from "@/types/country";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';

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

export const fetchCountryListAll = async (): Promise<Country[]> => {
	const response = await fetch(`${API_URL}/country?page_size=200&enabled=true`); // Get all enabled countries
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	const data: PaginatedResponse<Country> = await response.json();
	return data.items;
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
	all: () =>
		queryOptions({
			queryKey: ['countries-all'],
			queryFn: () => fetchCountryListAll(),
			staleTime: 10 * 60 * 1000, // 10 minutes
		}),
};

// Country mutations
export const useUpdateCountryStatus = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ countryId, enabled }: { countryId: string; enabled: boolean }) =>
			updateCountryStatus(countryId, enabled),
		onMutate: async ({ countryId, enabled }) => {
			// Cancel outgoing refetches so they don't overwrite our optimistic update
			await queryClient.cancelQueries({ queryKey: ['countries'] });

			// Snapshot previous values for rollback
			const previousData = queryClient.getQueriesData({ queryKey: ['countries'] });

			// Optimistically update all country list queries
			queryClient.setQueriesData(
				{ queryKey: ['countries'] },
				(old: PaginatedResponse<Country> | undefined) => {
					if (!old) return old;

					return {
						...old,
						items: old.items.map(country =>
							country.id.toString() === countryId
								? { ...country, enabled }
								: country
						)
					};
				}
			);

			return { previousData };
		},
		onSuccess: (data, variables) => {
			toast.success(`Country ${variables.enabled ? 'enabled' : 'disabled'} successfully`);
		},
		onError: (error, variables, context) => {
			// Rollback on error
			if (context?.previousData) {
				context.previousData.forEach(([queryKey, data]) => {
					queryClient.setQueryData(queryKey, data);
				});
			}
			toast.error('Failed to update country status. Please try again.');
			console.error('Failed to update country status:', error);
		},
		onSettled: (data, error) => {
			// Only refetch if there was an error to ensure consistency
			if (error) {
				queryClient.invalidateQueries({ queryKey: ['countries'] });
			}
		},
	});
};
