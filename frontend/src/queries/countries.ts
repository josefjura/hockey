import { apiGet, apiClient, createClientApiClient } from "@/lib/api-client";
import { Country } from "@/types/country";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';


export const fetchCountryList = async (page: number = 0, searchTerm?: string, accessToken?: string): Promise<PaginatedResponse<Country>> => {
	const params = new URLSearchParams({
		page: (page + 1).toString(), // Convert from 0-based to 1-based for backend
	});

	if (searchTerm) {
		params.append('name', searchTerm);
	}

	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<PaginatedResponse<Country>>(`/country?${params}`);
	}

	// Server-side: Use apiGet (SSR/prefetch)
	return apiGet<PaginatedResponse<Country>>(`/country?${params}`);
};

export const fetchCountryListAll = async (accessToken?: string): Promise<Country[]> => {
	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		const data = await client<PaginatedResponse<Country>>('/country?page_size=200&enabled=true');
		return data.items;
	}

	// Server-side: Use apiGet (SSR/prefetch)
	const data = await apiGet<PaginatedResponse<Country>>('/country?page_size=200&enabled=true');
	return data.items;
};

export const updateCountryStatus = async (countryId: string, status: boolean, accessToken?: string) => {
	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client(`/country/${countryId}`, {
			method: 'PATCH',
			body: JSON.stringify(status),
		});
	}

	// Server-side: Use apiClient (SSR/server actions)
	return apiClient(`/country/${countryId}`, {
		method: 'PATCH',
		body: JSON.stringify(status),
	});
}

// Simple query configuration for country list (no details needed)
export const countryQueries = {
	list: (searchTerm: string = '', page: number = 0, accessToken?: string) =>
		queryOptions({
			queryKey: ['countries', searchTerm, page],
			queryFn: () => fetchCountryList(page, searchTerm || undefined, accessToken),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),
	all: (accessToken?: string) =>
		queryOptions({
			queryKey: ['countries-all'],
			queryFn: () => fetchCountryListAll(accessToken),
			staleTime: 10 * 60 * 1000, // 10 minutes
		}),
};

// Country mutations
export const useUpdateCountryStatus = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ countryId, enabled, accessToken }: { countryId: string; enabled: boolean; accessToken?: string }) =>
			updateCountryStatus(countryId, enabled, accessToken),
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
