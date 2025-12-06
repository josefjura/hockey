import { apiGet, apiPost, apiPut, apiDelete, createClientApiClient } from "@/lib/api-client";
import { Team } from "@/types/team";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';


export const fetchTeamList = async (page: number = 0, searchTerm?: string, pageSize: number = 20, accessToken?: string): Promise<PaginatedResponse<Team>> => {
	const params = new URLSearchParams({
		page: (page + 1).toString(), // Convert from 0-based to 1-based for backend
		page_size: pageSize.toString(),
	});

	if (searchTerm) {
		params.append('name', searchTerm);
	}

	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<PaginatedResponse<Team>>(`/team?${params}`);
	}

	// Server-side: Use apiGet (SSR/prefetch)
	return apiGet<PaginatedResponse<Team>>(`/team?${params}`);
};

export const fetchTeamListSimple = async (accessToken?: string): Promise<Array<{ id: number, name: string }>> => {
	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<Array<{ id: number, name: string }>>('/team/list');
	}

	// Server-side: Use apiGet (SSR/prefetch)
	return apiGet<Array<{ id: number, name: string }>>('/team/list');
};

export const createTeam = async (teamData: { name: string | null; country_id: string }, accessToken?: string): Promise<{ id: number }> => {
	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<{ id: number }>('/team', {
			method: 'POST',
			body: JSON.stringify({
				name: teamData.name || null,
				country_id: parseInt(teamData.country_id),
			}),
		});
	}

	// Server-side: Use apiPost (SSR/server actions)
	return apiPost<{ id: number }>('/team', {
		name: teamData.name || null,
		country_id: parseInt(teamData.country_id),
	});
};

export const updateTeam = async (id: number, teamData: { name: string | null; country_id: string }, accessToken?: string): Promise<string> => {
	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<string>(`/team/${id}`, {
			method: 'PUT',
			body: JSON.stringify({
				name: teamData.name || null,
				country_id: parseInt(teamData.country_id),
			}),
		});
	}

	// Server-side: Use apiPut (SSR/server actions)
	return apiPut<string>(`/team/${id}`, {
		name: teamData.name || null,
		country_id: parseInt(teamData.country_id),
	});
};

export const deleteTeam = async (id: number, accessToken?: string): Promise<string> => {
	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<string>(`/team/${id}`, {
			method: 'DELETE',
		});
	}

	// Server-side: Use apiDelete (SSR/server actions)
	return apiDelete<string>(`/team/${id}`);
};

// Query configurations
export const teamQueries = {
	list: (searchTerm: string = '', page: number = 0, pageSize: number = 20, accessToken?: string) =>
		queryOptions({
			queryKey: ['teams', searchTerm, page, pageSize, accessToken],
			queryFn: () => fetchTeamList(page, searchTerm || undefined, pageSize, accessToken),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),

	all: (accessToken?: string) =>
		queryOptions({
			queryKey: ['teams', 'simple', accessToken],
			queryFn: () => fetchTeamListSimple(accessToken),
			staleTime: 10 * 60 * 1000, // 10 minutes
		}),
};

// Team mutations
export const useCreateTeam = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ teamData, accessToken }: { teamData: { name: string | null; country_id: string }; accessToken?: string }) =>
			createTeam(teamData, accessToken),
		onSuccess: (data, variables) => {
			// Invalidate teams queries to refetch data
			queryClient.invalidateQueries({ queryKey: ['teams'] });
			toast.success(`Team "${variables.teamData.name || 'National Team'}" created successfully`);
		},
		onError: (error) => {
			toast.error('Failed to create team. Please try again.');
			console.error('Failed to create team:', error);
		},
	});
};

export const useUpdateTeam = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ id, teamData, accessToken }: { id: number; teamData: { name: string | null; country_id: string }; accessToken?: string }) =>
			updateTeam(id, teamData, accessToken),
		onSuccess: (data, variables) => {
			queryClient.invalidateQueries({ queryKey: ['teams'] });
			toast.success(`Team "${variables.teamData.name || 'National Team'}" updated successfully`);
		},
		onError: (error) => {
			toast.error('Failed to update team. Please try again.');
			console.error('Failed to update team:', error);
		},
	});
};

export const useDeleteTeam = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ id, accessToken }: { id: number; accessToken?: string }) =>
			deleteTeam(id, accessToken),
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['teams'] });
			toast.success('Team deleted successfully');
		},
		onError: (error) => {
			toast.error('Failed to delete team. Please try again.');
			console.error('Failed to delete team:', error);
		},
	});
};