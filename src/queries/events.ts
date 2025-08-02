import { Event } from "@/types/event";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';

const API_URL = process.env.BACKEND_URL || 'http://localhost:8080';

export const fetchEventList = async (page: number = 0, searchTerm?: string, pageSize: number = 20): Promise<PaginatedResponse<Event>> => {
	const params = new URLSearchParams({
		page: (page + 1).toString(), // Convert from 0-based to 1-based for backend
		page_size: pageSize.toString(),
	});

	if (searchTerm) {
		params.append('name', searchTerm);
	}

	const response = await fetch(`${API_URL}/event?${params}`);
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

export const createEvent = async (eventData: { name: string; country_id: string | null }): Promise<{ id: number }> => {
	const response = await fetch(`${API_URL}/event`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json',
		},
		body: JSON.stringify({
			name: eventData.name,
			country_id: eventData.country_id ? parseInt(eventData.country_id) : null,
		}),
	});
	if (!response.ok) {
		throw new Error('Network response was not ok');
	}
	return response.json();
};

// Query configurations
export const eventQueries = {
	list: (searchTerm: string = '', page: number = 0, pageSize: number = 20) =>
		queryOptions({
			queryKey: ['events', searchTerm, page, pageSize],
			queryFn: () => fetchEventList(page, searchTerm || undefined, pageSize),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),
};

// Event mutations
export const useCreateEvent = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: createEvent,
		onSuccess: (data, variables) => {
			// Invalidate events queries to refetch data
			queryClient.invalidateQueries({ queryKey: ['events'] });
			toast.success(`Event "${variables.name}" created successfully`);
		},
		onError: (error) => {
			toast.error('Failed to create event. Please try again.');
			console.error('Failed to create event:', error);
		},
	});
};
