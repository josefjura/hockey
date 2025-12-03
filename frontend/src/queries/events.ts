import { apiGet, apiPost, apiPut, apiDelete } from "@/lib/api-client";
import { Event } from "@/types/event";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';


export const fetchEventList = async (page: number = 0, searchTerm?: string, pageSize: number = 20): Promise<PaginatedResponse<Event>> => {
	const params = new URLSearchParams({
		page: (page + 1).toString(), // Convert from 0-based to 1-based for backend
		page_size: pageSize.toString(),
	});

	if (searchTerm) {
		params.append('name', searchTerm);
	}

	return apiGet<PaginatedResponse<Event>>(`/event?${params}`);
};

export const fetchEventListAll = async (): Promise<Event[]> => {
	const data = await apiGet<PaginatedResponse<Event>>('/event?page_size=200');
	return data.items;
};

export const createEvent = async (eventData: { name: string; country_id: string | null }): Promise<{ id: number }> => {
	return apiPost<{ id: number }>('/event', {
		name: eventData.name,
		country_id: eventData.country_id ? parseInt(eventData.country_id) : null,
	});
};

export const updateEvent = async (id: number, eventData: { name: string; country_id: string | null }): Promise<string> => {
	return apiPut<string>(`/event/${id}`, {
		name: eventData.name,
		country_id: eventData.country_id ? parseInt(eventData.country_id) : null,
	});
};

export const deleteEvent = async (id: number): Promise<string> => {
	return apiDelete<string>(`/event/${id}`);
};

// Query configurations
export const eventQueries = {
	list: (searchTerm: string = '', page: number = 0, pageSize: number = 20) =>
		queryOptions({
			queryKey: ['events', searchTerm, page, pageSize],
			queryFn: () => fetchEventList(page, searchTerm || undefined, pageSize),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),
	all: () =>
		queryOptions({
			queryKey: ['events-all'],
			queryFn: () => fetchEventListAll(),
			staleTime: 10 * 60 * 1000, // 10 minutes
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

export const useUpdateEvent = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ id, ...eventData }: { id: number; name: string; country_id: string | null }) => 
			updateEvent(id, eventData),
		onSuccess: (data, variables) => {
			queryClient.invalidateQueries({ queryKey: ['events'] });
			toast.success(`Event "${variables.name}" updated successfully`);
		},
		onError: (error) => {
			toast.error('Failed to update event. Please try again.');
			console.error('Failed to update event:', error);
		},
	});
};

export const useDeleteEvent = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: deleteEvent,
		onSuccess: () => {
			queryClient.invalidateQueries({ queryKey: ['events'] });
			toast.success('Event deleted successfully');
		},
		onError: (error) => {
			toast.error('Failed to delete event. Please try again.');
			console.error('Failed to delete event:', error);
		},
	});
};
