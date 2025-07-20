import { Team } from "@/types/team";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions } from '@tanstack/react-query';

const API_URL = process.env.BACKEND_URL || 'http://localhost:8080';

export const fetchTeamList = async (page: number = 0, searchTerm?: string): Promise<PaginatedResponse<Team>> => {
	const params = new URLSearchParams({
		page: (page + 1).toString(), // Convert from 0-based to 1-based for backend
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

// Simple query configuration for team list
export const teamQueries = {
	list: (searchTerm: string = '', page: number = 0) => 
		queryOptions({
			queryKey: ['teams', searchTerm, page],
			queryFn: () => fetchTeamList(page, searchTerm || undefined),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),
};