import { apiGet, apiPost, apiPut, apiDelete, createClientApiClient } from "@/lib/api-client";
import { Event } from "@/types/event";
import { PaginatedResponse } from "@/types/paging";
import { queryOptions, useMutation, useQueryClient } from '@tanstack/react-query';
import toast from 'react-hot-toast';


export const fetchEventList = async (page: number = 0, searchTerm?: string, pageSize: number = 20, accessToken?: string): Promise<PaginatedResponse<Event>> => {
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
		return client<PaginatedResponse<Event>>(`/event?${params}`);
	}

	// Server-side: Use apiGet (SSR/prefetch)
	return apiGet<PaginatedResponse<Event>>(`/event?${params}`);
};

export const fetchEventListAll = async (accessToken?: string): Promise<Event[]> => {
	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		const data = await client<PaginatedResponse<Event>>('/event?page_size=200');
		return data.items;
	}

	// Server-side: Use apiGet (SSR/prefetch)
	const data = await apiGet<PaginatedResponse<Event>>('/event?page_size=200');
	return data.items;
};

export const createEvent = async (eventData: { name: string; country_id: string | null }, accessToken?: string): Promise<{ id: number }> => {
	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<{ id: number }>('/event', {
			method: 'POST',
			body: JSON.stringify({
				name: eventData.name,
				country_id: eventData.country_id ? parseInt(eventData.country_id) : null,
			}),
		});
	}

	// Server-side: Use apiPost (SSR/server actions)
	return apiPost<{ id: number }>('/event', {
		name: eventData.name,
		country_id: eventData.country_id ? parseInt(eventData.country_id) : null,
	});
};

export const updateEvent = async (id: number, eventData: { name: string; country_id: string | null }, accessToken?: string): Promise<string> => {
	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<string>(`/event/${id}`, {
			method: 'PUT',
			body: JSON.stringify({
				name: eventData.name,
				country_id: eventData.country_id ? parseInt(eventData.country_id) : null,
			}),
		});
	}

	// Server-side: Use apiPut (SSR/server actions)
	return apiPut<string>(`/event/${id}`, {
		name: eventData.name,
		country_id: eventData.country_id ? parseInt(eventData.country_id) : null,
	});
};

export const deleteEvent = async (id: number, accessToken?: string): Promise<string> => {
	// Client-side: Use createClientApiClient with token
	if (accessToken) {
		const client = createClientApiClient(accessToken);
		return client<string>(`/event/${id}`, {
			method: 'DELETE',
		});
	}

	// Server-side: Use apiDelete (SSR/server actions)
	return apiDelete<string>(`/event/${id}`);
};

// Query configurations
export const eventQueries = {
	list: (searchTerm: string = '', page: number = 0, pageSize: number = 20, accessToken?: string) =>
		queryOptions({
			queryKey: ['events', searchTerm, page, pageSize, accessToken],
			queryFn: () => fetchEventList(page, searchTerm || undefined, pageSize, accessToken),
			staleTime: 5 * 60 * 1000, // 5 minutes
		}),
	all: (accessToken?: string) =>
		queryOptions({
			queryKey: ['events-all', accessToken],
			queryFn: () => fetchEventListAll(accessToken),
			staleTime: 10 * 60 * 1000, // 10 minutes
		}),
};

// Event mutations
export const useCreateEvent = () => {
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: ({ data, accessToken }: { data: { name: string; country_id: string | null }; accessToken?: string }) =>
			createEvent(data, accessToken),
		onSuccess: (_, variables) => {
			// Invalidate events queries to refetch data
			queryClient.invalidateQueries({ queryKey: ['events'] });
			toast.success(`Event "${variables.data.name}" created successfully`);
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
		mutationFn: ({ id, data, accessToken }: { id: number; data: { name: string; country_id: string | null }; accessToken?: string }) =>
			updateEvent(id, data, accessToken),
		onSuccess: (_, variables) => {
			queryClient.invalidateQueries({ queryKey: ['events'] });
			toast.success(`Event "${variables.data.name}" updated successfully`);
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
		mutationFn: ({ id, accessToken }: { id: number; accessToken?: string }) =>
			deleteEvent(id, accessToken),
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
