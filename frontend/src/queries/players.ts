import { apiGet, apiPost, apiPut, apiDelete } from "@/lib/api-client";
import { Player } from "@/types/player";
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

export const fetchPlayerList = async (page: number = 0, searchTerm?: string, pageSize: number = 20): Promise<PaginatedResponse<Player>> => {
	const params = new URLSearchParams({
		page: (page + 1).toString(), // Convert from 0-based to 1-based for backend
		page_size: pageSize.toString(),
	});

	if (searchTerm) {
		params.append('name', searchTerm);
	}

	const data = await apiGet<PaginatedResponse<Player>>(`/player?${params}`);
	return validatePaginatedResponse<Player>(data);
};

export const createPlayer = async (playerData: { name: string; country_id: string }): Promise<{ id: number }> => {
	return apiPost<{ id: number }>('/player', {
		name: playerData.name,
		country_id: parseInt(playerData.country_id),
	});
};

export const updatePlayer = async (id: string, playerData: { name: string; country_id: string }): Promise<void> => {
	return apiPut<void>(`/player/${id}`, {
		name: playerData.name,
		country_id: parseInt(playerData.country_id),
	});
};

export const deletePlayer = async (id: string): Promise<void> => {
	return apiDelete<void>(`/player/${id}`);
};

// Query configurations
export const playerQueries = {
	list: (searchTerm: string = '', page: number = 0, pageSize: number = 20) =>
		queryOptions({
			queryKey: ['players', searchTerm, page, pageSize],
			queryFn: () => fetchPlayerList(page, searchTerm || undefined, pageSize),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),
};

// Player mutations
export const useCreatePlayer = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: createPlayer,
		onSuccess: (data, variables) => {
			// Invalidate players queries to refetch data
			queryClient.invalidateQueries({ queryKey: ['players'] });
			toast.success(`Player "${variables.name}" created successfully`);
		},
		onError: (error) => {
			toast.error('Failed to create player. Please try again.');
			console.error('Failed to create player:', error);
		},
	});
};

export const useUpdatePlayer = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ id, data }: { id: string; data: { name: string; country_id: string } }) => 
			updatePlayer(id, data),
		onSuccess: (_, variables) => {
			// Invalidate players queries to refetch data
			queryClient.invalidateQueries({ queryKey: ['players'] });
			toast.success(`Player "${variables.data.name}" updated successfully`);
		},
		onError: (error) => {
			toast.error('Failed to update player. Please try again.');
			console.error('Failed to update player:', error);
		},
	});
};

export const useDeletePlayer = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: deletePlayer,
		onSuccess: () => {
			// Invalidate players queries to refetch data
			queryClient.invalidateQueries({ queryKey: ['players'] });
			toast.success('Player deleted successfully');
		},
		onError: (error) => {
			toast.error('Failed to delete player. Please try again.');
			console.error('Failed to delete player:', error);
		},
	});
};