import { Team } from "@/types/team";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';

const API_URL = process.env.BACKEND_URL || 'http://localhost:8080';

export const fetchTeamList = async (page: number = 0, searchTerm?: string, pageSize: number = 20): Promise<PaginatedResponse<Team>> => {
	const params = new URLSearchParams({
		page: (page + 1).toString(), // Convert from 0-based to 1-based for backend
		page_size: pageSize.toString(),
	});

	if (searchTerm) {
		params.append('name', searchTerm);
	}

	const response = await fetch(`${API_URL}/team?${params}`);
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

export const fetchTeamListSimple = async (): Promise<Array<{id: number, name: string}>> => {
	const response = await fetch(`${API_URL}/team/list`);
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

export const createTeam = async (teamData: { name: string | null; country_id: string }): Promise<{ id: number }> => {
	const response = await fetch(`${API_URL}/team`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({
			name: teamData.name || null,
			country_id: parseInt(teamData.country_id),
		}),
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

export const updateTeam = async (id: number, teamData: { name: string | null; country_id: string }): Promise<string> => {
	const response = await fetch(`${API_URL}/team/${id}`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({
			name: teamData.name || null,
			country_id: parseInt(teamData.country_id),
		}),
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

export const deleteTeam = async (id: number): Promise<string> => {
	const response = await fetch(`${API_URL}/team/${id}`, {
		method: 'DELETE',
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

// Query configurations
export const teamQueries = {
	list: (searchTerm: string = '', page: number = 0, pageSize: number = 20) =>
		queryOptions({
			queryKey: ['teams', searchTerm, page, pageSize],
			queryFn: () => fetchTeamList(page, searchTerm || undefined, pageSize),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),
	
	all: () =>
		queryOptions({
			queryKey: ['teams', 'simple'],
			queryFn: () => fetchTeamListSimple(),
			staleTime: 10 * 60 * 1000, // 10 minutes
		}),
};

// Team mutations
export const useCreateTeam = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: createTeam,
		onSuccess: (data, variables) => {
			// Invalidate teams queries to refetch data
			queryClient.invalidateQueries({ queryKey: ['teams'] });
			toast.success(`Team "${variables.name || 'National Team'}" created successfully`);
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
		mutationFn: ({ id, ...teamData }: { id: number; name: string | null; country_id: string }) => 
			updateTeam(id, teamData),
		onSuccess: (data, variables) => {
			queryClient.invalidateQueries({ queryKey: ['teams'] });
			toast.success(`Team "${variables.name || 'National Team'}" updated successfully`);
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
		mutationFn: deleteTeam,
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