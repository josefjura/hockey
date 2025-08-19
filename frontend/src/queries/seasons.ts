import { API_URL } from "@/lib/config";
import { Season } from "@/types/season";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';


// Validation function for paginated response
const validatePaginatedResponse = <T>(data: unknown): PaginatedResponse<T> => {
	if (!data || typeof data !== 'object') {
		throw new Error('API response is not an object');
	}
	
	// Check if it's an array (old format) instead of paginated response
	if (Array.isArray(data)) {
		throw new Error('API returned array instead of paginated response - backend may be outdated');
	}
	
	const required = ['items', 'total', 'page', 'page_size', 'total_pages', 'has_next', 'has_previous'];
	for (const field of required) {
		if (!(field in data)) {
			throw new Error(`Missing required field: ${field} in API response`);
		}
	}
	
	if (!Array.isArray((data as Record<string, unknown>).items)) {
		throw new Error('API response items field is not an array');
	}
	
	return data as PaginatedResponse<T>;
};

export const fetchSeasonList = async (page: number = 0, searchTerm?: string, eventId?: string, pageSize: number = 20): Promise<PaginatedResponse<Season>> => {
	const params = new URLSearchParams({
		page: (page + 1).toString(), // Convert from 0-based to 1-based for backend
		page_size: pageSize.toString(),
	});

	if (searchTerm) {
		params.append('year', searchTerm);
	}

	if (eventId) {
		params.append('event_id', eventId);
	}

	const response = await fetch(`${API_URL}/season?${params}`);
	if (!response.ok) {
		throw new Error(`API request failed: ${response.status} ${response.statusText}`);
	}
	
	const data = await response.json();
	return validatePaginatedResponse<Season>(data);
};

export const fetchSeasonListSimple = async (): Promise<Array<{id: number, name: string, year: number, event_name: string}>> => {
	const response = await fetch(`${API_URL}/season/list`);
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

export const createSeason = async (seasonData: { year: number; display_name: string | null; event_id: string }): Promise<{ id: number }> => {
	const response = await fetch(`${API_URL}/season`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({
			year: seasonData.year,
			display_name: seasonData.display_name || null,
			event_id: parseInt(seasonData.event_id),
		}),
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

export const updateSeason = async (id: string, seasonData: { year: number; display_name: string | null; event_id: string }): Promise<void> => {
	const response = await fetch(`${API_URL}/season/${id}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({
			year: seasonData.year,
			display_name: seasonData.display_name || null,
			event_id: parseInt(seasonData.event_id),
		}),
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
};

export const deleteSeason = async (id: string): Promise<void> => {
	const response = await fetch(`${API_URL}/season/${id}`, {
		method: 'DELETE',
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
};

// Query configurations
export const seasonQueries = {
	list: (searchTerm: string = '', eventId: string = '', page: number = 0, pageSize: number = 20) =>
		queryOptions({
			queryKey: ['seasons', searchTerm, eventId, page, pageSize],
			queryFn: () => fetchSeasonList(page, searchTerm || undefined, eventId || undefined, pageSize),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),
	
	all: () =>
		queryOptions({
			queryKey: ['seasons', 'simple'],
			queryFn: () => fetchSeasonListSimple(),
			staleTime: 10 * 60 * 1000, // 10 minutes
		}),
};

// Season mutations
export const useCreateSeason = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: createSeason,
		onSuccess: (data, variables) => {
			// Invalidate seasons queries to refetch data
			queryClient.invalidateQueries({ queryKey: ['seasons'] });
			toast.success(`Season ${variables.year}${variables.display_name ? ` "${variables.display_name}"` : ''} created successfully`);
		},
		onError: (error) => {
			toast.error('Failed to create season. Please try again.');
			console.error('Failed to create season:', error);
		},
	});
};

export const useUpdateSeason = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ id, data }: { id: string; data: { year: number; display_name: string | null; event_id: string } }) => 
			updateSeason(id, data),
		onSuccess: (_, variables) => {
			// Invalidate seasons queries to refetch data
			queryClient.invalidateQueries({ queryKey: ['seasons'] });
			toast.success(`Season ${variables.data.year}${variables.data.display_name ? ` "${variables.data.display_name}"` : ''} updated successfully`);
		},
		onError: (error) => {
			toast.error('Failed to update season. Please try again.');
			console.error('Failed to update season:', error);
		},
	});
};

export const useDeleteSeason = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: deleteSeason,
		onSuccess: () => {
			// Invalidate seasons queries to refetch data
			queryClient.invalidateQueries({ queryKey: ['seasons'] });
			toast.success('Season deleted successfully');
		},
		onError: (error) => {
			toast.error('Failed to delete season. Please try again.');
			console.error('Failed to delete season:', error);
		},
	});
};